#[path = "./config.rs"]
pub mod rscp_config;

#[path = "./account.rs"]
pub mod account;

use mysql::*;
use rocket::*;
use mysql::prelude::*;
use handlebars::Handlebars;
use std::collections::HashMap;
use std::path::{PathBuf, Path};

struct Database(Pool);

pub struct Webserver {
    pub conf: rscp_config::Config,
    pub pool: Option<Pool>
}

#[derive(request::FromForm)]
struct UserLoginForm {
    userid: String,
    user_pass: String
}

#[post("/auth", data = "<data>")]
fn auth(data: request::Form<UserLoginForm>, db: State<Database>, mut cookies: http::Cookies) -> response::Flash<response::Redirect> {
    let pool: &Pool = &db.0;    
    let mut conn: PooledConn = pool.get_conn().unwrap();
    let mut success_login: bool = false;

    let stmt = conn.prep("SELECT * FROM login WHERE userid = ? AND user_pass = ?").unwrap();
    conn.exec_map(stmt, (data.userid.as_str(), data.user_pass.as_str()), |row: account::UserAccount| {
        if row.user_pass != data.user_pass {
            success_login = false;
        } else {
            success_login = true;
            cookies.add_private(http::Cookie::new("account", row.userid));
        }
    }).unwrap();

    if success_login {
        response::Flash::success(response::Redirect::to("/login"), format!("Successfully logged in as {}", data.userid))
    } else {
        response::Flash::error(response::Redirect::to("/login"), format!("Failed to login as {}", data.userid))
    }
}

fn get_route(url: &str, flash: Option<request::FlashMessage>, mut cookies: http::Cookies) -> Option<response::content::Html<String>> {
    let mut hbs = Handlebars::new();
    let mut data: HashMap<String, String> = HashMap::new();
    let route: String = format!("./public/{}.hbs", url);
    let userid = cookies.get_private("account");

    std::mem::drop(cookies);

    match flash {
        Some(val) => {
            data.insert("flash_msg".to_string(), val.msg().to_string());
        },
        _ => ()
    }

    match userid {
        Some(val) => {
            data.insert("account".to_string(), val.value().to_string());
        }
        _ => ()
    }

    if Path::new(route.as_str()).exists() {
        hbs.register_template_file(url, route.as_str()).unwrap();
        Some(response::content::Html(hbs.render(url, &data).unwrap()))
    } else { None }
}

#[get("/")]
fn index(flash: Option<request::FlashMessage>, cookies: http::Cookies) -> Option<response::content::Html<String>> {
    get_route("index", flash, cookies)
}

#[get("/<path..>")]
fn rscp_path(path: PathBuf, flash: Option<request::FlashMessage>, cookies: http::Cookies) -> Option<response::content::Html<String>> {
    let url: String = format!("routes/{}", path.to_string_lossy());
    get_route(url.as_str(), flash, cookies)
}

impl Webserver {
    pub fn start(mut self) {
        let url: String = format!("mysql://{}:{}@{}:{}/{}", self.conf.user, self.conf.password, self.conf.host, self.conf.port, self.conf.database);
        self.pool = Some(Pool::new(url).unwrap());
        
        rocket::ignite()
            .mount("/", routes![index, rscp_path, auth])
            .attach(fairing::AdHoc::on_attach("Database", |rocket| {
                Ok(rocket.manage(Database(self.pool.unwrap())))
            })).launch();
    }
}
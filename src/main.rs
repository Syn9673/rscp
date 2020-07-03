#![feature(proc_macro_hygiene, decl_macro)]
mod handler;

fn main() {
    let handler: handler::Webserver = handler::Webserver {
        conf: handler::rscp_config::Config {
            user: "root",
            password: "",
            database: "test_aj",
            host: "localhost",
            port: 3306
        },
        pool: None
    };

    handler.start();
}
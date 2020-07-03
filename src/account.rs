use serde::{Serialize, Deserialize};
use mysql::*;
use mysql::prelude::*;
use std::result::Result;
use serde_json::*;
use consts::ColumnType::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAccount {
    pub account_id: u64,
    pub userid: String,
    pub user_pass: String,
    pub sex: String,
    pub email: String,
    pub group_id: u8,
    pub state: u16,
    pub unban_time: u32,
    pub expiration_time: u32,
    pub logincount: u32,
    pub lastlogin: String,
    pub last_ip: String,
    pub birthdate: String,
    pub character_slots: u8,
    pub pincode: String,
    pub pincode_change: u16,
    pub vip_time: u32,
    pub old_group: u8,
    pub web_auth_token: String,
    pub web_auth_token_enabled: u8
}

impl FromRow for UserAccount {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {   
        let mut json = json!({});

        // convert all values to json
        for column in row.columns_ref() {
            match column.column_type() {
                MYSQL_TYPE_TINY | MYSQL_TYPE_LONG | MYSQL_TYPE_INT24 | MYSQL_TYPE_DECIMAL | MYSQL_TYPE_BIT  => {
                    let value = row.get_opt::<u64, &str>(column.name_str().as_ref()).unwrap();
                    
                    match value {
                        Ok(v) => json[column.name_str().as_ref()] = serde_json::to_value(v).unwrap(),
                        Err(_) => json[column.name_str().as_ref()] = serde_json::to_value(0).unwrap()
                    }
                },

                MYSQL_TYPE_VARCHAR | MYSQL_TYPE_DATE | MYSQL_TYPE_DATETIME | MYSQL_TYPE_VAR_STRING | MYSQL_TYPE_TINY_BLOB | MYSQL_TYPE_BLOB | MYSQL_TYPE_MEDIUM_BLOB | MYSQL_TYPE_LONG_BLOB | MYSQL_TYPE_STRING | MYSQL_TYPE_SET | MYSQL_TYPE_ENUM  => {
                    let value = row.get_opt::<String, &str>(column.name_str().as_ref()).unwrap();
                    
                    match value {
                        Ok(v) => json[column.name_str().as_ref()] = serde_json::to_value(v.as_str()).unwrap(),
                        Err(_) => json[column.name_str().as_ref()] = serde_json::to_value(String::new()).unwrap()
                    }
                },

                MYSQL_TYPE_NULL => json[column.name_str().as_ref()] = json!(null),

                _ => println!("Unhandled column: {} {:?}", column.name_str().as_ref(), column.column_type())
            }
        };

        let formatted: String = format!(r#"{}"#, json.to_string());
        Ok(serde_json::from_str(formatted.as_str()).unwrap())
    }
}
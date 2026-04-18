use std::path::Path;

use pharaohdb::{self, DbErrors, PharaohDatabase, TableBuilder};
use qight::gen_keypair;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::get_key;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String,
    key: String,
}

pub fn setup_db(user_name: String) -> Result<(), DbErrors> {
    let db_name = "qight_chat_user_db";
    let secret_key = "ouythbliojuy78t7rv";

    let mut db = PharaohDatabase::create_db(db_name.to_string(), secret_key)?;

    let users_table = TableBuilder::new("users")
        .add_string_field("name", false)
        .add_string_field("key", true)
        .build();

    if let Err(e) = db.create_table(users_table) {
        if !matches!(e, DbErrors::Tablealreadyexists) {
            return Err(e);
        }
    }

    let (sender_pub, _) = gen_keypair();

    let sender_key = get_key(sender_pub);

    let my_user = User {
        name: user_name,
        key: sender_key,
    };

    let user_id = db.insert("users", serde_json::to_value(my_user).unwrap())?;
    println!("Created a new user with ID: {}", user_id);
    Ok(())
}

pub fn check_user(user_name: String) -> (bool, String, String) {
    if !Path::new("qight_chat_user_db").is_dir() {
        println!("No database dectected, creating db");
        setup_db(user_name.clone()).unwrap();
        println!("database created!")
    }

    let secret_key = "ouythbliojuy78t7rv";

    let db = PharaohDatabase::create_db("qight_chat_user_db".to_string(), secret_key).unwrap();

    let existing = db.find_where("users", "name", &json!(user_name));

    if existing.is_empty() {
        return (false, "".to_string(), "".to_string());
    } else {
        let user: User =
            serde_json::from_value(existing[0].clone()).expect("Failed to deserialize");
        return (true, user.name, user.key);
    }
}

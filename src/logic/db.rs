use pharaohdb::{self, DbErrors, PharaohDatabase, TableBuilder};
use qight::{gen_key, gen_keypair};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String,
    key: [u8; 32],
}

fn setup_db() -> Result<(), DbErrors> {
    let db_name = "qight_chat_user_db";
    let secret_key = "ouythbliojuy78t7rv";

    let mut db = PharaohDatabase::create_db(db_name.to_string(), secret_key)?;

    let users_table = TableBuilder::new("users")
        .add_string_field("name", false)
        .add_boolean_field("key", true)
        .build();

    if let Err(e) = db.create_table(users_table) {
        if !matches!(e, DbErrors::Tablealreadyexists) {
            return Err(e);
        }
    }

    let (sender_pub, _) = gen_keypair();

    let my_user = User {
        name: "Idorocodes".into(),
        key: sender_pub,
    };

    let user_id = db.insert("users", serde_json::to_value(my_user).unwrap())?;
    println!("Created a new user with ID: {}", user_id);
    Ok(())
}


fn check_user() ->bool{
    
    
}
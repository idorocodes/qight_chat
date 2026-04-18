
use crossterm::style::{Color, Stylize};
use std::{
    error::Error,
    fs,
    io::{stdin, stdout, Write},
};

pub mod logic;
pub use logic::*;
pub mod ui;
pub use ui::*;
use qight::{MessageEnvelope, RelayClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    qight_intro();

    let mut username = String::new();
    print!("Enter preferred username: ");
    stdout().flush()?;
    stdin().read_line(&mut username)?;
    let mut username = username.trim().to_string();

    let mut global_client: Option<RelayClient> = None;
    let mut reciever_key_str = String::new();

    loop {
        print!("{}", "qightchat> ".with(Color::Green).bold());
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;
        let command = input.trim();

        if command.is_empty() {
            continue;
        }

        match command {
            "/help" => {
                qight_intro();
            }
            "/quit" => {
                println!("Closing qight chat!");
                break; 
            }
            "/reset" => {
                let _ = fs::remove_dir_all("qight_chat_user_db");
                print!("Enter preferred username: ");
                stdout().flush()?;
                username.clear();
                stdin().read_line(&mut username)?;
                username = username.trim().to_string();

                print!("Enter recipient: ");
                stdout().flush()?;
                reciever_key_str.clear();
                stdin().read_line(&mut reciever_key_str)?;
                reciever_key_str = reciever_key_str.trim().to_string();

                global_client = None;
                println!("Fresh start!");
            }
            "/connect" => {
                let client = set_up_connection().await?;
                client.hello("client-123").await?;
                
                let listener_client = client.clone();
                let user_details = check_user(username.clone());
                let my_pubkey_hex = user_details.2.clone();

                tokio::spawn(async move {
                    loop {
                        if let Ok(messages) = listener_client.fetch(&my_pubkey_hex).await {
                            for msg in messages {
                                let text = String::from_utf8_lossy(&msg.payload);
                                println!("\n[{}]: {}", msg.sender, text);
                                print!("{}", "qightchat> ".with(Color::Green).bold());
                                let _ = stdout().flush();
                            }
                        }
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    }
                });

                global_client = Some(client);
                println!("Connected!");
            }
            c if c.starts_with("/receiver") => {
                let parts: Vec<&str> = c.split_whitespace().collect();
                if let Some(key) = parts.get(1) {
                    reciever_key_str = key.to_string();
                    println!("Recipient set");
                } else {
                    println!("Usage: /receiver <public_key>");
                }
            }
            c if c.starts_with("/send") => {
                if let Some(ref client) = global_client {
                    let message_content = c.strip_prefix("/send ").unwrap_or("").trim();
                    if message_content.is_empty() {
                        println!("Error: Cannot send empty message");
                        continue;
                    }

                    let user_details = check_user(username.clone());

                 
                    let sender_key_vec = match hex::decode(&user_details.2) {
                        Ok(v) => v,
                        Err(_) => { println!("Error: Your public key is not valid hex."); continue; }
                    };
                    let sender_priv_vec = match hex::decode(&user_details.3) {
                        Ok(v) => v,
                        Err(_) => { println!("Error: Your private key is not valid hex."); continue; }
                    };
                    let reciever_key_vec = match hex::decode(&reciever_key_str) {
                        Ok(v) => v,
                        Err(_) => { println!("Error: Recipient key is missing or invalid. Use /receiver first."); continue; }
                    };

                    let sender_key: [u8; 32] = sender_key_vec.try_into().unwrap_or([0; 32]);
                    let sender_privkey: [u8; 32] = sender_priv_vec.try_into().unwrap_or([0; 32]);
                    let reciever_key: [u8; 32] = reciever_key_vec.try_into().unwrap_or([0; 32]);

                    let payload = message_content.as_bytes().to_vec();
                    let mut message = MessageEnvelope::new(
                        username.clone(),
                        reciever_key,
                        sender_key,
                        payload,
                        6000,
                    );
                    message.sign(&sender_privkey);

                    if let Err(e) = client.send(&message).await {
                        println!("Failed to send: {}", e);
                    } else {
                        println!("Message sent!");
                    }
                } else {
                    println!("Error: Not connected. Use /connect first.");
                }
            }
            _ => println!("Unknown command. Type /help for info."),
        }
    } 

    Ok(())
}

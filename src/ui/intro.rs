use crossterm::style::{Color, Stylize};



pub fn qight_intro(){

     println!(
            "{}",
            "╔══════════════════════════════════════════════════════════════╗".with(Color::Cyan)
        );
        println!(
            "{}",
            "║                      QightChat v0.1.0                        ║"
                .with(Color::Green)
                .bold()
        );
        println!(
            "{}",
            "║                 Secure QUIC-based Messaging                  ║".with(Color::Yellow)
        );
        println!(
            "{}",
            "║                                                              ║".with(Color::Cyan)
        );
        println!(
            "{}",
            "║  Commands:                                                   ║".with(Color::White)
        );
        println!(
            "{}",
            "║  /help     - Show commands                                   ║".with(Color::White)
        );
        println!(
            "{}",
            "║  /connect  - Connect to relay                                ║".with(Color::White)
        );
        println!(
            "{}",
            "║  /send     - Send message to user                            ║".with(Color::White)
        );
        println!(
            "{}",
            "║  /receiver    - Set recipient                                ║".with(Color::White)
        );
         println!(
            "{}",
            "║  /reset    - Reset whole session                             ║".with(Color::White)
        );
        println!(
            "{}",
            "║  /quit     - Exit                                            ║".with(Color::White)
        );
        println!(
            "{}",
            "║                                                              ║".with(Color::Cyan)
        );
        println!(
            "{}",
            "║  Type a message or command...                                ║".with(Color::Magenta)
        );
        println!(
            "{}",
            "╚══════════════════════════════════════════════════════════════╝".with(Color::Cyan)
        );
}
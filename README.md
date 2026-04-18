# Qight Chat

A command-line chat application built in Rust, utilizing the  <a href="https://www.github.com/idorocodes/qight"> Qight </a>. library for secure, decentralized messaging over relays. It supports encrypted message sending and receiving with public/private key pairs.

## Features

- **Secure Messaging**: Uses cryptographic keys for signing and verifying messages.
- **Relay-Based Communication**: Connects to a relay server for message exchange.
- **Command-Line Interface**: Simple CLI with commands for setup, connection, and messaging.
- **Asynchronous Operations**: Built with Tokio for non-blocking I/O.
- **User Management**: Handles username and key generation/storage in a <a href="https://www.github.com/idorocodes/pharaohdb"> PharaohDb</a>.

## Prerequisites

- Rust (latest stable version recommended)
- Cargo (comes with Rust)

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/idorocodes/qight_chat
   cd qight_chat
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the application:
   ```
   cargo run
   ```

## Usage

Launch the application and follow the prompts. Use commands to interact with the chat system.

### Commands

- `/help`: Display the introduction and available commands.
- `/quit`: Exit the application.
- `/reset`: Reset the user database, username, and recipient. Prompts for new details.
- `/connect`: Establish a connection to the relay server and start listening for messages.
- `/receiver <public_key>`: Set the recipient's public key (in hex format).
- `/send <message>`: Send an encrypted message to the set recipient.

### Example Workflow

1. Run the app and enter your preferred username.
2. Use `/connect` to connect to the relay.
3. Set the recipient with `/receiver <hex_key>`.
4. Send messages with `/send Hello, world!`.
5. Incoming messages will be displayed automatically.

## Project Structure

- `src/main.rs`: Main entry point with CLI logic.
- `src/logic.rs`: Core logic for user checks and connections.
- `src/ui.rs`: UI-related functions, including intro display.

## Contributing

Contributions are welcome. Please ensure code follows Rust best practices and includes tests.

## License

[MIT]

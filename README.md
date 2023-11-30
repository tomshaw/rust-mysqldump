# Rust MySQL Dump 🚀

![Build](https://github.com/tomshaw/rust-mysqldump/actions/workflows/rust.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/d/rust_mysqldump.svg)](https://crates.io/crates/rust_mysqldump)
[![Crates.io](https://img.shields.io/crates/v/rust_mysqldump.svg)](https://crates.io/crates/rust_mysqldump)

Rust MySQL Dump is a command-line application for quickly backing up your MySQL databases. It supports exporting multiple databases in a single operation, runs asynchronously and is extremely fast. Outputs export information as each operation completes. Tested on moderate to large size databases without any problems.

## Features

- Export all your databases in a single operation.
- Runs asynchronously and is extremely fast.
- Exclude selected databases from the backup.
- Prints exported database list in a colorized table.

## Usage

1. Set the following environment variables:

    - `DB_HOST`: The hostname of your MySQL server.
    - `DB_PORT`: The port number of your MySQL server.
    - `DB_USERNAME`: The username to use when connecting to your MySQL server.
    - `DB_PASSWORD`: The password to use when connecting to your MySQL server.
    - `DB_EXPORTS`: A comma-separated list of databases to backup. Use `*` to backup all databases.
    - `DB_FORGETS`: A comma-separated list of databases to exclude from the backup.

2. Run the application:

    ```bash
    cargo run
    ```

## Dependencies

This utility depends on the following Rust crates:

- `mysql`: To connect to the MySQL server and retrieve the list of databases.
- `cli-table`: To print the list of databases in a neat table.
- `colored`: To colorize the output to the terminal.
- `dotenv`: To load the database configuration from environment variables.
- `serde`: To deserialize the database configuration.

## Contributing

Contributions are welcome! Please submit a pull request or create an issue on GitHub.

## License 

The MIT License (MIT). See [License File](LICENSE) for more information.

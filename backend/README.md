# City bicycle application backend

# Development requirements

* Rust toolchain [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

* SQLx CLI 
  * `cargo install sqlx-cli`
  * Required to update database queries for offline compilation. If you are not going to update database queries, this is not required.

* Running database service (Run `docker compose up -d database` in the parent folder for example.)
  * Database queries are checked during compile time by the `sqlx` crate, which uses the `DATABASE_URL` environment variable (from the parent folder's dotenv `.env` file) to connect to a database.
  
The dotenv file `.env` from the parent directory will also be used while developing this backend application as there is no development-specific environment configuration created for now.

# Compiling and running the application during development
*Using the --release argument in the following commands compiles an optimized, usually much faster, release binary.*

To only build the application, use the command:
```
cargo build [--release]
```

To run the application you can use the following command. This will also rebuild the application if any changes have been made. By default, the backend application will listen to 127.0.0.1:3000.
```
cargo run [--release]
```

# Containerization

If you have updated any database queries, you will have to also update the generated `sqlx-data.json` by running the command
```
cargo sqlx prepare
```

This is required in order to build the backend without database connection inside a Docker container. Running this command is not required if there weren't any database query modifications or additions.

# Tests
You can run automated tests of this application by using the command:
```
cargo test
```

# Additional notes
Intermediate compilation related files will be stored inside a directory called target/. This folder can be safely removed after compile. If the target/ directory is removed, subsequent compiles will be slower.

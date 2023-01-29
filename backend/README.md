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

## The target/ directory

Intermediate compilation related files will be stored inside a directory called target/. This folder can be safely removed after compilation has completed. Note that if the target/ directory is removed, subsequent compiles will be slower as the intermediate files are used as a cache.

## File hierarchy

The `Cargo.toml` and `Cargo.lock` files contain dependency, version and feature information of this application.

**queries/**
* This directory contains database table definitions, creation and dropping queries for specific database systems if required by the used database system. This application currently only supports the `PostgreSQL` database system.

**tests/**
* General test related code. Currently only contains test data, as all tests are included directly in source files at least for now, which is a common Rust pattern.

**src/**

* api/
  * Contains modules related to the HTTP API which this backend application provides. Uses database/*_view modules for now, but if database state changes through the API would be supported, the api/ modules could also use other database modules that are not related to views.
  
* database/
  * Contains modules related to databases (surprise!). This application currently only supports Postgres as its database, but the database is abstracted through a feature flag, leading to possible support of different database systems. Database modules, such as postgres/, contain code that can change the database state.
  * Modules without the *_view extension contain database code that can change the database state. Mostly used by the code in the datasource/ directory, but also used for example in main.rs for database initialization.
  * Modules with the *_view extension contain database code that cannot change the database state. They provide data structure definitions and database support for views that for example the HTTP API can respond with.
  
* datasource/
  * Contains code related to data parsing and database insertion, but doesn't have any database system specific code. There is currently support for importing stations and journeys from csv files. Data sources can change the database state by using the database/ modules without the _view extensions.
  
* model/
  * Contains data structure and type definitions. Only contains direct functionality for the data structure definitions which does not call to code outside this module. These data structure and type definitions are often used in other modules.


* main.rs
  * Imports the most general modules: api.rs, database.rs, datasource.rs, model.rs and unit.rs
  * Contains the main() function, which is the starting point of this application. Initializes the database connection and further passes the database connection variable to the HTTP api after the database startup has been completed. Also contains code to perform database update operations by downloading csv files and using datasource/ modules.

* database.rs
  * Contains very general database related code and has database module imports. Imports database system specific code based on the active feature flags.
  
* datasource.rs
  * Contains very general data source related code and imports datasource/ modules.

* model.rs
  * Only imports modules from the model/ directory for now.

* unit.rs
  * Defines general units used by this application (such as Meters, Seconds or Coordinate).

# Helsinki city bike app

![City bike app image](/readme.png)

# What is this?

This repository contains my solution to the Solita Dev Academy 2023 [pre-assignment](https://github.com/solita/dev-academy-2023-exercise/blob/main/README.md).

# Requirements

## System requirements to build and run the application
* [Docker](https://docs.docker.com/get-docker/)
* [Docker Compose](https://docs.docker.com/compose/install/)

## Development requirements
* Described in the service related subdirectories. They are not required to build and run the application.

## User requirements
* Familiarity with command line.
* Familiarity with environment variables.
* Suggested: Familiarity with Docker and Docker Compose.

# First time setup
Clone or download the repository
```
git clone https://github.com/MRunsten/solita-dev-academy-2023-exercise
cd solita-dev-academy-2023-exercise
```
Copy or rename the included dotfile `.env.example` to `.env`.

Open the `.env` dotfile with your favorite text editor and edit the environment variables for `POSTGRES_USER`, `POSTGRES_PASSWORD` and `DATABASE_URL` to contain a database user account information. You can also change the database name or other environment variables, but only database user account configuration is required. For short testing purposes, simple values such as 'solita' and 'password123' can be used for user account information. 

Verify that the same values that you defined for `POSTGRES_USER` and `POSTGRES_PASSWORD` are also included in the `DATABASE_URL` environment variable in the `.env` dotfile. (*If you also edited the POSTGRES_DB environment variable, check that it is also included in the DATABASE_URL variable.*)

The docker-compose stack's database service creates a user and database based on the `POSTGRES_*` environment values when you start up the database service for the first time. The `DATABASE_URL` environment is used by the backend service when running (and developing) the application.

Additional environment configuration information is provided as comments in the `.env` dotfile.

You are now ready to build and run the application via Docker Compose.

# Building and running
When building and running the application for the first time, it is suggested to run the stack without the -d (detach) command line parameter in order to see status messages. During the first time startup, the `docker compose up` command will also compile the application's services and build the required images. This takes a few minutes, depending on your system and download speed.

**Warning: By default, the following command will expose ports 5432 (database service, postgres) and 3000 (reverse HTTP proxy service, nginx) of your local system!** (For more information, see docker-compose.yml).
```
docker compose up [-d]
```
When building the application, the backend build process may seem to be stuck on `Updating crates.io index`, but this is expected as this is somewhat of a slow step in the backend build process.

I tried a completely fresh build process of the entire service stack on my desktop computer (12-core AMD 5900x, 32GB, NVMe SSD, Gigabit fiber) which took about 2 minutes. I also tried this build process with my laptop (4-core Intel 8350U, 16GB, NVMe SSD, Wi-Fi) which took about 6 minutes.

When starting up the stack, expect some errors and warnings especially from the database container while its startup status is being checked. What you should keep an eye on is the messages from the `backend-container`.

Expected output example (startup took about 1 minute as it had to download ~250mb of data and insert it to the database. This was running on my desktop system as described above):
```
mrunsten-solita-database-container       | 2023-01-29 12:10:57.336 UTC [37] FATAL:  the database system is not yet accepting connections
mrunsten-solita-database-container       | 2023-01-29 12:10:57.336 UTC [37] DETAIL:  Consistent recovery state has not been yet reached.
...
mrunsten-solita-database-container       | 2023-01-29 12:09:01.751 UTC [1] LOG:  database system is ready to accept connections
...
mrunsten-solita-backend-container        | 2023-01-29T12:09:02.084356Z  INFO backend: Running in PRODUCTION mode
mrunsten-solita-backend-container        | 2023-01-29T12:09:02.094774Z  INFO sqlx::postgres::notice: relation "cities" already exists, skipping
...
mrunsten-solita-backend-container        | 2023-01-29T12:10:58.628507Z  INFO sqlx::postgres::notice: relation "return_date_index" already exists, skipping
mrunsten-solita-backend-container        | 2023-01-29T12:11:01.722800Z  INFO backend: Emptying and reloading database
mrunsten-solita-backend-container        | 2023-01-29T12:11:01.862422Z  INFO backend: Updating stations
mrunsten-solita-backend-container        | 2023-01-29T12:11:01.862443Z  INFO backend: Downloading https://opendata.arcgis.com/datasets/726277c507ef4914b0aec3cbcfcbfafc_0.csv (May take a while depending on file size and internet speed).
mrunsten-solita-backend-container        | 2023-01-29T12:11:13.778890Z  INFO backend: Downloading completed.
mrunsten-solita-backend-container        | 2023-01-29T12:11:13.778913Z  INFO backend: Updating stations database
mrunsten-solita-backend-container        | 2023-01-29T12:11:13.791896Z  INFO backend: Added 457 stations to the database
mrunsten-solita-backend-container        | 2023-01-29T12:11:13.791911Z  INFO backend: Updating journeys
mrunsten-solita-backend-container        | 2023-01-29T12:11:13.791915Z  INFO backend: Downloading https://dev.hsl.fi/citybikes/od-trips-2021/2021-05.csv (May take a while depending on file size and internet speed).
mrunsten-solita-backend-container        | 2023-01-29T12:11:19.887087Z  INFO backend: Downloading completed.
mrunsten-solita-backend-container        | 2023-01-29T12:11:19.887112Z  INFO backend: Updating journeys database
mrunsten-solita-backend-container        | 2023-01-29T12:11:27.052322Z  INFO backend: Parsed 785206 rows, but added 392603 new unique journeys to the database (skipped 392603 rows).
mrunsten-solita-backend-container        | 2023-01-29T12:11:27.052529Z  INFO backend: Downloading https://dev.hsl.fi/citybikes/od-trips-2021/2021-06.csv (May take a while depending on file size and internet speed).
mrunsten-solita-backend-container        | 2023-01-29T12:11:31.294962Z  INFO backend: Downloading completed.
mrunsten-solita-backend-container        | 2023-01-29T12:11:31.294984Z  INFO backend: Updating journeys database
mrunsten-solita-backend-container        | 2023-01-29T12:11:42.487238Z  INFO backend: Parsed 1177874 rows, but added 588937 new unique journeys to the database (skipped 588937 rows).
mrunsten-solita-backend-container        | 2023-01-29T12:11:42.487523Z  INFO backend: Downloading https://dev.hsl.fi/citybikes/od-trips-2021/2021-07.csv (May take a while depending on file size and internet speed).
mrunsten-solita-backend-container        | 2023-01-29T12:11:46.338789Z  INFO backend: Downloading completed.
mrunsten-solita-backend-container        | 2023-01-29T12:11:46.338813Z  INFO backend: Updating journeys database
mrunsten-solita-backend-container        | 2023-01-29T12:11:57.266310Z  INFO backend: Parsed 1163184 rows, but added 581592 new unique journeys to the database (skipped 581592 rows).
mrunsten-solita-backend-container        | 2023-01-29T12:11:57.266479Z  INFO backend: Refreshing materialized views
...
mrunsten-solita-backend-container        | 2023-01-29T12:12:01.236494Z  INFO backend: Database reloaded
mrunsten-solita-backend-container        | 2023-01-29T12:12:01.236612Z  INFO backend::api: Bicycle application backend listening on [::]:3000
```

While you can access the frontend service even before the backend startup has completed, expect errors, as the backend HTTP Api is not running before the message `INFO backend::api: Bicycle application backend listening on [::]:3000`. After this message the application is ready, and it should be accessible (with default configuration) at [http://localhost:3000](http://localhost:3000)

If you ran the `docker compose` command without the `-d` (detach) argument, you can shut down the docker-compose stack by pressing for example CTRL+C (or CMD+C, depends on the system). If you press this key combination more than once, Docker will shut down the service containers faster (non-gracefully) (which can actually be suggested as there is currently no signal handling in the backend and frontend containers).

If you ran the `docker compose` command with `-d` (detach) argument, you can shut down the docker-compose stack by running the command `docker compose down`.

## After the first run
When using the default environment value `RELOAD_DATABASE=true`, the backend service will on startup clear (drop) all database data, redownload data based on the `LOAD_*` environment variable values, and insert the freshly downloaded data to the re-created database tables. This means that if the database has been edited in any way, changes will be lost. However, this application does currently not change the database state after the initial data loading operation.

After the first run, it is suggested to change the environment variable value `RELOAD_DATABASE` from `true` to `false` in your `.env` dotfile. This will guide the backend service to not clear the database and redownload data in subsequent service runs. The environment variable `RELOAD_DATABASE` should however be set back to true if you change the environment variables `LOAD_STATIONS_FROM` or `LOAD_JOURNEYS_FROM` to any other values in order to reload (reinitialize) the database based on new data source settings. 

The application stack usually starts in less than 5 seconds if no data has to be downloaded/inserted (`RELOAD_DATABASE=false`)

## Additional commands related to building the application container stack

If you wish to only build this application, you can use the command:
```
docker compose build
```

If you wish to only start a specific service, such as the database service for development purposes, you can use the command:
```
docker compose up [-d] database-service
```

If you wish to only build a single service of this application, you can use the command:
```
docker compose build <service name from docker-compose.yml>
```

If you wish to rebuild the whole stack or a single service, use the --no-cache parameter for the build command:
```
docker compose build --no-cache backend-service
```


# Additional information

## About chosen technologies

* Backend
  * [Rust](https://www.rust-lang.org/) programming language
    * General interest in learning Rust (I still consider myself to be new to Rust).
    * I prefer statically and strongly typed programming languages.
    * Rust provides memory safety, performance and functional programming features, which are all things that I find meaningful.
    * If a Rust application compiles, it often also works as expected (albeit there can still be logical bugs).
* Database
  * [PostgreSQL](https://www.postgresql.org/) RDBMS
    * Well performing relational database that I have also used before.
* Frontend
  * [SvelteKit](https://kit.svelte.dev/)
    * General interest for Svelte as it has been one of the most liked frontend frameworks for a while. SvelteKit was chosen it provided a ready-to-use frontend development and deployment framework for Svelte. I did not have prior experience using Svelte(Kit).
  * [TypeScript](https://www.typescriptlang.org/)
    * I prefer type-safety for everything that is not a single use script for example.
    * I have used TypeScript before.
* Reverse proxy
  * [Nginx](https://www.nginx.com/)
    * Well performing HTTP server and reverse proxy that I have also used before.
* Runtime
  * [Docker](https://www.docker.com/)
    * Containerization makes it easier to run applications. At the time being it can be also be considered to be the de-facto way to run for example web-application backends.
  * [Docker compose](https://docs.docker.com/compose/)
    * Provides a simple way to run a multi-container application service stack.
    * While I have also used kubernetes before and some kubernetes distributions can be started with docker compose, kubernetes seemed like an overkill for an application like this. It could have certainly been used or configuration files could have been provided, although using kubernetes would have made the application configuration and startup more complex, especially as I currently don't have ready-made service images uploaded for example to Docker hub.


## Feature support

### Data import

* ✔️ Import data from the CSV files to a database or in-memory storage
  * The application uses a PostgreSQL database.
* ✔️ Validate data before importing
  * The CSV file structures are checked.
  * Only valid stations are allowed for journeys.
  * Duplicate journeys are not imported to the database.
* ✔️ Don't import journeys that lasted for less than ten seconds
* ✔️ Don't import journeys that covered distances shorter than 10 meters

### Journey list view

### Recommended

* ✔️ List journeys
* ✔️ For each journey show departure and return stations, covered distance in kilometers and duration in minutes.
  * Additionally departure and return date are shown.

### Additional

* ✔️ Pagination

### Station list

* ✔️ List all the stations
* ✔️ Pagination

### Single station view

* ✔️ Station name
* ✔️ Station address (including city)
* ✔️ Station operator name
* ✔️ Total number of journeys starting from the station
* ✔️ Total number of journeys ending at the station

### Surprise us with
* ✔️ Running and building the backend in Docker
  * The application is configured to be built via Docker and can be run as a docker-compose stack.
* ✔️ Mobile browser support
  * The frontend has styling also for smaller displays.

### Notable missing features
* TLS (HTTPS) support
  * This application will often be run on local systems, where TLS configuration is often more difficult (and also somewhat unnecessary). This feature could have still been provided as a configurable option for example.

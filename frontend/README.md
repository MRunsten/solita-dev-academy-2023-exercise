# Bicycle application frontend

## Development requirements

* node [https://nodejs.org/en/](https://nodejs.org/en/)
* npm (installed usually with node) (other JavaScript/TypeScript package managers may also work)

After your system has a working node & npm installation, run the command
```
npm i
```
which is short for npm install and will download and install required packages for your system. 


It is suggested to start a backend service before running the frontend development server. The backend service can be run through Docker Compose or directly in development mode. However, the simple solution is to run `docker-compose up` in the parent folder as this will start all required services. Note that this will also start an old, already built version of the frontend, but this doesn't really matter for frontend development: The frontend development server will be running on a different port, but it and your browser can still access the backend api from the Docker Compose stack due to current CORS (Cross-Origin Resource Sharing) configuration on the backend service.

ote that running the backend also requires a database service or server to be running (for example via docker compose).

The frontend development server can be started without any backend running, but this will mostly show errors when viewed in browser.

You can start the frontend development server (`vite`) with
```bash
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Building this application
You can use the following command to create a release-ready version of this application, but the actual frontend service image is built via Docker Compose, as explained in the parent directory of this file.
```
npm build
```

## Tests
Unfortunately the frontend service currently does not have any automatic tests created. However, initial (non-working) configuration for automatic testing does exist for [Playwright](https://playwright.dev/).

# Additional notes

## File hierarchy

Note: Only partial description is currently provided.

* src/
  * lib/
    * Contains components that this frontend application can use in routes
  * routes/
    * File system hierarchy defines the route hierarchy for this frontend framework. For more information, see [https://kit.svelte.dev/docs/routing](https://kit.svelte.dev/docs/routing).
    * Pages and their functionality are defined within the routes/ folder.
  * styles/
    * Contains general styling related code (*.scss) for this frontend application.
  * app.html, app.scss
    * The root components of this application.
    * Currently, scss color parameters are also defined in app.scss.


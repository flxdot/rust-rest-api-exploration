# Swagger UI

This is a copy of the [Swagger UI](https://github.com/swagger-api/swagger-ui/tree/master/dist) project.
It has been copied in order to provide a working version in combination with `axum`. The available [`swagger-ui`](https://crates.io/crates/swagger-ui) package does not work with `axum` (or I'm too stupid to make it work...).

It will be included in the build process and become part of the executable.

It is modified to work in this project. The changes are:
- The `index.html` file is modified to use the correct paths under which the swagger-ui files are served.
- The `swagger-initializer.js` file is modified to use the `openapi.json` file from the server.
- The `swagger-initializer.js` has been adapted to remove the default standalone layout.

# Rust Log Parser API Documentation

## Overview
The Game Log Parser API is designed to parse game log files and provide detailed game statistics. This API is built using Rust with the Axum framework and follows best practices for REST API development.

## Prerequisites
- Rust and Cargo installed on your machine. You can install Rust using [rustup](https://rustup.rs/).
- The log file (`qgames.log`) should be present in the `data` directory.

## Running the Project
To run the project, follow these steps:

1. Clone the repository:
    ```sh
    git clone git@github.com:nekelpatrick/rust_quake_log_parser.git
    cd game-log-parser
    ```

2. Build the project:
    ```sh
    cargo build
    ```

3. Run the server:
    ```sh
    cargo run
    ```

The server will start at `http://localhost:8080`.

## Endpoints

### Health Check
- **URL:** `/api/healthCheck`
- **Method:** GET
- **Description:** Checks if the API service is running.
- **Response:**
    ```json
    {
        "status": "ok",
        "message": "API Services"
    }
    ```

### Get Log Data
- **URL:** `/api/logs`
- **Method:** GET
- **Description:** Parses the game log file and returns the game statistics.
- **Query Parameters:**
  - `debug` (optional, boolean): If set to `true`, the endpoint content will be printed in the console.
- **Response:**
    ```json
    {
        "games": [
            // List of games with their statistics
        ],
        "player_rankings": [
            // List of player rankings
        ],
        "total_deaths_by_means": {
            // Death counts by means
        }
    }
    ```

## Testing
To run the tests, use the following command:

```sh
cargo test
```
This will execute all the test cases defined in the project to ensure the functionality works as expected.

## Debugging
To enable debugging, add the debug=true query parameter to the /api/logs endpoint. This will print the parsed log content to the console:

```sh
curl "http://localhost:8080/api/logs?debug=true"
```
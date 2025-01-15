# Energy Price App

This is a Rust application that fetches energy prices from the European Data API and writes the data to CSV files. The application fetches historical data for the past year and tomorrow's data, and organizes the data into daily CSV files.

## Dependencies

The project uses the following dependencies:

- `reqwest`: For making HTTP requests.
- `serde`: For serializing and deserializing JSON data.
- `serde_json`: For working with JSON data.
- `chrono`: For date and time manipulation.
- `tokio`: For asynchronous programming.

## Setup

1. **Install Rust**: If you haven't installed Rust yet, you can install it using `rustup`:

   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

## Build

```sh
cargo build
```

## Run

```sh
cargo run
```
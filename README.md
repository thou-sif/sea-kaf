# Sea-Kaf Chat Application

Sea-Kaf is a simple chat application written in Rust that uses the `sea-streamer` library to interact with Redpanda, a Kafka-compatible streaming platform. The application leverages `tokio` for asynchronous runtime and `serde`/`serde_json` for data serialization and deserialization.

## Installation

To get started with Sea-Kaf, ensure you have Rust and Cargo installed on your system. If you don't have them installed, you can follow the instructions [here](https://www.rust-lang.org/tools/install).

Clone the repository:

```sh
git clone https://github.com/thou-sif/sea-kaf.git
cd sea-kaf
```

## Running Redpanda with Docker Compose

To run Redpanda using Docker Compose, ensure you have Docker and Docker Compose installed on your system. If you don't have them installed, you can follow the instructions [here](https://docs.docker.com/get-docker/) and [here](https://docs.docker.com/compose/install/).

Run the following command to start Redpanda:

```sh
docker-compose up -d
```

## Creating the Kafka Topic

After starting Redpanda, create the Kafka topic that the chat application will use. Run the following command:

```sh
docker exec -it redpanda-02 rpk topic create simple-chat
```

Build the project:

```sh
cargo build
```

## Usage

To run the chat application, execute the following command:

```sh
cargo run
```

The application will prompt you to enter messages, which will be sent to the Redpanda topic. The consumer will simultaneously listen for and display messages from the same topic.

## Project Structure

The project follows a standard Rust project structure:

```
sea-kaf/
├── Cargo.lock
├── Cargo.toml
└── src/
    ├── consumer.rs
    ├── main.rs
    └── producer.rs
```

- `Cargo.toml`: Configuration file for the Rust project, specifying metadata, dependencies, and build settings.
- `src/consumer.rs`: Contains the implementation of the Kafka consumer.
- `src/main.rs`: Entry point of the application.
- `src/producer.rs`: Contains the implementation of the Kafka producer.

## Dependencies

The project relies on the following main dependencies:

- `sea-streamer`: Provides core functionality related to Kafka and sockets.
- `serde`: Used for serializing and deserializing data.
- `serde_json`: JSON serialization/deserialization library.
- `tokio`: Asynchronous runtime for I/O operations.

You can find the exact versions and features of these dependencies in the `Cargo.toml` file:

```toml
[dependencies]
sea-streamer = { version = "0.5.0", features = ["kafka", "socket", "runtime-tokio"] }
serde = { version = "1.0.202", features = ["derive"] }
serde_json = "1.0.117"
tokio = { version = "1.37.0", features = ["full"] }
```

## License

MIT
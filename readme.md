# Patience

This project demonstrates asynchronous message handling using Tokio, a popular asynchronous runtime for Rust.

## Installation

- Use the cargo package manager to run the binaries.

```bash
cargo build
```

- For Documentation:

```bash
cargo doc --open
```

## Files

### `1_intros.rs`

Demonstrates basic asynchronous programming in Rust with Tokio using asynchronous functions and futures.

```bash
cargo run -q --bin 1_intros
```

### `2_tasks.rs`

Demonstrates asynchronous functions in Rust with Tokio, including spawning and waiting for multiple tasks.

```bash
cargo run -q --bin 2_tasks
```

### `3_block_on.rs`

Demonstrates running asynchronous code with the `futures` crate using the `block_on` function.

```bash
cargo run -q --bin 3_block_on
```

### `4_tokio_block_on.rs`

Demonstrates creating a customized Tokio runtime with specific settings and running an asynchronous function.

```bash
cargo run -q --bin 4_tokio_block_on
```

### `5_tokio_join.rs`

Demonstrates using Tokio's `tokio::join!` macro and `JoinSet` for handling multiple asynchronous tasks.

```bash
cargo run -q --bin 5_tokio_join
```

### `6_tokio_yield.rs`

Demonstrates running two asynchronous functions (`ticker` and `tocker`) concurrently using Tokio's `tokio::join!` macro.

```bash
cargo run -q --bin 6_tokio_yield
```

### `7_tokio_select.rs`

Demonstrates running three asynchronous random sleep tasks concurrently using Tokio's `tokio::select!` macro.

```bash
cargo run -q --bin 7_tokio_select
```

### `8_tokio_sleep.rs`

Demonstrates asynchronous delay with Tokio using `tokio::time::sleep` and compares it with `std::thread::sleep`.

```bash
cargo run -q --bin 8_tokio_sleep
```

### `9_tokio_channel.rs`

Demonstrates asynchronous message sending and receiving using Tokio's `tokio::sync::mpsc` channel.

```bash
cargo run -q --bin 9_tokio_channel
```

### `10_tokio_channels.rs`

Demonstrates asynchronous message sending and receiving with multiple channels using Tokio's `tokio::sync::mpsc` channel.

```bash
cargo run -q --bin 10_tokio_channels
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)

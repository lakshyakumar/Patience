//! # Async Ticker and Tocker
//!
//! This example demonstrates the usage of two asynchronous functions (`ticker` and `tocker`)
//! running concurrently using Tokio's `tokio::join!` macro.

/// Asynchronous function `ticker`.
///
/// This function prints "tick" for 10 iterations with a yield in between each tick.
///
/// # Examples
///
/// ```rust
/// use tokio::task::JoinSet;
///
/// async fn example() {
///     ticker().await;
/// }
/// ```
async fn ticker() {
    for i in 0..10 {
        println!("tick {i}");
        tokio::task::yield_now().await;
    }
}

/// Asynchronous function `tocker`.
///
/// This function prints "tock" for 10 iterations with a yield in between each tock.
///
/// # Examples
///
/// ```rust
/// use tokio::task::JoinSet;
///
/// async fn example() {
///     tocker().await;
/// }
/// ```
async fn tocker() {
    for i in 0..10 {
        println!("tock {i}");
        tokio::task::yield_now().await;
    }
}

/// The main function.
///
/// This function uses `tokio::join!` to run two asynchronous functions (`ticker` and `tocker`)
/// concurrently.
///
/// # Examples
///
/// ```rust
/// #[tokio::main]
/// async fn main() {
///     main().await;
/// }
/// ```
#[tokio::main]
async fn main() {
    let _ = tokio::join!(tokio::spawn(ticker()), tokio::spawn(tocker()),);
}

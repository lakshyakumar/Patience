//! # Tokio Join Example
//!
//! This example demonstrates the usage of Tokio's `tokio::join!` macro and `JoinSet` for handling multiple asynchronous tasks.

use tokio::task::JoinSet;

/// Asynchronous function `double`.
///
/// This function takes an `i32` and returns its double value.
///
/// # Arguments
///
/// * `n` - The input value.
///
/// # Examples
///
/// ```rust
/// use tokio::task::JoinSet;
///
/// async fn example() {
///     let result = double(2).await;
///     println!("Result: {}", result);
/// }
/// ```
async fn double(n: i32) -> i32 {
    n * 4
}

/// Asynchronous function `hello`.
///
/// This function prints a message, uses `tokio::join!` and `JoinSet` to handle multiple asynchronous tasks.
///
/// # Examples
///
/// ```rust
/// use tokio::task::JoinSet;
///
/// async fn example() {
///     hello().await;
/// }
/// ```
async fn hello() {
    println!("Hello from async");

    // Use the tokio::join! macro
    let result = tokio::join!(double(2), double(3));
    println!("{result:?}");

    // You can still use futures join_all
    let futures = vec![double(2), double(3)];
    let result = futures::future::join_all(futures).await;
    println!("{result:?}");

    // Using Tokio JoinSet
    let mut set = JoinSet::new();
    for i in 0..10 {
        set.spawn(double(i));
    }
    while let Some(res) = set.join_next().await {
        println!("{res:?}");
    }
}

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

/// The main function.
///
/// This function uses `tokio::join!` to run two asynchronous functions concurrently.
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
    let _ = tokio::join!(tokio::spawn(hello()), tokio::spawn(ticker()),);
    println!("Finished");
}

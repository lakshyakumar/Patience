//! # Async Delay with Tokio
//!
//! This example demonstrates the usage of asynchronous delay with Tokio using `tokio::time::sleep`.

use std::time::Duration;

/// Asynchronous function `hello_delay`.
///
/// This function prints a message, sleeps for a specified time asynchronously,
/// and then prints another message when the sleep is done.
///
/// # Arguments
///
/// * `task` - The task number.
/// * `time` - The sleep duration in milliseconds.
///
/// # Examples
///
/// ```rust
/// use tokio::time::Duration;
///
/// #[tokio::main]
/// async fn example() {
///     hello_delay(1, 1000).await;
/// }
/// ```
async fn hello_delay(task: u64, time: u64) {
    println!("Task {task} has started");
    // std::thread::sleep(Duration::from_millis(time));
    tokio::time::sleep(Duration::from_millis(time)).await;
    println!("Task {task} is done.");
}

/// The main function.
///
/// This function creates a vector of asynchronous tasks (`hello_delay`),
/// each with a different sleep duration, and then joins them using `futures::future::join_all`.
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
    let mut futures = Vec::new();
    for i in 0..5 {
        futures.push(hello_delay(i, 500 * i));
    }
    futures::future::join_all(futures).await;
}

// Difference between std::thread::sleep and tokio::time::sleep:
// - `std::thread::sleep` is a synchronous blocking operation that pauses the entire thread.
// - `tokio::time::sleep` is an asynchronous operation that only suspends the current task,
//   allowing other tasks to run concurrently.
// Using `tokio::time::sleep` is preferred in asynchronous code to avoid blocking the entire runtime.

//! # Async Random Sleep with Tokio Select
//!
//! This example demonstrates the usage of Tokio's `tokio::select!` macro with asynchronous random sleep tasks.

use rand::Rng;

/// Asynchronous function `sleep_random`.
///
/// This function sleeps for a random number of seconds (between 0 and 5 seconds).
///
/// # Examples
///
/// ```rust
/// use tokio::time::Duration;
///
/// #[tokio::main]
/// async fn example() {
///     sleep_random().await;
/// }
/// ```
async fn sleep_random() {
    let mut rng = rand::thread_rng();
    let secs = rng.gen_range(0..5);
    tokio::time::sleep(tokio::time::Duration::from_secs(secs)).await;
}

/// The main function.
///
/// This function uses `tokio::select!` to run three asynchronous random sleep tasks concurrently
/// and prints a message when each task returns.
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
    for _ in 0..10 {
        tokio::select! {
            _ = sleep_random() => println!("Task 1 Returned"),
            _ = sleep_random() => println!("Task 2 Returned"),
            _ = sleep_random() => println!("Task 3 Returned"),
        }
    }
}

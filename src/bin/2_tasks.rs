//! # Asynchronous Function with Tokio
//!
//! This example demonstrates the usage of asynchronous functions with Tokio, including spawning
//! multiple tasks and waiting for them to complete.

use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    // Create a vector to store task handles
    let mut handles = vec![];

    // Spawn two asynchronous tasks using Tokio
    for i in 0..2 {
        let handle = tokio::spawn(async move {
            // Call the asynchronous function `foo` with the current iteration index
            foo(i).await;
        });
        handles.push(handle);
    }

    // Wait for all spawned tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }
}

/// Asynchronous function `foo` that returns a `usize`.
///
/// This function simulates an asynchronous operation by sleeping for a certain duration
/// based on the input parameter `i`. It then prints a message and returns 0.
///
/// # Arguments
///
/// * `i` - The iteration index.
///
/// # Examples
///
/// ```rust
/// #[tokio::main]
/// async fn main() {
///     foo(0).await;
/// }
/// ```
async fn foo(i: i32) -> usize {
    // Simulate an asynchronous operation by sleeping
    sleep(Duration::from_millis(1000 * (i + 1) as u64)).await;

    // Print a message indicating the iteration
    println!("Foo with iteration {}", i);

    // Return 0
    0
}

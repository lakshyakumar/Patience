//! # Asynchronous Code with Futures
//!
//! This example demonstrates the use of the `futures` crate to run asynchronous code using the `block_on` function.

use futures::executor::block_on;

/// Asynchronous function `do_work`.
///
/// This function prints a message in the asynchronous world.
///
/// # Examples
///
/// ```rust
/// use futures::executor::block_on;
///
/// fn main() {
///     block_on(do_work());
/// }
/// ```
async fn do_work() {
    println!("Hello, async world!");
}

/// The main function.
///
/// This function prints a message in the synchronous world and calls the `do_work` function using `block_on`.
///
/// # Examples
///
/// ```rust
/// fn main() {
///     block_on(do_work());
/// }
/// ```
fn main() {
    println!("Hello, sync world");

    // Call the asynchronous function `do_work` using `block_on`
    block_on(do_work());
}

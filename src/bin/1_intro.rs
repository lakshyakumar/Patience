//! # Future Example
//!
//! This example demonstrates the usage of asynchronous futures in Rust with Tokio.

#![allow(dead_code, unused_variables)]

use std::future::Future;

#[tokio::main]
async fn main() {
    println!("Future intro!");
    let x = foo().await;
    let y = foo_another().await;
}

/// Asynchronous function `foo` that returns a `usize`.
///
/// This function prints "Foo" and returns 0.
///
/// # Examples
///
/// ```rust
/// #[tokio::main]
/// async fn main() {
///     let result = foo().await;
///     println!("Result: {}", result);
/// }
/// ```

async fn foo() -> usize {
    println!("Foo");
    0
}

/// Asynchronous function `foo_another` that returns a `usize` wrapped in a future.
///
/// This function prints "Foo_Another" and awaits the result of `foo` before returning 0.
///
/// # Examples
///
/// ```rust
/// #[tokio::main]
/// async fn main() {
///     let result = foo_another().await;
///     println!("Result: {}", result);
/// }
/// ```

fn foo_another() -> impl Future<Output = usize> {
    async {
        println!("Foo_Another");
        foo().await;
        0
    }
}

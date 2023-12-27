//! # Customized Tokio Runtime Example
//!
//! This example demonstrates how to create a customized Tokio runtime with specific settings.

use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::runtime;

/// Asynchronous function `hello`.
///
/// This function prints a message from the asynchronous world.
///
/// # Examples
///
/// ```rust
/// use std::sync::atomic::{AtomicUsize, Ordering};
/// use tokio::runtime;
///
/// async fn hello() {
///     println!("Hello from async");
/// }
///
/// fn thread_namer() -> String {
///     static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
///     let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
///     format!("my-pool-{id}")
/// }
///
/// fn main() {
///     let rt = runtime::Builder::new_multi_thread()
///         .worker_threads(4)
///         .thread_name_fn(thread_namer)
///         .thread_stack_size(3 * 1024 * 1024)
///         .event_interval(61)
///         .global_queue_interval(61)
///         .max_blocking_threads(512)
///         .max_io_events_per_tick(1024)
///         .enable_all()
///         .build()
///         .unwrap();
///
///     rt.block_on(hello());
/// }
/// ```
async fn hello() {
    println!("Hello from async");
}

/// Generates a custom thread name.
///
/// This function is used to name the threads in the Tokio runtime.
///
/// # Examples
///
/// ```rust
/// use std::sync::atomic::{AtomicUsize, Ordering};
/// use tokio::runtime;
///
/// fn thread_namer() -> String {
///     static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
///     let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
///     format!("my-pool-{id}")
/// }
/// ```
fn thread_namer() -> String {
    static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
    let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
    format!("my-pool-{id}")
}

/// The main function.
///
/// This function creates a customized Tokio runtime with specific settings and runs the `hello` function.
///
/// # Examples
///
/// ```rust
/// use std::sync::atomic::{AtomicUsize, Ordering};
/// use tokio::runtime;
///
/// async fn hello() {
///     println!("Hello from async");
/// }
///
/// fn thread_namer() -> String {
///     static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
///     let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
///     format!("my-pool-{id}")
/// }
///
/// fn main() {
///     let rt = runtime::Builder::new_multi_thread()
///         .worker_threads(4)
///         .thread_name_fn(thread_namer)
///         .thread_stack_size(3 * 1024 * 1024)
///         .event_interval(61)
///         .global_queue_interval(61)
///         .max_blocking_threads(512)
///         .max_io_events_per_tick(1024)
///         .enable_all()
///         .build()
///         .unwrap();
///
///     rt.block_on(hello());
/// }
/// ```
fn main() {
    // Create a customized Tokio runtime
    let rt = runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name_fn(thread_namer)
        .thread_stack_size(3 * 1024 * 1024)
        .event_interval(61)
        .global_queue_interval(61)
        .max_blocking_threads(512)
        .max_io_events_per_tick(1024)
        .enable_all()
        .build()
        .unwrap();

    // Run the `hello` function in the customized runtime
    rt.block_on(hello());
}

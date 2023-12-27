//! # Asynchronous Message Sender and Receiver
//!
//! This example demonstrates the usage of asynchronous message sending and receiving
//! using Tokio's `tokio::sync::mpsc` channel.

/// Enum representing a message type.
#[derive(Debug)]
enum Message {
    Tick,
}

/// Asynchronous function `sender`.
///
/// This function sends `Message::Tick` messages at a regular interval.
///
/// # Arguments
///
/// * `tx` - The sender end of the channel.
///
/// # Examples
///
/// ```rust
/// use tokio::sync::mpsc;
///
/// #[tokio::main]
/// async fn example() {
///     let (tx, rx) = mpsc::channel(100);
///     tokio::spawn(sender(tx));
/// }
/// ```
async fn sender(tx: tokio::sync::mpsc::Sender<Message>) {
    loop {
        tx.send(Message::Tick).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

/// Asynchronous function `receiver`.
///
/// This function listens for incoming messages and prints "Tick" when a `Message::Tick` is received.
///
/// # Arguments
///
/// * `rx` - The receiver end of the channel.
///
/// # Examples
///
/// ```rust
/// use tokio::sync::mpsc;
///
/// #[tokio::main]
/// async fn example() {
///     let (tx, rx) = mpsc::channel(100);
///     tokio::spawn(receiver(rx));
/// }
/// ```
async fn receiver(mut rx: tokio::sync::mpsc::Receiver<Message>) {
    while let Some(message) = rx.recv().await {
        match message {
            Message::Tick => println!("Tick"),
        }
    }
}

/// The main function.
///
/// This function creates a channel, spawns the sender and receiver tasks, and waits for them to complete.
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
    let (tx, rx) = tokio::sync::mpsc::channel::<Message>(100);
    tokio::spawn(sender(tx));
    receiver(rx).await;
}

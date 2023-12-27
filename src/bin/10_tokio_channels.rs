//! # Asynchronous Message Sender and Receiver
//!
//! This example demonstrates the usage of asynchronous message sending and receiving
//! with multiple channels using Tokio's `tokio::sync::mpsc` channel.

/// Enum representing a message type.
#[derive(Debug)]
enum Message {
    Tick(u32),
}

/// Asynchronous function `sender`.
///
/// This function sends `Message::Tick` messages with a specified value at a regular interval.
///
/// # Arguments
///
/// * `tx` - The sender end of the channel.
/// * `n` - The value to be sent in the messages.
///
/// # Examples
///
/// ```rust
/// use tokio::sync::mpsc;
///
/// #[tokio::main]
/// async fn example() {
///     let (tx, rx) = mpsc::channel(100);
///     tokio::spawn(sender(tx, 1));
/// }
/// ```
async fn sender(tx: tokio::sync::mpsc::Sender<Message>, n: u32) {
    loop {
        tx.send(Message::Tick(n)).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

/// Asynchronous function `receiver`.
///
/// This function listens for incoming messages on two different channels
/// and prints the received message values.
///
/// # Arguments
///
/// * `rx` - The receiver end of the first channel.
/// * `rx2` - The receiver end of the second channel.
///
/// # Examples
///
/// ```rust
/// use tokio::sync::mpsc;
///
/// #[tokio::main]
/// async fn example() {
///     let (tx, rx) = mpsc::channel(100);
///     let (tx2, rx2) = mpsc::channel(100);
///     tokio::spawn(receiver(rx, rx2));
/// }
/// ```
async fn receiver(
    mut rx: tokio::sync::mpsc::Receiver<Message>,
    mut rx2: tokio::sync::mpsc::Receiver<Message>,
) {
    loop {
        tokio::select! {
            Some(Message::Tick(n)) = rx.recv() => println!("Received message {n} from channel 1"),
            Some(Message::Tick(n)) = rx2.recv() => println!("Received message {n} from channel 2"),
        }
    }
}

/// The main function.
///
/// This function creates two channels, spawns sender tasks for each channel, and a receiver task
/// that listens on both channels and prints the received message values.
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
    let (tx2, rx2) = tokio::sync::mpsc::channel::<Message>(100);
    tokio::spawn(sender(tx, 1));
    tokio::spawn(sender(tx2, 2));
    receiver(rx, rx2).await;
}

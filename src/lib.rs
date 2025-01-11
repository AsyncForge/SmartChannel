//! # Smart Channel
//! This crate provides enhanced functionalities for working with Tokio's channels in mpsc
//!
//! ## Overview
//! In its current version, this crate introduces:
//! - **Identifiable Channels**: A way to associate `Sender` and `Receiver` with unique IDs to track and bind channels logically.
//!
//! ## Usage Example
//! ```rust
//! #[tokio::main]
//! async fn main() {
//!     use smart_channel::channel;
//!
//!     let id = 1;
//!     let (sender, mut receiver) = channel::<String, _>(100, id);
//!
//!     tokio::spawn(async move {
//!         sender.send("Hello from sender!".to_string()).await.unwrap();
//!     });
//!
//!     let message = receiver.recv().await.unwrap();
//!     assert_eq!(message, "Hello from sender!");
//! }
//! ```
//! ## Features
//! - Bind `Sender` and `Receiver` using an ID for stronger logical coupling.
//! - Provides methods like `is_binded_with` to check relationships between channels.

mod channels;

pub use channels::{bind, channel, Receiver, Sender};

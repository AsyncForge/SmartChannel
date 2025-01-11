use std::ops::{Deref, DerefMut};

use tokio::sync::mpsc::{
    channel as tokio_channel, Receiver as TokioReceiver, Sender as TokioSender,
};

/// Creates a Tokio channel (`tokio::sync::mpsc::channel`) and associates it with the provided ID.
pub fn channel<T, Id: Clone + Eq + PartialEq>(
    channel_size: usize,
    id: Id,
) -> (Sender<T, Id>, Receiver<T, Id>) {
    let (sender, receiver) = tokio_channel(channel_size);
    bind(sender, receiver, id)
}

/// Wraps the given sender and receiver with the provided ID.
/// This function ensures that calling `is_bound_to` on the returned sender with the returned receiver
/// will return `true` and vice-versa.
/// However, it does not guarantee that the Tokio channels are correctly connected.
pub fn bind<T, Id: Clone + Eq + PartialEq>(
    sender: TokioSender<T>,
    receiver: TokioReceiver<T>,
    id: Id,
) -> (Sender<T, Id>, Receiver<T, Id>) {
    (Sender::new(sender, id.clone()), Receiver::new(receiver, id))
}

/// A wrapper around Tokio's sender, with an associated ID for identification.
/// Multiple senders can share the same ID if they are cloned or created using `channel` or `bind` with the same ID.
#[derive(Debug, Clone)]
pub struct Sender<T, Id: Clone + Eq + PartialEq> {
    id: Id,
    sender: TokioSender<T>,
}

impl<T, Id: Clone + Eq + PartialEq> PartialEq for Sender<T, Id> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T, Id: Clone + Eq + PartialEq> Eq for Sender<T, Id> {}

/// A wrapper around Tokio's receiver, with an associated ID for identification.
/// Since receivers are not clonable, each receiver must have its own unique ID unless you manually create another receiver with the same ID.
#[derive(Debug)]
pub struct Receiver<T, Id: Clone + Eq + PartialEq> {
    id: Id,
    receiver: TokioReceiver<T>,
}

impl<T, Id: Clone + Eq + PartialEq> Sender<T, Id> {
    /// Private constructor. To create a `Sender`, use `channel`.
    fn new(sender: TokioSender<T>, id: Id) -> Self {
        Self { id, sender }
    }

    /// Returns a reference to the ID of the `Sender`.
    pub fn id(&self) -> &Id {
        &self.id
    }

    /// Returns `true` if `self` is associated with the given `Receiver`, meaning they share the same ID.
    pub fn is_bound_to(&self, receiver: &Receiver<T, Id>) -> bool {
        self.id == receiver.id
    }
}

impl<T, Id: Clone + Eq + PartialEq> Receiver<T, Id> {
    /// Private constructor. To create a `Receiver`, use `channel`.
    fn new(receiver: TokioReceiver<T>, id: Id) -> Self {
        Self { id, receiver }
    }

    /// Returns a clone of the ID of the `Receiver`.
    pub fn id(&self) -> Id {
        self.id.clone()
    }

    /// Returns `true` if `self` is associated with the given `Sender`, meaning they share the same ID.
    pub fn is_bound_to(&self, sender: &Sender<T, Id>) -> bool {
        self.id == sender.id
    }
}

impl<T, Id: Clone + Eq + PartialEq> Deref for Sender<T, Id> {
    type Target = TokioSender<T>;

    fn deref(&self) -> &Self::Target {
        &self.sender
    }
}

impl<T, Id: Clone + Eq + PartialEq> DerefMut for Sender<T, Id> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sender
    }
}

impl<T, Id: Clone + Eq + PartialEq> Deref for Receiver<T, Id> {
    type Target = TokioReceiver<T>;

    fn deref(&self) -> &Self::Target {
        &self.receiver
    }
}

impl<T, Id: Clone + Eq + PartialEq> DerefMut for Receiver<T, Id> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.receiver
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sender_receiver_binding() {
        let id = 1;
        let (sender, receiver) = channel::<(), _>(10, id.clone());

        assert_eq!(sender.id(), &id);
        assert_eq!(receiver.id(), id);
        assert!(sender.is_bound_to(&receiver));
        assert!(receiver.is_bound_to(&sender));
    }

    #[tokio::test]
    async fn test_send_receive_message() {
        let id = 2;
        let (sender, mut receiver) = channel(10, id);

        sender.send("Hello, world!".to_string()).await.unwrap();
        let received = receiver.recv().await.unwrap();

        assert_eq!(received, "Hello, world!");
    }

    #[tokio::test]
    async fn test_unbound_sender_receiver() {
        let id1 = 1;
        let id2 = 2;

        let (sender, _) = channel::<String, _>(10, id1);
        let (_, receiver) = channel::<String, _>(10, id2);

        assert!(!sender.is_bound_to(&receiver));
        assert!(!receiver.is_bound_to(&sender));
    }

    #[tokio::test]
    async fn test_multiple_senders_with_same_id() {
        let id = 1;
        let (sender1, mut receiver) = channel(10, id.clone());
        let sender2 = sender1.clone();

        sender1.send("From sender1".to_string()).await.unwrap();
        let msg1 = receiver.recv().await.unwrap();
        assert_eq!(msg1, "From sender1");

        sender2.send("From sender2".to_string()).await.unwrap();
        let msg2 = receiver.recv().await.unwrap();
        assert_eq!(msg2, "From sender2");
    }

    #[tokio::test]
    async fn test_receiver_closes_on_drop() {
        let id = 1;
        let (sender, receiver) = channel::<String, _>(10, id);

        drop(receiver);

        let result = sender.send("This should fail".to_string()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_no_duplicate_message_on_same_id() {
        let id = 4;
        let (sender1, mut receiver) = channel(10, id);
        let sender2 = sender1.clone();

        sender1.send("Message 1".to_string()).await.unwrap();
        sender2.send("Message 2".to_string()).await.unwrap();

        let received1 = receiver.recv().await.unwrap();
        let received2 = receiver.recv().await.unwrap();

        assert_ne!(received1, received2);
    }

    #[tokio::test]
    async fn test_cloning_sender_does_not_create_new_id() {
        let id = 42;
        let (sender1, _) = channel::<String, _>(10, id.clone());
        let sender2 = sender1.clone();

        assert_eq!(sender1.id(), sender2.id());
        assert!(sender1 == sender2);
    }
}

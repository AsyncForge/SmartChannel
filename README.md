# SmartChannel

**SmartChannel** is a Rust library built on top of Tokio that enhances the capabilities of the standard Tokio sender/receiver channels by providing additional features such as ID-based sender/receiver pairs.
 
---

## **Features**

- **ID-Linked Channels**: Each sender and receiver is associated with a unique connection ID, enabling verification of sender-receiver bindings.
- **Lightweight Wrappers**: Sender and receiver types wrap Tokio's standard channels, adding extra metadata while keeping the same behavior.
- **Interoperability**: SmartChannel supports direct dereferencing to use all standard Tokio sender/receiver methods.
- **Custom Bindings**: Provides bind and channel functions to associate custom IDs with channels.


---

## **Installation**

Add the following line to your Cargo.toml:
```toml
[dependencies]
smart_channel = "0.1.0"
```

## **Usage**

### Exemple 1: Basic Usage
```rust
use smart_channel::channel;

#[tokio::main]
async fn main() {
    let id = 1;
    let (sender, mut receiver) = channel::<String, _>(100, id);

    tokio::spawn(async move {
        sender.send("Hello from sender!".to_string()).await.unwrap();
    });

    let message = receiver.recv().await.unwrap();
    assert_eq!(message, "Hello from sender!");
}
```
### Example 2: Sender and Receiver ID Matching
```rust
use smart_channel::channel;

#[tokio::main]
async fn main() {
    let id = "channel-1".to_string();
    let (sender, receiver) = channel::<i32, _>(100, id.clone());

    assert!(sender.is_bound_to(&receiver));
    println!("Sender and receiver are bound by ID: {:?}", id.id);
}
```

## **Exemple 3: Multiple Channels with Differents IDs**

```rust
use smart_channel::channel;

#[tokio::main]
async fn main() {
    let id1 = "channel-1".to_string();
    let id2 = "channel-2".to_string();

    let (sender1, receiver1) = channel::<String, _>(100, id1.clone());
    let (sender2, receiver2) = channel::<String, _>(100, id2.clone());

    assert!(sender1.is_bound_to(&receiver1));
    assert!(!sender1.is_bound_to(&receiver2));  // Different IDs

    println!("Channel-1: bound correctly");
}
```

---

## **Contributing**

Contributions are welcome! If you encounter bugs, have feature requests, or want to submit a pull request, feel free to visit the GitHub repository.
Check the issues section for upcoming features, such as clonable receivers and broadcasting channels.

## **License**

This project is licensed under the MIT License. See the LICENSE file for details.

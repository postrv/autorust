// messaging.rs

use serde_json::Value;
use std::collections::VecDeque;

/// Represents a message to be processed by an agent.
pub struct Message {
    pub(crate) sender_id: String,
    recipient_id: String,
    pub(crate) content: Value,  // JSON content
}

/// A queue for storing and managing messages for an agent.
pub struct MessageQueue {
    queue: VecDeque<Message>,
}

impl MessageQueue {
    pub fn new() -> Self {
        MessageQueue {
            queue: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, message: Message) {
        self.queue.push_back(message);
    }

    pub fn dequeue(&mut self) -> Option<Message> {
        self.queue.pop_front()
    }

    // Additional methods can be added as needed
}


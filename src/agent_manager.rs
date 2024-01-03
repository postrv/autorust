// agent_manager.rs

use std::collections::HashMap;
use async_trait::async_trait;

use serde_json::Value;

use crate::agent_registry::{AgentRegistry, AgentInfo};
use crate::messaging::Message;

/// Represents the state and functionality of an agent.
#[async_trait]
pub trait Agent: Send {
    async fn analyze_image(&self, image_data: &[u8]) -> Result<Value, reqwest::Error>;
    fn process_message(&mut self, message: &Message);
    fn send_message(&mut self, recipient_id: &str, content: Value);
    // Additional methods can be added here
    fn enqueue_message(&mut self, message: Message);
}

/// Manages agents and their interactions.
pub struct AgentManager {
    agents: HashMap<String, Box<dyn Agent>>,
    agent_registry: AgentRegistry,
}

impl AgentManager {
    pub fn new() -> Self {
        AgentManager {
            agents: HashMap::new(),
            agent_registry: AgentRegistry::new(),
        }
    }

    pub fn register_agent(&mut self, id: String, agent: Box<dyn Agent>) {
        self.agents.insert(id, agent);
    }

    // Method to integrate with Agent Registry
    pub fn register_agent_with_info(&mut self, agent_info: AgentInfo, agent: Box<dyn Agent>) {
        self.agents.insert(agent_info.id.clone(), agent);
        self.agent_registry.register_agent(agent_info);
    }

    // Method to retrieve agent information
    pub fn get_agent_info(&self, id: &str) -> Option<&AgentInfo> {
        self.agent_registry.get_agent_info(id)
    }
    // New method to enqueue a message to a specific agent's queue
    pub fn enqueue_message(&mut self, recipient_id: String, message: Message) {
        if let Some(agent) = self.agents.get_mut(&recipient_id) {
            agent.enqueue_message(message);
        } else {
            // Handle the case where the recipient agent is not found
            // This could be logging an error or a similar action
        }
    }
}


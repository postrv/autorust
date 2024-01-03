// agent_registry.rs

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Represents the information of an agent.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentInfo {
    pub id: String,
    pub function: String,
    // Add additional fields as needed
}

/// Manages the registration and information of agents.
pub struct AgentRegistry {
    registry: HashMap<String, AgentInfo>,
}

impl AgentRegistry {
    pub fn new() -> Self {
        AgentRegistry {
            registry: HashMap::new(),
        }
    }

    pub fn register_agent(&mut self, agent_info: AgentInfo) {
        self.registry.insert(agent_info.id.clone(), agent_info);
    }

    pub fn get_agent_info(&self, id: &str) -> Option<&AgentInfo> {
        self.registry.get(id)
    }

    // Additional methods for managing the registry can be added here
}

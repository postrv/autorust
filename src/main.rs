mod agent_manager;
mod agent_registry;
mod code_checker_agent;
mod code_interpreter_agent;
mod coding_agent;
mod gpt_vision_agent;
mod messaging;

use crate::agent_manager::AgentManager;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref AGENT_MANAGER: Mutex<AgentManager> = Mutex::new(AgentManager::new());
}

fn main() {
    println!("Hello, world!");
}

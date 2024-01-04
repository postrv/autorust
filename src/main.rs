mod agent_manager;
mod agent_registry;
mod gpt_vision_agent;
mod messaging;
mod coding_agent;
mod code_interpreter_agent;
mod code_checker_agent;



use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::agent_manager::AgentManager;

lazy_static! {
    pub static ref AGENT_MANAGER: Mutex<AgentManager> = Mutex::new(AgentManager::new());
}



fn main() {
    println!("Hello, world!");
}

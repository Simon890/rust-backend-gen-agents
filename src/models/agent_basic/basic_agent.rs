use crate::models::general::llm::Message;

use super::basic_traits::BasicTraits;

#[derive(Debug, PartialEq)]
pub enum AgentState {
    Discovery,
    Working,
    UnitTesting,
    Finished
}

#[derive(Debug)]
pub struct BasicAgent {
    pub objective: String,
    pub position: String,
    pub state: AgentState,
    pub memory: Vec<Message>
}

impl BasicTraits for BasicAgent {
    fn new(objective: String, position: String) -> Self {
        BasicAgent { 
            objective, 
            position, 
            state: AgentState::Discovery, 
            memory: vec!()
        }
    }

    fn update_state(&mut self, new_state: AgentState) {
        self.state = new_state;
    }

    fn get_memory(&self) -> &Vec<Message> {
        &self.memory
    }

    fn get_objective(&self) -> &str {
        &self.objective
    }

    fn get_position(&self) -> &str {
        &self.position
    }

    fn get_state(&self) -> &AgentState {
        &self.state
    }
}
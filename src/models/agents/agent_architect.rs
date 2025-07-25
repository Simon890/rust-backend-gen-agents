use crate::ai_functions::aifunc_arch::{print_project_scope, print_site_urls};
use crate::helpers::command_line::PrintCommand;
use crate::helpers::general::{ai_task_request_decoded, check_status_code};
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agent_basic::basic_traits::BasicTraits;
use crate::models::agents::agent_traits::{FactSheet, ProjectScope, SpecialFunctions};

use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;

// Solutions Architect
#[derive(Debug)]
pub struct AgentSolutionArchitect {
    attributes: BasicAgent
}

impl AgentSolutionArchitect {
    pub fn new() -> Self {
        let attributes = BasicAgent::new(
            "Gathers information and design solutions for website development".to_string(), 
            "Solutions Architect".to_string()
        );
        Self {
            attributes
        }
    }

    //Retrieve project scope
    async fn call_project_scope(&mut self, factsheet: &mut FactSheet) -> ProjectScope {
        let msg_context: String = format!("{}", factsheet.project_description);
        let ai_response: ProjectScope = ai_task_request_decoded::<ProjectScope>(
            msg_context, 
            &self.attributes.position, 
            get_function_string!(print_project_scope), 
            print_project_scope
        ).await;
        factsheet.project_scope = Some(ai_response.clone());
        self.attributes.update_state(AgentState::Finished);
        ai_response
    }
}
use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::apis::call_request::call_gpt;
use crate::models::general::llm::Message;
use crate::helpers::command_line::PrintCommand;
use std::fs;

const CODE_TEMPLATE_PATH: &str = "./template/code_template";
const EXECUTE_MAIN_PATH: &str = "./template/main";
const API_SCHEMA_PATH: &str = "../schemas/api_schema.json";

/// Extends ai functions to encourage specific output
pub fn extend_ai_functions(ai_function: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str = ai_function(func_input);
    
    //Extend the str to encourage only printing the output
    let message: String = format!("FUNCTION: {}
    INSTRUCTION: You are a function printer. You ONLY print the results of functions. 
    Nothing else. No commentary. Here is the input to the function: {}.
    Print out what the function will return.", 
    ai_function_str, func_input);

    Message {
        role: "system".to_string(),
        content: message,
    }
}

pub async fn ai_task_request(msg_context: String, agent_position: &str, agent_operation: &str, function_pass: for<'a> fn(&'a str) -> &'static str) -> String {
    let extended_msg: Message = extend_ai_functions(function_pass, &msg_context);

    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    let llm_res = call_gpt(vec![extended_msg.clone()]).await;

    match llm_res {
        Ok(llm_response) => llm_response,
        Err(_) => call_gpt(vec![extended_msg.clone()]).await.expect("Failed twice to call OpenAI")
    }
}

pub async fn ai_task_request_decoded<T: DeserializeOwned>(msg_context: String, agent_position: &str, agent_operation: &str, function_pass: for<'a> fn(&'a str) -> &'static str) -> T {
    let llm_res = ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;
    let decoded_res: T = serde_json::from_str(llm_res.as_str()).expect("Failed to decode ai response from serde_json");
    decoded_res
}

/// Checks if a request url is valid
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response: reqwest::Response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

/// Get code template
pub fn read_code_template_contents() -> String {
    let path: String = CODE_TEMPLATE_PATH.to_string();
    fs::read_to_string(path).expect("Failed to read code template")
}

/// Save new backend code
pub fn save_backend_code(contents: &str) {
    let path: String = EXECUTE_MAIN_PATH.to_string();
    fs::write(path, contents).expect("Failed to write main.rs file");
}

/// Save JSON API Endpoint Schema
pub fn save_api_endpoints(api_endpoints: &str) {
    let path: String = API_SCHEMA_PATH.to_string();
    fs::write(path, api_endpoints).expect("Failed to write API endpoints to file");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn tests_extending_ai_function() {
        let extended = extend_ai_functions(convert_user_input_to_goal, "input");
        assert_eq!(extended.role, "system".to_string());
    }

    #[tokio::test]
    async fn tests_ai_task_request() {
        let ai_func_param = "Build a webserver for making stock price api requests".to_string();

        let res = ai_task_request(ai_func_param, 
            "Managing Agent", 
            "Defining user requirements", 
            convert_user_input_to_goal
        ).await;

        dbg!(res);
    }
}


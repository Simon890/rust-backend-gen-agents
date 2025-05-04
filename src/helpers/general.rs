use crate::models::general::llm::Message;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn tests_extending_ai_function() {
        let extended = extend_ai_functions(convert_user_input_to_goal, "input");
        assert_eq!(extended.role, "system".to_string());
    }
}
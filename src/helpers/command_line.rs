use std::io::{stdin, stdout};

use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand
};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = stdout();

        let statement_color: Color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red
        };

        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("[Agent] {}: ", agent_pos);
        
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);

        stdout.execute(ResetColor).unwrap();
    }
}

/// Get user request
pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);

    stdout.execute(ResetColor).unwrap();

    let mut user_response = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response");

    user_response.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prints_agent_messages() {
        PrintCommand::AICall
            .print_agent_message("Managing Agent", "Testing testing, processing something...");
        
        PrintCommand::Issue
            .print_agent_message("Managing Agent", "Found a bug on line 15");
        
    }
}

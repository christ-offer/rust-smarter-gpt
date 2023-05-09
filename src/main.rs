mod utils;

use std::error::Error;
use std::io;
use utils::intern::process_question;
use utils::researcher::review_answers;
use utils::professor::final_answer;

struct ChatMessage {
    question: String,
    intern_responses: [String; 3],
    researcher_response: String,
    professor_response: String,
}

struct ChatHistory {
    chat_history: Vec<ChatMessage>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut chat_history = ChatHistory {
        chat_history: Vec::new(),
    };

    loop {
        println!("Please enter your question (type 'exit' to quit):");
        let mut question = String::new();
        io::stdin().read_line(&mut question)?;

        let question = question.trim();
        if question.to_lowercase() == "exit" {
            break;
        }
        // stringify the chat history
        let mut chat_history_string = String::new();
        for message in &chat_history.chat_history {
            chat_history_string.push_str(&format!("User: {}\n", message.question));
            for (i, intern_response) in message.intern_responses.iter().enumerate() {
                chat_history_string.push_str(&format!("Intern {}: {}\n", i + 1, intern_response));
            }
            chat_history_string.push_str(&format!("Researcher: {}\n", message.researcher_response));
            chat_history_string.push_str(&format!("Assistant: {}\n", message.professor_response));
        }

        let intern_responses = process_question(&question, &chat_history_string).await?;


        let researcher_response = review_answers(&question, &intern_responses).await?;

        let professor_response = final_answer(&question, &intern_responses, &researcher_response).await?;

        chat_history.chat_history.push(ChatMessage {
            question: question.to_string(),
            intern_responses,
            researcher_response,
            professor_response: professor_response.clone(),
        });

        println!("\nAssistant: {}", professor_response);
    }

    println!("Chat history:");
    for message in &chat_history.chat_history {
        println!("User: {}", message.question);
        for (i, intern_response) in message.intern_responses.iter().enumerate() {
            println!("Intern {}: {}", i + 1, intern_response);
        }
        println!("Researcher: {}", message.researcher_response);
        println!("Assistant: {}\n", message.professor_response);
    }

    Ok(())
}

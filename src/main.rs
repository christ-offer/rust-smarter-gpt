mod utils;

use std::error::Error;
use std::io;
use utils::intern::process_question;
use utils::researcher::review_answers;
use utils::professor::final_answer;

#[derive(Debug)]
struct ChatMessage {
    sender: String,
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut chat_history: Vec<ChatMessage> = Vec::new();

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
        for message in &chat_history {
            chat_history_string.push_str(&format!("{}: {}\n", message.sender, message.message));
        }
        
        //println!("Chat history:\n{}", &chat_history_string);

        let responses = process_question(&question, &chat_history_string).await?;

        let review = review_answers(&question, &responses).await?;

        let final_answer = final_answer(&question, &responses, &review).await?;

        chat_history.push(ChatMessage {
            sender: "user".to_string(),
            message: question.to_string(),
        });
        chat_history.push(ChatMessage {
            sender: "assistant".to_string(),
            message: final_answer.clone(),
        });

        println!("\n");
    }

    println!("Chat history:");
    for message in &chat_history {
        println!("{}: {}", message.sender, message.message);
    }

    Ok(())
}

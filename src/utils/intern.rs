use std::error::Error;

use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};
use futures::future::join_all;

pub async fn process_question(question: &str, chat_history: &str) -> Result<[String; 3], Box<dyn Error>> {
    println!("Interns are going to work:\n\n");
    let client = Client::new();
    
    // System message should include chat_history
    let system_message = format!(
      "You are the Intern AI.\n\nYour tasks are as follows:\n\n1) Read the question.\n2) Read the chat history if there is one.\n3) Determine the answer.\n4) Print the answer in full.\n\nIf there are unknown variables, you can ask the user for the value of the variables.\n\nChat History:\n\n{}\n\n",
      chat_history,
    );

    let temperatures = [0.3, 0.5, 0.7];

    let requests = temperatures
        .iter()
        .map(|temperature| {
            CreateChatCompletionRequestArgs::default()
                .max_tokens(512u16)
                .model("gpt-3.5-turbo")
                .temperature(*temperature)
                .messages([
                    ChatCompletionRequestMessageArgs::default()
                        .role(Role::System)
                        .content(&system_message)
                        .build()?,
                    ChatCompletionRequestMessageArgs::default()
                        .role(Role::User)
                        .content(question)
                        .build()?,
                ])
                .build()
        })
        .collect::<Result<Vec<_>, _>>()?;

    let tasks = requests
        .into_iter()
        .map(|request| {
            let client = client.clone();
            tokio::spawn(async move {
                client.chat().create(request).await
            })
        })
        .collect::<Vec<_>>();

    let responses = join_all(tasks).await;
    
    let mut answers = [String::new(), String::new(), String::new()];
    for (i, response) in responses.into_iter().enumerate() {
        let response = response??;
        for choice in response.choices {
            answers[i] = choice.message.content;
        }
    }
    
    println!("\nResponses:\n");
    for (i, answer) in answers.iter().enumerate() {
        println!("Intern {} (Temperature: {}):", i + 1, temperatures[i]);
        println!("Content:\n\n {}\n\n", answer);
    }
    Ok(answers)
}

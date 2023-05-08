use std::error::Error;

use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};
use futures::future::join_all;

pub async fn process_question(question: &str) -> Result<[String; 3], Box<dyn Error>> {
    println!("Interns are going to work:\n\n");
    let client = Client::new();

    let temperatures = [0.3, 0.5, 0.7];

    let requests = temperatures
        .iter()
        .map(|temperature| {
            CreateChatCompletionRequestArgs::default()
                .max_tokens(512u16)
                .model("gpt-4")
                .temperature(*temperature)
                .messages([
                    ChatCompletionRequestMessageArgs::default()
                        .role(Role::System)
                        .content("You are Smarter-GPT, an AI Research Agent. Let's work the answer out in a step-by-step way to ensure we are correct.")
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

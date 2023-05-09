use std::error::Error;

use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};
//use futures::future::join_all;

pub async fn final_answer(question: &str, messages: &[String; 3], researcher_answer: &str) -> Result<String, Box<dyn Error>> {
    println!("\n\n\n Professor is assembling the final answer:\n\n");
    let system_message = format!(
        "You are the Resolver AI.\n\nYour tasks are as follows:\n\n1) Determine which of the answer options the researcher thought was the best.\n2) Improve that answer.\n3) Print the improved answer in full.\n\n"
    );

    let user_message = format!(
        "Here's the information you need to complete your tasks:\n\nUser's Question: {}\n\nIntern AI Answers:\n1) {}\n2) {}\n3) {}\n\nResearcher AI: {}\n\nLet's work this out in a step-by-step way to be sure we have the right answer:\n",
        question,
        messages[0],
        messages[1],
        messages[2],
        researcher_answer,
    );
    
    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-4")
        .temperature(0.1)
        .messages([
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content(system_message)
                .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(user_message)
                .build()?,
        ])
        .build()?;

    let response = client.chat().create(request).await?;
    
    println!("\nResponse:\n");
    println!("\n{}\n\n", response.choices[0].message.content);
    
    Ok(response.choices[0].message.content.clone())
}

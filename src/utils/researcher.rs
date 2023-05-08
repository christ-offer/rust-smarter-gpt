use std::error::Error;

use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};
//use futures::future::join_all;

pub async fn review_answers(question: &str, messages: &[String; 3]) -> Result<String, Box<dyn Error>> {
    println!("\n\n\nReviewing Answers:\n\n");
    let system_message = "You are a Researcher tasked with identifying flaws and faulty logic in each answer.
    \n
    Let's work through this process in a step-by-step manner to ensure all potential faulty logic and flaws are detected.";
  
    let user_message = format!(
      "Users Question: {}\n\nResearch AI Answers:\n\nAnswer 1:\n {}\nAnswer 2:\n {}\nAnswer 3:\n {}\n\n",
      question,
      messages[0],
      messages[1],
      messages[2],
  );
  
    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-4")
        .temperature(0.2)
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
    println!("\n{}\n\n", &response.choices[0].message.content);
    
    Ok(response.choices[0].message.content.clone())
}

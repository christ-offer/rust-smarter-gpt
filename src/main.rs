mod utils;

use std::error::Error;
use std::io;
use utils::intern::process_question;
use utils::researcher::review_answers;
use utils::professor::final_answer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Please enter your question:");
    let mut question = String::new();
    io::stdin().read_line(&mut question)?;

    let responses = process_question(&question.trim()).await?;

    let review = review_answers(&question.trim(), &responses).await?;
    
    let _final_answer = final_answer(&question.trim(), &responses, &review).await?;
    
    Ok(())
}

# SmarterGPT Implementation in Rust - Based on AI-Explained's Smarter GPT concept

## How it works

When the question is sent off:

* The question is first answered by three "interns".
* Before being sent to a "researcher" who analyzes the answers and looks for flaws and logical errors.
* And lastly the whole chain is sent to the "professor" who gives the final answer.

Chat history is fed into the interns to give it context. (But only the question and the final "professor" answer).

## How to use

Set the OPENAI_API_KEY environment variable to your OpenAI API key.

```bash
export OPENAI_API_KEY=your-api-key
```

Then run the program with the question as the first argument.

```bash
cargo run
```

## Credits

All credit goes to AI-Explained for the concept.


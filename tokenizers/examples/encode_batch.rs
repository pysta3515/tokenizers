   
use tokenizers::{Tokenizer};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let tokenizer = Tokenizer::from_file("/workspaces/tokenizers/tokenizers/examples/tokenizer.json")?;

    let encoding = tokenizer.encode("hello, world [PAD]", false)?;
    println!("out:{:?}", encoding.get_tokens());
    Ok(())
}

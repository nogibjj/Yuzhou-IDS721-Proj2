use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "yuzhou",
    about = "A Hugging Face Translation Tool in Rust"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "yuzhou")]
    Translate {
        #[clap(short, long)]
        text: String,
    }
}
// create main function that uses lib.rs
fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Translate { text }) => {
            translate::translate_text(text)?;
        }
        None => {
            println!("No command given");
        }
    }
    Ok(())
}
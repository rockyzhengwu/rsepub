use clap::Parser;

mod commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser, Debug)]
enum Commands {
    CreateAudioBook(commands::sync_audio::Config),
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::CreateAudioBook(cfg)) => commands::sync_audio::command(cfg),
        None => {}
    }
}

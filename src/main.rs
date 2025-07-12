mod watcher;

use std::{error::Error, sync::mpsc};

use clap::Parser;
use watcher::{recursive_file_reader, WatcherEvent};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let (tx, rx) = mpsc::channel::<WatcherEvent>();

    recursive_file_reader(&args.path, tx);

    for event in rx {
        println!("{} changed", event.file_name);
    }

    Ok(())
}

#[derive(Parser, Debug)]
struct Args {
    /// File or folder path
    #[arg(short, long)]
    path: String,
}

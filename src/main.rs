mod watcher;

use std::{error::Error, sync::mpsc};

use watcher::{recursive_file_reader, WatcherEvent};

fn main() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::channel::<WatcherEvent>();

    let file_path = ".";

    recursive_file_reader(file_path, tx);

    for event in rx {
        println!("{} changed", event.file_name);
    }

    Ok(())
}

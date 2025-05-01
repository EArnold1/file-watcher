use std::{
    error::Error,
    path::PathBuf,
    sync::mpsc,
    thread,
    time::{Duration, SystemTime},
};

fn main() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::channel::<()>();

    let file_path = "raw.txt";

    // perform watch in another thread
    // calling the watcher in the main thread will block it and the receiver will still be waiting for the function to finish executing
    thread::spawn(|| watcher(file_path, tx));

    for _ in rx {
        println!("{file_path}: changed");
    }

    Ok(())
}

fn watcher(file_path: &str, tx: mpsc::Sender<()>) -> std::io::Result<()> {
    let mut last_modified: Option<SystemTime> = None;

    let path = PathBuf::from(file_path);

    loop {
        let metadata = path.metadata()?;

        if last_modified.is_none() {
            last_modified = Some(metadata.modified()?); // set last_modified
        }

        let modified = metadata.modified()?;

        if Some(modified) != last_modified {
            let _ = tx.send(()); // send modified signal
            last_modified = Some(modified)
        }

        thread::sleep(Duration::from_millis(300)); // check every 300 millis
    }
}

use std::{
    error::Error,
    path::PathBuf,
    sync::mpsc,
    thread,
    time::{Duration, SystemTime},
};

struct WatcherEvent {
    file_name: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::channel::<WatcherEvent>();

    let file_path = ".";

    recursive_file_reader(file_path, tx);

    for event in rx {
        println!("{} changed", event.file_name);
    }

    Ok(())
}

fn watcher(path: PathBuf, tx: mpsc::Sender<WatcherEvent>) -> std::io::Result<()> {
    let mut last_modified: Option<SystemTime> = None;

    loop {
        let metadata = path.metadata()?;

        if last_modified.is_none() {
            last_modified = Some(metadata.modified()?); // set last_modified
        }

        let modified = metadata.modified()?;

        if Some(modified) != last_modified {
            if let Some(file_name) = path.file_name() {
                match file_name.to_str() {
                    Some(file_name) => {
                        let _ = tx.send(WatcherEvent {
                            file_name: file_name.to_string(),
                        }); // send modified signal
                        last_modified = Some(modified)
                    }
                    None => panic!("something went wrong"),
                }
            }
        }

        thread::sleep(Duration::from_millis(300)); // check every 300 millis
    }
}

fn recursive_file_reader(file_path: &str, tx: mpsc::Sender<WatcherEvent>) {
    let path = PathBuf::from(file_path);

    if path.is_dir() {
        for entry in path.read_dir().expect("read_dir call failed").flatten() {
            if let Some(file_name) = entry.path().to_str() {
                recursive_file_reader(file_name, tx.clone())
            };
        }
    } else {
        // perform watch in another thread
        // calling the watcher in the main thread will block it and the receiver will still be waiting for the function to finish executing
        // thread::spawn(|| watcher(file_path, tx));
        thread::spawn(move || watcher(path, tx));
        // is spawning a new thread for folders with nested files/folder good ?
    }
}

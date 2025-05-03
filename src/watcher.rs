use std::{
    path::PathBuf,
    sync::mpsc,
    thread,
    time::{Duration, SystemTime},
};

macro_rules! some_extractor {
    ($e:expr) => {
        $e.expect("something went wrong")
    };
}

pub struct WatcherEvent {
    pub file_name: String,
}

pub fn watcher(path: PathBuf, tx: mpsc::Sender<WatcherEvent>) -> std::io::Result<()> {
    let mut last_modified: Option<SystemTime> = None;

    loop {
        let metadata = path.metadata()?;

        if last_modified.is_none() {
            last_modified = Some(metadata.modified()?); // set last_modified
        }

        let modified = metadata.modified()?;

        if Some(modified) != last_modified {
            let file_name = some_extractor!(path.canonicalize()?.to_str()).to_string();

            let _ = tx.send(WatcherEvent { file_name }); // send modified signal
            last_modified = Some(modified)
        }

        thread::sleep(Duration::from_millis(300)); // check every 300 millis
    }
}

pub fn recursive_file_reader(file_path: &str, tx: mpsc::Sender<WatcherEvent>) {
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

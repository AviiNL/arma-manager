use std::{
    collections::HashMap,
    io::{Read, SeekFrom},
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use axum::response::sse::Event;
use glob::Pattern;
use notify::{RecursiveMode, Result as NResult, Watcher};
use tokio::{
    io::{AsyncReadExt, AsyncSeekExt},
    sync::broadcast,
};

use futures::{channel::mpsc::unbounded, SinkExt, StreamExt};

#[derive(Clone)]
pub struct LogService {
    map: Arc<RwLock<HashMap<String, WatchOptions>>>,
    tx: broadcast::Sender<Event>,
}

impl Default for LogService {
    fn default() -> Self {
        Self {
            map: Arc::new(RwLock::new(HashMap::new())),
            tx: broadcast::channel(100).0,
        }
    }
}

impl LogService {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&self, channel: impl Into<String>, path_to_file_or_folder: impl Into<PathBuf>) {
        self.watch(channel.into(), path_to_file_or_folder.into());
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.tx.subscribe()
    }

    pub fn get_latest(&self, channel_name: String) -> Vec<String> {
        let map = self.map.read().unwrap();
        let Some(watch_options) = map.get(&channel_name) else {
            return vec![];
        };

        // build a path string to glob against
        let mut path = watch_options.path.clone();

        if let Some(stem) = watch_options.stem.as_ref() {
            path.push(stem.clone());
        } else {
            path.push("*");
        }

        if let Some(extension) = watch_options.extension.as_ref() {
            path.set_extension(extension.clone());
        }

        let path = path.to_str().unwrap();

        let mut glob = glob::glob(path).unwrap();

        let mut files = Vec::new();

        while let Some(Ok(entry)) = glob.next() {
            files.push(entry);
        }

        files.sort_by(|a, b| {
            b.metadata()
                .unwrap()
                .modified()
                .unwrap()
                .cmp(&a.metadata().unwrap().modified().unwrap())
        });

        // take the last modified file
        if let Some(file) = files.first() {
            let mut file = std::fs::File::open(file).unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            let contents = String::from_utf8(buffer).unwrap();

            // map all lines to Vec
            let lines = contents
                .lines()
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            // take the last 100 lines
            let lines = lines
                .iter()
                .rev()
                .take(100)
                .rev()
                .map(|line| line.to_string())
                .collect::<Vec<String>>();

            // COnvert to json array
            return lines;
        }

        vec![]
    }

    fn watch(&self, channel_name: String, path: PathBuf) {
        let watch_options = WatchOptions::new(&channel_name, &path);
        self.map.write().unwrap().insert(channel_name, watch_options.clone());
        let (mut tx, mut rx) = unbounded();

        let mut watcher = notify::recommended_watcher(move |res: NResult<notify::Event>| {
            futures::executor::block_on(async {
                let _ = tx.send(res).await;
            })
        })
        .expect("watcher");

        let tx = self.tx.clone();
        tokio::spawn(async move {
            if let Err(e) = watcher.watch(watch_options.path.as_path(), RecursiveMode::Recursive) {
                println!("watch error: {:?}", e);
                return;
            }

            let options = watch_options.clone();
            let mut last_pos: u64 = 0;
            while let Some(res) = rx.next().await {
                match res {
                    Ok(event) => {
                        if event.kind != notify::EventKind::Modify(notify::event::ModifyKind::Any) {
                            continue;
                        }

                        if let Some(path) = options.matches(&event.paths) {
                            if let Ok(Some((pos, content))) = get_contents_from_pos(path.as_path(), last_pos).await {
                                last_pos = pos;
                                let k = Event::default().event(options.channel.clone()).data(content);
                                _ = tx.send(k);
                            }
                        }
                    }
                    Err(e) => {
                        println!("watch error: {:?}", e);
                    }
                }
            }
        });
    }
}

async fn get_contents_from_pos(file: &Path, pos: u64) -> std::io::Result<Option<(u64, String)>> {
    let mut pos = pos;
    let mut file = tokio::fs::File::open(file).await?;
    let metadata = file.metadata().await?;
    let new_pos = metadata.len();

    if new_pos == 0 {
        return Ok(None); // file is empty
    }

    if new_pos < pos {
        pos = 0;
    }

    file.seek(SeekFrom::Start(pos)).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    // stick every line in a vec
    let contents = contents
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    // json encode the vec
    let contents = serde_json::to_string(&contents).unwrap();

    Ok(Some((new_pos, contents)))
}

#[derive(Clone, Debug)]
struct WatchOptions {
    channel: String,
    path: PathBuf,
    stem: Option<String>,
    extension: Option<String>,
}

impl WatchOptions {
    pub fn new(channel: &str, path: &Path) -> Self {
        let mut watch_options = Self {
            channel: channel.to_owned(),
            path: path.to_path_buf(),
            stem: None,
            extension: None,
        };

        // check if path points to a file or folder
        if path.to_str().unwrap().contains('*') {
            // we'll resolve it in realtime
            watch_options.path = path.parent().unwrap().into();
            let stem = path.file_stem().unwrap().to_str().unwrap().to_string();
            if stem.len() > 1 {
                watch_options.stem = Some(stem);
            }
            watch_options.extension = Some(path.extension().unwrap().to_str().unwrap().to_string());
        } else if let Ok(metadata) = std::fs::metadata(path) {
            if metadata.is_file() {
                watch_options.path = path.parent().unwrap().into();
                let stem = path.file_stem().unwrap().to_str().unwrap().to_string();
                watch_options.stem = Some(stem);
                watch_options.extension = Some(path.extension().unwrap().to_str().unwrap().to_string());
            }
        }

        watch_options
    }

    fn matches(&self, paths: &Vec<PathBuf>) -> Option<PathBuf> {
        // create a string from self.watch_options to comapre against
        let mut path = self.path.clone();
        if let Some(stem) = &self.stem {
            path.push(stem);
        } else {
            path.push("*");
        }

        if let Some(extension) = &self.extension {
            path.set_extension(extension);
        }

        let test_path = path.to_str().unwrap().to_string();

        for path in paths {
            let eosrihgweruihg = path.to_str().unwrap();

            if !Pattern::new(&test_path).unwrap().matches(eosrihgweruihg) {
                continue;
            }

            return Some(path.clone());
        }

        None
    }
}

use anyhow::Result;
use futures::{
    channel::mpsc::{channel, Receiver, Sender},
    SinkExt, StreamExt,
};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::{io::SeekFrom, path::Path};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncSeekExt},
};

pub fn watch() -> Result<Receiver<String>> {
    let path = paths::get_log_path();
    let (tx, rx) = channel(100);

    tokio::spawn(async move {
        if let Err(e) = async_watch(path, tx).await {
            println!("watch error: {:?}", e);
        }
    });

    Ok(rx)
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

async fn async_watch<P: AsRef<Path>>(dir: P, mut tx: Sender<String>) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(dir.as_ref(), RecursiveMode::Recursive)?;

    // find the newest log file
    let mut file = None;
    let mut newest = 0;

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().unwrap_or_default() != "log" {
            continue;
        }

        let metadata = tokio::fs::metadata(&path).await?;
        let modified = metadata.modified()?.elapsed().unwrap().as_millis();

        if modified > newest {
            newest = modified;
            file = Some(path);
        }
    }

    let Some(path) = file else {
        return Ok(());
    };

    let mut contents = tokio::fs::read_to_string(path).await.unwrap();
    let mut pos: u64 = contents.len() as u64;

    // only take the last 100 lines
    let lines = contents.lines().rev().take(100).collect::<Vec<_>>();
    let lines = lines.iter().rev().collect::<Vec<_>>();

    for line in lines {
        if line.len() == 0 {
            continue; // skip blank lines
        }
        if let Err(e) = tx.send(line.to_string()).await {
            println!("send error: {:?}", e);
        }
    }

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => {
                if event.kind != notify::EventKind::Modify(notify::event::ModifyKind::Any) {
                    continue;
                }

                let path = {
                    let mut path = None;
                    for p in event.paths.iter() {
                        if p.extension().unwrap_or_default() == "log" {
                            path = Some(p);
                            break;
                        }
                    }
                    path
                };

                if let Some(path) = path {
                    let Ok(mut f) = File::open(&path).await else {
                        // tracing::error!("failed to open file");
                        continue;
                    };
                    let len = f.metadata().await.unwrap().len();

                    if len == 0 {
                        continue;
                    }

                    if len < pos {
                        // tracing::warn!("file truncated");
                        pos = 0;
                    }

                    f.seek(SeekFrom::Start(pos)).await.unwrap();

                    contents.clear();
                    f.read_to_string(&mut contents).await.unwrap();

                    pos = len;

                    for line in contents.lines() {
                        if line.len() == 0 {
                            continue; // skip blank lines
                        }
                        if let Err(e) = tx.send(line.to_string()).await {
                            println!("send error: {:?}", e);
                            return Ok(());
                        }
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

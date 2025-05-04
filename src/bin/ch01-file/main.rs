use ::std::io::Error;
use ::std::path::PathBuf;
use ::std::time::SystemTime;
use ::tokio::fs::File as AsyncFile;
use ::tokio::io::AsyncReadExt;
use ::tokio::sync::watch::{self, Sender};
use ::tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
  let (tx, mut rx) = watch::channel(());

  tokio::spawn(watch_file_changes(tx));

  loop {
    let _ = rx.changed().await;

    if let Ok(contents) = read_file("data.txt").await {
      println!("{contents}");
    }
  }
}

async fn read_file(filename: &str) -> Result<String, Error> {
  let mut file: AsyncFile = AsyncFile::open(filename).await?;

  let mut contents: String = String::new();

  file.read_to_string(&mut contents).await?;

  Ok(contents)
}

async fn watch_file_changes(tx: Sender<()>) {
  let path: PathBuf = PathBuf::from("data.txt");

  let mut last_modified = None;

  loop {
    if let Ok(metadata) = path.metadata() {
      let modified: SystemTime = metadata.modified().unwrap();

      if last_modified != Some(modified) {
        last_modified = Some(modified);

        let _ = tx.send(());
      }
    }

    time::sleep(Duration::from_millis(100)).await;
  }
}

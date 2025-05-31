use ::futures_util::future::join_all;
use ::std::fs::{File, OpenOptions};
use ::std::future::Future;
use ::std::io::prelude::*;
use ::std::sync::{Arc, Mutex};
use ::std::task::{Context, Poll};
use ::tokio::task::JoinHandle;

type AsyncFileHandle = Arc<Mutex<File>>;

type FileJoinHandle = JoinHandle<Result<bool, String>>;

struct AsyncWriteFuture {
  entry: String,
  handle: AsyncFileHandle,
}

impl Future for AsyncWriteFuture {
  type Output = Result<bool, String>;

  fn poll(
    self: std::pin::Pin<&mut Self>,
    context: &mut Context<'_>,
  ) -> Poll<Self::Output> {
    let result: Result<
      std::sync::MutexGuard<'_, File>,
      std::sync::TryLockError<std::sync::MutexGuard<'_, File>>,
    > = self.handle.try_lock();

    let mut guard: std::sync::MutexGuard<'_, File> = match result {
      Ok(guard) => guard,
      Err(error) => {
        println!("error for {}: {}", self.entry, error);

        let waker: &std::task::Waker = context.waker();

        waker.wake_by_ref();

        return Poll::Pending;
      },
    };

    let lined_entry: String = format!("{}\n", self.entry);

    let buf: &[u8] = lined_entry.as_bytes();

    match guard.write_all(buf) {
      Ok(_) => println!("written for: {}", self.entry),
      Err(e) => println!("{e}"),
    }

    Poll::Ready(Ok(true))
  }
}

#[tokio::main]
async fn main() {
  let login_handle: AsyncFileHandle = get_handle(&"login.txt");

  let logout_handle: AsyncFileHandle = get_handle(&"logout.txt");

  let names: [&'static str; 6] = [
    "one", "two", "three", "four", "five", "six",
  ];

  let mut handles: Vec<FileJoinHandle> = Vec::new();

  for name in names {
    let file_handle_one: AsyncFileHandle = login_handle.clone();

    let file_handle_two: AsyncFileHandle = logout_handle.clone();

    let line: String = name.to_string();

    let handle_one: FileJoinHandle = write_log(file_handle_one, line.clone());

    let handle_two: FileJoinHandle = write_log(file_handle_two, line);

    handles.push(handle_one);

    handles.push(handle_two);
  }

  let _ = join_all(handles).await;
}

fn get_handle(file_path: &dyn ToString) -> AsyncFileHandle {
  let mut open_options: OpenOptions = OpenOptions::new();

  let _: &mut OpenOptions = open_options.append(true);

  let path: String = file_path.to_string();

  let result: Result<File, std::io::Error> = open_options.open(path.clone());

  let file = match result {
    Ok(opened_file) => opened_file,
    Err(_) => File::create(path).unwrap(),
  };

  let data: Mutex<File> = Mutex::new(file);

  Arc::new(data)
}

fn write_log(
  file_handle: AsyncFileHandle,
  line: String,
) -> FileJoinHandle {
  let future: AsyncWriteFuture = AsyncWriteFuture {
    entry: line,
    handle: file_handle,
  };

  tokio::task::spawn(future)
}

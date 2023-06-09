use std::{future::Future, thread::sleep, time::Duration};

struct ReadFileFuture {}

impl Future for ReadFileFuture {
  type Output = String;

  fn poll(
    self: std::pin::Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Self::Output> {
    println!("Tokio! stop polling me");
    cx.waker().wake_by_ref();
    // std::task::Poll::Pending
    std::task::Poll::Ready(String::from("Hello there, from file 1"))
  }
}

fn read_from_file_2() -> impl Future<Output = String> {
  async {
    sleep(Duration::new(2, 0));
    println!("Processing file 2");
    String::from("Hello there, from file 2!")
  }
}

#[tokio::main]
async fn main() {
  println!("***** Running Futures Example *****");

  println!("Hello before reading file!");

  let handle_1 = tokio::spawn(async {
    let future_1 = ReadFileFuture {};
    future_1.await
  });

  let handle_2 = tokio::spawn(async {
    let file_contents_2 = read_from_file_2().await;
    println!("{file_contents_2:?}");
  });

  let _ = tokio::join!(handle_1, handle_2);
}

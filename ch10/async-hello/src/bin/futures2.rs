use std::{
  future::Future,
  task::Poll,
  thread::sleep,
  time::{Duration, Instant},
};

struct AsyncTimer {
  expiration_time: Instant,
}

impl Future for AsyncTimer {
  type Output = String;

  fn poll(
    self: std::pin::Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Self::Output> {
    if Instant::now() >= self.expiration_time {
      println!("Hello, it's time for future 1");
      Poll::Ready(String::from("Future 1 has completed"))
    } else {
      println!("Hello, it's not yet time for future 1. Going to sleep");

      let waker : std::task::Waker = cx.waker().clone();
      let expiration_time = self.expiration_time;

      std::thread::spawn(move || {
        let current_time = Instant::now();

        if current_time < expiration_time {
          std::thread::sleep(expiration_time - current_time);
        }

        waker.wake();
      });
      Poll::Pending
    }
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
  println!("***** Running Futures Example with Timer *****");

  println!("Hello before reading file!");

  let handle_1 = tokio::spawn(async {
    let future_1: AsyncTimer = AsyncTimer {
      expiration_time: Instant::now() + Duration::from_millis(4000),
    };

    println!("{0:?}", future_1.await);
  });

  let handle_2 = tokio::spawn(async {
    let file_contents_2 = read_from_file_2().await;
    println!("{file_contents_2:?}");
  });

  let _ = tokio::join!(handle_1, handle_2);
}

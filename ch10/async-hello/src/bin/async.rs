use std::thread::sleep;
use std::time::Duration;

async fn read_from_file_1() -> String {
  sleep(Duration::new(4, 0));
  println!("Processing file 1");
  String::from("Hello there, from file 1!")
}

async fn read_from_file_2() -> String {
  sleep(Duration::new(2, 0));
  println!("Processing file 2");
  String::from("Hello there, from file 2!")
}

#[tokio::main]
async fn main() {
  println!("***** Running with Async *****");

  println!("Hello before reading file!");

  let handle_1 = tokio::spawn(async {
    let file_contents = read_from_file_1().await;
    println!("{file_contents:?}");
  });

  let handle_2 = tokio::spawn(async {
    let file_contents = read_from_file_2().await;
    println!("{file_contents:?}");
  });

  let _ = tokio::join!(handle_1, handle_2);
}

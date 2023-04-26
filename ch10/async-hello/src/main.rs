use std::time::Duration;
use std::{thread, thread::sleep};

fn read_from_file() -> String {
  sleep(Duration::new(2, 0));
  String::from("Hello there!")
}

fn read_from_file_1() -> String {
  sleep(Duration::new(2, 0));
  String::from("Hello there, from file 1!")
}

fn read_from_file_2() -> String {
  sleep(Duration::new(4, 0));
  String::from("Hello there, from file 2!")
}

async fn read_from_file_3() -> String { 
  sleep(Duration::new(4, 0));
  println!("Processing file 3");
  String::from("Hello there, from file 3!")
}

async fn read_from_file_4() -> String {
  sleep(Duration::new(2, 0));
  println!("Processing file 4");
  String::from("Hello there, from file 4!")
}

fn run_sequential() {
  println!("Hello before reading file!");

  // let file_contents: String = read_from_file();
  // println!("{file_contents:?}"); // Note: debug formatting adds the extraquotation marks

  let file_contents: String = read_from_file_1();
  println!("{file_contents:?}");
  println!("Hello after reading file 1!");

  let file_contents: String = read_from_file_2();
  println!("{file_contents:?}");
  println!("Hello after reading file 2!");
}

fn run_thread() {
  println!("Hello before reading file!");

  let handle_1 = thread::spawn(|| {
    let file_contents = read_from_file_1();
    println!("{file_contents:?}");
  });

  let handle_2 = thread::spawn(|| {
    let file_contents = read_from_file_2();
    println!("{file_contents:?}");
  });

  handle_1.join().unwrap();
  handle_2.join().unwrap();
}

fn main_1() {
  // run_sequential();
  // run_thread();
}

#[tokio::main]
async fn main() {
  println!("Hello before reading file!");

  let handle_1 = tokio::spawn(async {
    let file_contents = read_from_file_3().await;
    println!("{file_contents:?}");
  });

  let handle_2 = tokio::spawn(async {
    let file_contents = read_from_file_4().await;
    println!("{file_contents:?}");
  });

  let _ = tokio::join!(handle_1, handle_2);
}

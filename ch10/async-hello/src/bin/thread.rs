use std::thread::{self, sleep};
use std::time::Duration;

fn read_from_file_1() -> String {
  sleep(Duration::new(2, 0));
  String::from("Hello there, from file 1!")
}

fn read_from_file_2() -> String {
  sleep(Duration::new(4, 0));
  String::from("Hello there, from file 2!")
}

fn main() {
  println!("***** Running with Threads *****");
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

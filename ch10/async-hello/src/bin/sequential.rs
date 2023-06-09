use std::thread::sleep;
use std::time::Duration;

// fn read_from_file() -> String {
//   sleep(Duration::new(2, 0));
//   String::from("Hello there!")
// }

fn read_from_file_1() -> String {
  sleep(Duration::new(2, 0));
  String::from("Hello there, from file 1!")
}

fn read_from_file_2() -> String {
  sleep(Duration::new(4, 0));
  String::from("Hello there, from file 2!")
}

fn main() {
  println!("***** Running with Sequential *****");

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

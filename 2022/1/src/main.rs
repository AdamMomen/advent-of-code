use std::{fs::OpenOptions, string};

fn main() {
     let file = OpenOptions::new()
          .read(true)
          .write(false)
          .open("test.txt");


      file match {
         Some(_) => { println!("{}", _) },
     }
}

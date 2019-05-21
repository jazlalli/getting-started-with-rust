use std::io;

fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }

  &s[..]
}

fn main() {
    let mut text = String::new();
    println!("Enter some text...");

    io::stdin().read_line(&mut text)
      .expect("Failed to read input");

    let result: &str = first_word(&text);
    println!("First word is: {}", result);
}

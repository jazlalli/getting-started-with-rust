use std::io;

fn fib(n: u32) -> u32 {
  // starting values for fib numbers and iteration idx
  let mut prev_1: u32 = 1;
  let mut prev_2: u32 = 1;
  let mut iteration: u32 = 2;

  let mut tmp: u32;
  let mut next: u32 = prev_1;

  let mut result: String = format!("{}, {}, ", prev_1, prev_2);

  while iteration <= n {
    if iteration < 1 {
      break;
    }

    tmp = prev_1.wrapping_add(prev_2);
    prev_1 = prev_2;
    prev_2 = tmp;
    next = tmp;

    result.push_str(&(next.to_string() + ", "));

    iteration += 1;
  }

  println!("{}", result);

  next
}

fn main() {
    println!("Which fibonacci number do you want (zero-based, i.e. the first one is 0)?");

    let mut fib_number = String::new();

    io::stdin().read_line(&mut fib_number)
        .expect("Failed to read input");

    let fib_number: u32 = fib_number.trim().parse().unwrap();

    println!("Calculating Fib({})", fib_number.to_string());
    println!("Result: {}", fib(fib_number));
}

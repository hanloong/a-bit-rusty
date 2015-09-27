fn main() {
  for n in (1..101).map(to_fzb) {
    println!("{}", n);
  }
}

fn to_fzb(n: u64) -> String {
  match n {
    n if n % 15 == 0 => "FizzBuzz".to_string(),
    n if n % 5 == 0 => "Buzz".to_string(),
    n if n % 3 == 0 => "Fizz".to_string(),
    n => n.to_string(),
  }
}

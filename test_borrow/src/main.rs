fn main() {
  let mut out = "Hello world!".to_string();
  println!("{out}");
  let r = &mut out;
  r.push_str(" - from Rust");
  println!("{out}");
  let mut i = 42;
  i += 2;
  let d:f64;
}

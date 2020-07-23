use whistle_core;
use whistle_vm;

fn main() {
  println!("Hello, cli!");
  whistle_core::run();
  whistle_vm::run();
}

use whistle_engine::engine::Engine;
use whistle_engine::{value::Value, chunk::{Chunk, OpCode}};

fn main() {
  let mut engine = Engine::new();
  let chunk = engine.get_chunk();

  let idx = chunk.add_constant(Value::Float(32_f32));
  chunk.write(OpCode::Constant(idx), 1);
  chunk.write(OpCode::Return, 2);

  let encoded: Vec<u8> = chunk.serialize().unwrap();

  let decoded: Chunk = Chunk::deserialize(&encoded[..]).unwrap();

  println!("{:?}", decoded);

  engine.interpret();
}

use ::SayHello;

pub struct English;

impl SayHello for English {
  fn say_hello() -> String {
    "hello, world".to_string()
  }
}

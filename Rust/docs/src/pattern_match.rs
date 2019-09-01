
fn main() {
  let five = Some(5);
  let six = plus_one(five);
  println!("six is {:?}", six);

  let some_u8_value = Some(0u8);
  match some_u8_value {
    Some(3) => println!("three" ),
    _ => ()
  }

  if let Some(3) = some_u8_value {
    println!("three!", )
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i+1),
  }
}


#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

// メソッド
impl Rectangle {
  fn square(size: u32) -> Rectangle {
    Rectangle {width:size, height:size}
  }

  // add code here
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

fn main() {

  let rect1 = (30, 50);

  let rect2 = Rectangle {width: 30, height: 50};
  let rect3 = Rectangle {width: 15, height: 30};

  println!("四角形は{:#?}", rect2);
  println!("面積は{}です", area_struct(&rect2));

  println!("面積は{}です！", area_tuple(rect1));

  // メソッド
  println!("メソッドver {}", rect2.area());

  println!("大きさ比較 {}", rect2.can_hold(&rect3));

  let sq1 = Rectangle::square(3);

  println!("正方形 {:#?}", sq1);
}

fn area(w: u32, h: u32) -> u32 {
  w * h
}

fn area_tuple(dimensions: (u32,u32)) -> u32 {
  dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}

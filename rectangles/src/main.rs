#[derive(Debug)]
struct Rectangle {
  height: u32,
  width: u32,
}

// dead code allow lets me have methods that are never used
#[allow(dead_code)]
impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    return self.width > other.width && self.height > other.height
  }
}

impl Rectangle {
  fn square(size: u32) -> Rectangle {
      Rectangle {
          width: size,
          height: size,
      }
  }
}

fn main() {
  let rect1 = Rectangle{ width: 30,  height: 50 };
  let rect2 = Rectangle{ width: 10, height: 40 };
let rect3 = Rectangle{ width: 60, height: 45 };
  // println!("rect1 is {:#?}",  rect1);
  // println!("the area of the rectangle is {} square pixels", rect1.area());
  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
  let length = 32;
  println!("the area of a square of length {} is {} square pixels", length,  Rectangle::square(length).area());

}
pub fn add_two(a: u64) -> u64 {
  a + 2
}


#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn it_adds_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
  }
  
}

// #[test]
// fn exploration() {
//   let result = add(2, 2);
//   assert_eq!(result, 4);
// }
// #[test]
// fn larger_can_hold_smaller() {
//   let larger = Rectangle {
//     width: 8,
//     height: 7,
//   };
//   let smaller = Rectangle {
//     width: 5,
//     height: 1,
//   };
//   assert!(larger.can_hold(&smaller));
// } 

// #[test]
// #[test]
// fn smaller_cannot_hold_larger() {
//   let larger = Rectangle {
//     width: 8,
//     height: 7,
//   };
//   let smaller = Rectangle {
//     width: 5,
//     height: 1,
//   };
//   assert!(!smaller.can_hold(&larger));
// }
// fn another() {
  //   panic!("Make this test fail");
  // }
  //   #[derive(Debug)]
// struct Rectangle {
//   width: u32,
//   height: u32,
// }

// impl Rectangle {
//   fn can_hold(&self, other: &Rectangle) -> bool {
//     self.width < other.width && self.height > other.height
//   }
// }
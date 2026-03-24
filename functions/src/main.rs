#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2,3), 5);
        assert_eq!(add(-1,1), 0);
    }

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn test_divide_by_zero() {
        divide(10, 0).unwrap();
    }
}

fn main() {
    
}

// #[inline(always)]
// fn small_operation(x: i32) -> i32 {
//     x * 2
// }

// fn with_function_ptr(x: i32, f: fn(i32) -> i32) -> i32 {
//     f(x)
// }

// fn with_closure<F: Fn(i32) -> i32>(x: i32, f: F) -> i32 {
//     f(x)
// }



// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() { x } else { y }
// }

// struct Excerpt<'a> {
//     part: &'a str,
// }

// fn main() {
//     let text = String::from("Hello world");
//     let excerpt = Excerpt { part: &text };
// }

// fn panic_function() -> ! { 
//     panic!("This function never returns");
// }

// fn loop_forever() -> ! {
//     loop {

//     }
// }


// fn takes_ownership(s: String) {
//     println!("{}", s);
// }

// fn borrows(s: &String) {
//     println!("{}", s);
// }

// fn main() {
//     let s = String::from("hello");

//     takes_ownership(s);

//     let s2 = String::from("world");
//     borrows(&s2);
//     println!("{}", s2);

// }


// fn identity<T>(value: T) -> T {
//     value
// }

// fn print_value<T: std::fmt::Debug>(value: T) {
//     println!("{:?}", value);
// }

// fn swap<A, B>(a: A, b: B) -> (B, A) {
//     (b,a)
// }


// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn scale(&mut self, factor: u32) {
//         self.width *= factor;
//         self.height *= factor;
//     }

//     fn new(width: u32, height: u32) -> Self {
//         Self {width, height}
//     }
// }

// fn main() {
//     let rect = Rectangle::new(10, 20);
//     println!("Area: {}", rect.area());
// }


// fn main() {
//     let add_five = make_adder(5);
//     let result = add_five(10);
//     println!("{}", result);
// }



// fn make_adder(amount: i32) -> impl Fn(i32) -> i32 {
//     move |x| x + amount
// }
// let add_one = |x: i32| -> i32 {x+ 1};
// let add_one = |x| x + 1;

// let factor = 2;

// let multiply = |x| x * factor;

// let mut counter = 0;
// let mut increment = || {
//     counter += 1;
//     counter
// };

// let data = vec![1,2,3];
// let consume = move || {
//     println!("{:?}", data);
// }

// fn main() {
//     let result = apply_operation(5,3, add);
//     let result2 = apply_operation(5,3, subtract);
//     println!("{} {}", result, result2);
// }

// fn add(x: i32, y: i32) -> i32 {
//     x + y
// }

// fn subtract(x: i32, y: i32) -> i32 {
//     x - y
// }

// fn apply_operation(x: i32, y: i32, operation: fn(i32, i32) -> i32) -> i32 {
//     operation(x, y)
// }

// fn get_value() -> i32 {
//     42
// }

// fn get_coordinates()  -> (f64, f64) {
//     (10.0, 20.0)
// }

// fn first_word(s: &str) -> &str {
//     &s[..1]
// }

// fn divide(a: f64, b: f64) -> Result<f64, String>
//  {
//     if b == 0.0 {
//         Err("Division by zero".to_string())
//     } else {
//         Ok(a/b)
//     }
//  }

//  fn find_user(id: u32) -> Option<String> {
//     if id == 1 {
//         Some("Alice".to_string())
//     } else {
//         None
//     }
//  }
// fn process(x: i32, y: String, z: &str) -> bool {

// }

// fn no_params() {
//     println!("Hello");
// }

// fn no_return() {

// }

// fn add (x: i32, y: i32) -> i32 {
//     x + y
// }

// fn multiply(x: i32, y: i32) -> i32 {
//     return x * y;
// }

// fn by_value(x: i32) {

// }

// fn by_reference(x: &i32) {

// }

// fn by_mutable_reference(x: &mut i32) {
//     *x += 1;
// }


// fn main() {
//     // another_function(5);
//     // print_labeled_measurement(5, 'h');
//     // let x = (let y = 6);
//     // let y = {
//     //     let x = 3;
//     //     x + 1
//     // };
//     // println!("The value of y is : {y}");
//     let x = plus_one(5);
//     println!("The value of x is: {x}");
// }

// // fn five() -> i32 {
// //     5
// // }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }


// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}")
// }


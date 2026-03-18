fn main() {
    let s1:String = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of {} is {}", s2, len);
}

fn calculate_length(s:String) -> (String, usize) {
    let length:usize = s.len();
    return (s, length);
}


// fn main() {
//     let s1:String = get_string();
//     println!("This is s1: {}", s1);

//     let s2:String = String::from("world");
//     let s3:String = send_get_string(s2); //Ownership Transferred

//     println!("This is s3:{}", s3);
// }

// fn get_string() -> String {
//     let new_string = String::from("hello");
//     return new_string;
// }

// fn send_get_string(received_string:String) -> String {
//     return received_string;
// }

// fn main() {
//     let x:String = String::from("Hello");
//     process_string(x);
//     // println!("The value of x in main() is {}", x);
// }

// fn process_string(item:String) {
//     println!("The value of x in process_string() is {}", item);
// }

// fn main() {
//     let num1:u8 = 10;
//     let num2:u8 = 20;
//     let result:u8 = add(num1,num2);
//     println!("The sum of num1 and num2 is {}", result);
// }

// fn add(item1:u8, item2:u8) -> u8 {
//     return item1 + item2;
// }

// fn main() {
//     print_value(5);
// }

// fn print_value(item:u8) {
//     println!("{}", item);
// }



// use std::io;

// fn main() {
//     let emp_info:(&str, u8) = ("Ramesh", 50);

//     let emp_name = emp_info.0;
//     let emp_age = emp_info.1;

//     let (employee_name, employee_age) = emp_info;

//     println!("Employee Name = {}, Employee Age = {}", employee_name, employee_age);

//     println!("Employee name = {}, Employee Age = {}", emp_name, emp_age);
// }

// fn main() {
//     let mut string_literal:String = String::from("Hi, Code Eaters!!!");
//     string_literal.push_str(" What's up?");
//     println!("This is string literal {}", string_literal);
// }

// fn main() {
    // let a = [1,2,3,4,5];

    // println!("Please enter an array index.");

    // let mut index = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");

    // let element = a[index];

    // println!("The value of the element at index {index} is: {element}")
    // let x = 2.0;

    // let y: f32 = 3.0;

    // let t = true;

    // let f: bool = false;

    // let c = 'z';
    // let z: char = 'Z';
    // let heart_eyed_cat = '😻';

//     let tup: (i32, f64, u8) = (500, 6.4,1);

//     let (x, y,z ) = tup;

//     println!("The value of y is: {y}");
//     println!("{0}",x);
//     println!("{0}",z);

//    math();
    //   arrays();




// fn math() {
//      //addition
//      let sum = 5 + 10;
//         println!("{sum}");

//      //subtraction
//      let difference = 95.5 -4.3;
//         println!("{difference}");

 
//      //multiplication
//      let product = 4 * 30;
//         println!("{product}");
 
//      //division
//      let quotient = 56.7 / 32.2;
//      let truncated = -5 /3;
//         println!("{quotient}");
//         println!("{truncated}");
//      //remainder
//      let remainder = 43 % 5;
//         println!("{remainder}");

//     // let x: (i32, f64, u8) = (500, 6.4, 1);

//     // let five_hundred = x.0;
//     // let six_point_four = x.1;
//     // let one = x.2;
// }

// fn arrays() {
//     let a: [i32; 5] = [1,2,3,4,5];

//     let months = ["January", "February", "March", "April", "May", "June"];

//     let b = [3;5];

//     println!("{}", a[0]);
//     println!("{}", a[1]);
//     println!("{}", months[4]);
//     println!("{}", b[3]);
// }
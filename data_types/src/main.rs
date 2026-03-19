fn main() {
    let mut vrr: Vec<&str> = vec!["Hello", "World", "Coders"];
    // write_vrr(&vrr);
    // write_vrr(vrr.clone());
    write_vrr(&mut vrr);

    println!("vrr={:?}", vrr);
}

fn write_vrr(vrr2: &mut Vec<&str>) {
    vrr2.push("Eater");
    println!("vrr2= {:?}", vrr2);
}


// fn main() {
//     // let mut v:Vec<i32> = Vec::new();
//     // let mut v = Vec::<i32>::new();

//     // v.push(1);
//     // v.push(2);
//     // v.push(3);

//     let mut v = vec![1,2,3,4,5];
//     v.push(10);
//     v.pop();

//     println!("Vector v={:?}", v);
// }



// fn main() {
//     let arr: [&str; 3] = ["Hello", "World", "Coders"];
//     read_arr(&arr);
//     println!("arr={:?}", arr);

//     // let mut arr: [&str; 3] = ["Hello", "World", "Coders"];
//     // write_arr(&mut arr);
//     // println!("arr={:?}", arr);
//     // let arr: [&str; 3] = ["Hello", "World", "Coders"];
//     // write_arr(arr);
//     // println!("arr={:?}", arr);
// }

// fn read_arr(arr2:& [&str; 3]) {
//     println!("arr2={:?}", arr2);
//     // arr2[0] = "Fellow";
//     // println!("arr2={:?}", arr2);
//     // arr1[0] = "Fellow";
//     // println!("arr1={:?}",arr1);
// }


// fn main() {
//     let mut arr1;

//     arr1 = [1,2,3,4,5];

//     println!("arr1[0]={}", arr1[0]);
//     println!("arr1={:?}", arr1);

//     arr1[2] = 30;
//     println!("arr1={:?}", arr1);

//     println!("Array length is {}", arr1.len());

// }



// mod manufacturer {
//     pub mod factory {
//         pub fn make_product() {
//             println!("Making Product.");
//         }

//         pub fn package_product() {
//             println!("Packaging product");
//         }
//     }
// }

// mod store_a {
//     use crate::manufacturer::factory::make_product;

//     pub fn sell_to_customer() {
//         make_product();
//     }
// }

// fn main() {
//     let reference_to_nothing = create_string_ref();
// }

// fn create_string_ref() -> &String {
//     let s:String = String::from("hello");
//     return &s;
// }


// fn main() {
//     let x = 5;
//     // println!("address={:p}", &x);
//     let y = &x;
//     println!("{}", y);
// }


// mod building {
//     pub mod first_floor {
//         pub mod room_101 {
//             pub fn say_hello() {
//                 println!("Hello from Room 101!");
//             }

//             pub fn call_neighbor() {
//                 crate::building::first_floor::room_102::say_hi();
//                 super::room_102::say_hi();
//                 self::say_hello();
//             }
//         }

//         pub mod room_102 {
//             pub fn say_hi() {
//                 println!("Hi from Room 102!");
//             }

//             pub fn call_upstairs() {
//                 super::super::second_floor::room_201::say_hey();
//             }
//         }
//     }

//     pub mod second_floor {
//         pub mod room_201 {
//             pub fn say_hey() {
//                 println!("Hey from Room 201!");
//             }
//             pub fn call_downstairs() {
//                 super::super::first_floor::room_101::say_hello();
//             }
//         }
//     }
// }

// fn main() {
//     crate::building::first_floor::room_101::say_hello();
//     building::first_floor::room_101::say_hello();

//     building::first_floor::room_101::call_neighbor();
//     building::first_floor::room_102::call_upstairs();
//     building::second_floor::room_201::call_downstairs();
// }

// fn main() {
//     let mut s1:String = String::from("hello");
//     let w1 = &mut s1;
//     w1.push_str(" World");
//     println!("w1 = {}", w1);
//     let w2 = &mut s1;
//     w2.push_str(" Code");
//     println!("w2 = {}", w2);

// }


// fn main() {
//     let mut s1:String = String::from("Hello");
//     append_string(&mut s1);
//     println!("The new string is {}", s1);
// }

// fn append_string(s3:&mut String) {
//     s3.push_str(" World");
// }

// fn main() {
//     let s1:String = String::from("hello");
//     let len:usize = calculate_length(&s1);
//     println!("The length of {} is {}", s1, len);
// }

// fn calculate_length(s2:&String) -> usize {
//     return s2.len();
// }

// fn main() {
//     let s1:String = String::from("hello");
//     let len = calculate_length(s1.clone());
//     println!("The length of {} is {}", s1, len);
// }

// fn calculate_length(s:String) -> usize {
//     let length:usize = s.len();
//     return length;
// }


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
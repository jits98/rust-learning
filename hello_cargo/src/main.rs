fn non_repeating(s: &str) -> char {
    let new;
}



// fn reverse_vector(v: Vec<i32>) -> Vec<i32> {
//     let reversed: Vec<i32> = v.into_iter().rev().collect();

//     reversed
// }

// fn main() {
//     let vec = vec![1,2,3,4];

//     println!("The reverse vector is: {:?}", reverse_vector(vec));
// }


// fn count_chars(s: &str) -> usize {
//     s.len()
// }

// fn main() {
//     let word = String::from("rust");

//     println!("The number of characters in word: {}", count_chars(&word));
// }



// fn even_number(v: &Vec<i32>) -> Vec<i32> {
//     let mut even_vec = Vec::new();
//     for num in v {
//         if num % 2 == 0 {
//             even_vec.push(*num);
//         }
//     }
//     even_vec
// }

// fn main() {
//     let list = vec![1,2,3,4,5,6];

//     println!("The even numbers in list: {:?}", even_number(&list));
// }


// fn main() {
//     let rect1 = Rectangle {
//         width: 20.00,
//         height: 30.00,
//     };
//     let rect2 = Rectangle {width: 30.00, height:40.00};

//     let square = Rectangle::square(10.00);

//     println!("{:?}", rect1);

//     println!("Area of Rectangle: {:?}", rect1.area());

//     println!("The answer of holding is: {}", rect1.can_hold(&rect2));

//     println!("The square is: {:?} ", square);
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: f64,
//     height: f64,
// }

// impl Rectangle {
//     fn area(&self) -> f64 {
//         self.width * self.height
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }

//     fn square(size: f64) -> Rectangle {
//         Rectangle { width: size, height : size}
//     } 
// }



// use std::io;

// fn main() {
//     fizzbuzz();
    

// }

// fn fizzbuzz() {

//     println!("Enter a number between 1 and 100 for print fizzbuzz");

//     let mut input = String::new();

//     io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read the input");

//     let num = input.trim().parse().expect("Please enter a valid integer");

//     if num < 1 || num > 100 {
//         println!("Please enter a number between 1 and 100")
//     };


//     for number in 1..=num {
//         if number % 3 == 0 && number % 5 == 0 {
//             println!("FizzBuzz");
//         } else if number % 3 == 0 {
//             println!("Fizz");
//         } else if number % 5 == 0 {
//             println!("Buzz");
//         } else {
//             println!("{}", number)
//         }
    
// }
// }

// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// fn greet(name: &str) -> String {
//     let greeting = String::from("Hello, ") + name;
//     greeting
// }

// fn is_even(num: i32) -> bool {
//     num % 2 == 0
// }



    // let mut your_age: i32 = 26;
    // println!("My age is: {}", your_age);

    // let height: f64 = 1.72;
    // println!("My height in meters: {}", height);

    // let name = String::from("Jitendra Singh");
    // println!("My name is {}", name);

    // let whether_you_like_coding = true;
    // println!("I like coding which is {}", whether_you_like_coding);

    // your_age = 27;
    // println!("My age is {}", your_age);

    // println!("The sum of 13 and 12 is {}", add(13, 12));

    // let name = String::from("Jitendra Singh");
    // println!("{}", greet(&name));

    // println!("{}", is_even(5));


fn main() {

}



// fn read_username() -> Result<String, io::Error> {
    //     let mut f = File::open("hello.txt")?;
    //     let mut s = String::new();
    //     f.read_to_string(&mut s)?;
    //     Ok(s)
// }
// let f = File::open("hello.txt").unwrap();
// let f = File::open("hello.txt").expect("Failed");  
// use std::fs::File;
 // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // let file = File::open("hello.txt");
    // let file = match file {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating file: {:?}", e),
    //         },
    //         other_error => panic!("Problem opening file: {:?}", other_error),
    //     }
    // };

// // panic!("crash and burn");

// let v = vec![1,2,3];
// v[99];
// use std::collections::HashMap;
// use std::io;
// fn gives_ownership() -> String {
//     String::from("hello")
// }

// fn takes_and_gives_back(s: String) -> String {
//     s
// }

// fn take_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn make_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }

// fn add(x: i32, y: i32) -> i32 {
//     x + y
// }
  // let condition = true;

    // let number = if condition { 5 } else { 10 };
    // println!("{}", number);

    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // while number != 0 {

    // }
    //    enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(u8, u8, u8),
//    }

    // for element in [1,2,3].iter() {
    //     println!("{}", element);
    // }

    // for number in 1..4 {
    //     println!("{}", number);
    // }

    // let result = add(5,3);
    // println!("{}", result);

    // let tup = (500, "hello", true);
    // let (x,y,z) = tup;
    // let first = tup.0;

    // let arr = [1,2,3,4,5];
    // let first = arr[0];
    // let mut guess = String::new();

    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read");

    // let guess: u32 = guess.trim().parse().expect("Not a number!");

    // match guess.cmp(&secret_number) {
    //     Ordering::Less => println!("Too small"),
    //     Ordering::Greater => println!("Too big!"),
    //     Ordering::Equal => println!("You win!"),
    // }
    // let s = String::from("hello");
    // take_ownership(s);

    // let x = 5;
    // make_copy(x);  
    // let s1 = gives_ownership();
    //    let s2 = String::from("hello");
    //    let s3 = takes_and_gives_back(s2);
     // let s = String::from("hello world");
    // let hello = &s[0..5];
    // println!("{}", hello);
    // let world = &s[6..11];
    // println!("{}", world);
    // let whole = &s[..];  
    // println!("{}", whole); 
    // let s: &str = "hello";
    // println!("{}", s);
        // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // let some_number = Some(5);
    // let some_string = Some("hello");
    // let absent_number: Option<i32> = None;
    // let x: i32 = 5;
    // let y: Option<i32> = Some(10);
    // let sum = x + y;

    // fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }
 // let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("Maximum is {}", max),
    //     _ => (),
    // }

    // if let Some(max) = config_max {
    //     println!("Maximum is {}", max);
    // }
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1,2,3];

    // let mut v = Vec::new();
    // v.push(5);
    // v.push(6);
    // v.pop();
    // println!("{:?}", v);
    // let third = v.get(2);
    // println!("{:?}", third);

    // for i in &v {
    //     println!("{}", i);
    // }

    // for i in &mut v {
    //     *i += 50;
    //     println!("{}", i);
    // }
    // let mut s = String::from("foo");
    // s.push_str("bar");
    // s.push('!');
    // s += "baz";
    // println!("{}", s);
        // let s = format!("{}-{}-{}", "a", "b", "c");
    // println!("{}", s);
    // let hello =  "नमस्ते";
    // for c in hello.chars() {
    //     println!("{}", c);
    // }
      // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 10);
    // println!("{:?}", scores);

    // let score = scores.get("Blue");
    // println!("{:?}", score);

    // scores.entry(String::from("Blue")).or_insert(50);
    // scores.entry(String::from("Red")).or_insert(75);
    // println!("{:?}", scores);
    // for (key, value) in &scores {
    //     println!("{}: {}", key, value);
    // }

    // fn read_username() -> Result<String, io::Error> {
    //     let mut s = String::new();
    //     File::open("hello.txt")?.read_to_string(&mut s)?;
    //     Ok(s);
    // }

    // fn read_username() -> Result<String, io::Error> {
    //     fs::read_to_string("hello.txt")
    // }


// #[derive(Debug)]
// struct Book {
//     title: String,
//     author: String,
//     pages: u32,
// }

// impl Book {
//     fn is_long(&self) -> bool {
//         self.pages > 300
//     }

//     fn same_author(&self, other: &Book) -> bool {
//         self.author == other.author
//     }

//     fn describe(&self) -> String {
//         format!("{} by {}, and total pages in the book are {}", self.title, self.author, self.pages)
        
//         // self.title.clone() + " by " + &self.author + ", " + &self.pages.to_string();

//     }

//     fn new(title: &str, author: &str, pages: u32) -> Book {
//         Book { 
//             title: title.to_string(), 
//             author: author.to_string(), 
//             pages 
//         }
//     }

//     fn is_longer_than(&self, other: &Book) -> bool {
//         self.pages > other.pages      
//     }
// }


// fn main() {
//     let book1 = Book::new("Exercised", "Daniel", 350);

//     let book2 = Book::new("My life in full", "Indira Nooyi", 280);

//     let book3 = Book::new("Edible Economics", "Ho joon chong", 320);

//     println!("{}", book1.describe());
//     println!("{}", book2.describe());
//     println!("{}", book3.describe());

//     println!("The first is more than 300 pages: {}", book1.is_long());
//     println!("The second book is more than 300 pages: {}", book2.is_long());
//     println!("The third book is more than 300 pages: {}", book3.is_long());

//     println!("The first book is more lengthy than second book: {}", book1.is_longer_than(&book2));

//     println!("The first and second book has same author: {}", book1.same_author(&book2));

//     let max_dog_age = Dog::max_age();
//     println!("{}", max_dog_age);
//     let my_dog = Dog::new("Rex", 5);
//     println!("{:?}", my_dog);

//     println!("{}", my_dog.bark());
    
// }

// #[derive(Debug)]
// struct Dog {
//     name: String,
//     age: u32,
// }

// impl Dog {
//     fn bark(&self) -> String {
//         format!("{} says woof!", self.name)
//     }

//     fn new(name: &str, age: u32) -> Dog {
//         Dog {
//             name: name.to_string(),
//             age
//         }
//     }

//     fn max_age() -> u32 {
//         20
//     }
// }

// // fn non_repeating(s: &str) -> char {
// //     let new;
// // }



// // fn reverse_vector(v: Vec<i32>) -> Vec<i32> {
// //     let reversed: Vec<i32> = v.into_iter().rev().collect();

// //     reversed
// // }

// // fn main() {
// //     let vec = vec![1,2,3,4];

// //     println!("The reverse vector is: {:?}", reverse_vector(vec));
// // }


// // fn count_chars(s: &str) -> usize {
// //     s.len()
// // }

// // fn main() {
// //     let word = String::from("rust");

// //     println!("The number of characters in word: {}", count_chars(&word));
// // }



// // fn even_number(v: &Vec<i32>) -> Vec<i32> {
// //     let mut even_vec = Vec::new();
// //     for num in v {
// //         if num % 2 == 0 {
// //             even_vec.push(*num);
// //         }
// //     }
// //     even_vec
// // }

// // fn main() {
// //     let list = vec![1,2,3,4,5,6];

// //     println!("The even numbers in list: {:?}", even_number(&list));
// // }


// // fn main() {
// //     let rect1 = Rectangle {
// //         width: 20.00,
// //         height: 30.00,
// //     };
// //     let rect2 = Rectangle {width: 30.00, height:40.00};

// //     let square = Rectangle::square(10.00);

// //     println!("{:?}", rect1);

// //     println!("Area of Rectangle: {:?}", rect1.area());

// //     println!("The answer of holding is: {}", rect1.can_hold(&rect2));

// //     println!("The square is: {:?} ", square);
// // }

// // #[derive(Debug)]
// // struct Rectangle {
// //     width: f64,
// //     height: f64,
// // }

// // impl Rectangle {
// //     fn area(&self) -> f64 {
// //         self.width * self.height
// //     }

// //     fn can_hold(&self, other: &Rectangle) -> bool {
// //         self.width > other.width && self.height > other.height
// //     }

// //     fn square(size: f64) -> Rectangle {
// //         Rectangle { width: size, height : size}
// //     } 
// // }



// // use std::io;

// // fn main() {
// //     fizzbuzz();
    

// // }

// // fn fizzbuzz() {

// //     println!("Enter a number between 1 and 100 for print fizzbuzz");

// //     let mut input = String::new();

// //     io::stdin()
// //         .read_line(&mut input)
// //         .expect("Failed to read the input");

// //     let num = input.trim().parse().expect("Please enter a valid integer");

// //     if num < 1 || num > 100 {
// //         println!("Please enter a number between 1 and 100")
// //     };


// //     for number in 1..=num {
// //         if number % 3 == 0 && number % 5 == 0 {
// //             println!("FizzBuzz");
// //         } else if number % 3 == 0 {
// //             println!("Fizz");
// //         } else if number % 5 == 0 {
// //             println!("Buzz");
// //         } else {
// //             println!("{}", number)
// //         }
    
// // }
// // }

// // fn add(a: i32, b: i32) -> i32 {
// //     a + b
// // }

// // fn greet(name: &str) -> String {
// //     let greeting = String::from("Hello, ") + name;
// //     greeting
// // }

// // fn is_even(num: i32) -> bool {
// //     num % 2 == 0
// // }



//     // let mut your_age: i32 = 26;
//     // println!("My age is: {}", your_age);

//     // let height: f64 = 1.72;
//     // println!("My height in meters: {}", height);

//     // let name = String::from("Jitendra Singh");
//     // println!("My name is {}", name);

//     // let whether_you_like_coding = true;
//     // println!("I like coding which is {}", whether_you_like_coding);

//     // your_age = 27;
//     // println!("My age is {}", your_age);

//     // println!("The sum of 13 and 12 is {}", add(13, 12));

//     // let name = String::from("Jitendra Singh");
//     // println!("{}", greet(&name));

//     // println!("{}", is_even(5));

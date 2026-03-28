#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

fn validate_person(name: &str, age: &str, email: &str) -> Result<Person, String> {
    if name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }

    let age_parsed = age.parse::<u32>()
        .map_err(|_| "Age must be a number".to_string())?;

    if age_parsed > 150 {
        return Err("Age cannot exceed 150".to_string());
    }

    if !email.contains('@') {
        return Err("Email must contain '@'".to_string());
    }
    Ok(Person {
        name: name.to_string(),
        age: age_parsed,
        email: email.to_string(),
    })
}

fn main() {
    match validate_person("Alice", "30", "alice@example.com") {
        Ok(person) => println!("Valid person: {:?}", person),
        Err(e) => println!("Error: {}", e),
    }

    validate_person("", "30", "alice@example.com").unwrap_err();
    validate_person("Alice", "not-a-number", "alice@example.com").unwrap_err();
    validate_person("Alice", "200", "alice@example.com").unwrap_err();
    validate_person("Alice", "30", "invalid-email").unwrap_err();
}






















































// fn get_element_at_index(arr: &[i32], index: usize) -> Result<i32, String> {
//     arr.get(index)
//         .copied()
//         .ok_or_else(|| format!("Index {} out of bounds! Array length: {}", index, arr.len()))
// }

// fn main() {
//     let numbers = vec![10, 20, 30];

//     match get_element_at_index(&numbers, 1) {
//         Ok(value) => println!("Value at index 1: {}", value),
//         Err(e) => println!("Error: {}", e),
//     }

//     match get_element_at_index(&numbers, 5) {
//         Ok(value) => println!("Value at index 5: {}", value),
//         Err(e) => println!("Error: {}", e),
//     }
// }






































// use std::num::ParseIntError;

// fn parse_and_double(input: &str) -> Result<i32, ParseIntError> {
//     input
//         .parse::<i32>()
//         .map(|num| num * 2)
// }

// fn main() {
//     let result = parse_and_double("10");
//     println!("Doubled: {:?}", result);

//     let bad_result = parse_and_double("not a number");
//     println!("Bad: {:?}", bad_result);
// }


























// use std::fmt;

// #[derive(Debug)]
// enum BankError {
//     InsufficientFunds {balance: f64, requested: f64 },
//     AccountLocked,
//     InvalidAmount { amount: f64, reason: String },
// }

// impl fmt::Display for BankError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             BankError::InsufficientFunds { balance, requested } => {
//                 write!(f, "Insufficient funds! Balance: ${}, Requested: ${}", balance, requested)
//             }
//             BankError::AccountLocked => {
//                 write!(f, "Account is locked due to security concerns")
//             }
//             BankError::InvalidAmount { amount, reason } => {
//                 write!(f, "Invalid amount ${}: {}", amount, reason)
//             }
//         }
//     }
// }

// impl std::error::Error for BankError {}

// struct BankAccount {
//     owner: String,
//     balance: f64,
//     locked: bool,
// }

// impl BankAccount {
//     fn new(owner: String) -> Self {
//         BankAccount {
//             owner,
//             balance: 0.0,
//             locked: false,
//         }
//     }

//     fn deposit(&mut self, amount: f64) -> Result<(), BankError> {
//         if amount <= 0.0 {
//             return Err(BankError::InvalidAmount {
//                 amount,
//                 reason: String::from("Amount must be positive"),
//             });
//         }
//         if self.locked {
//             return Err(BankError::AccountLocked);
//         }

//         self.balance += amount;
//         println!("Deposited ${:.2}. New balance: ${:.2}", amount, self.balance);
//         Ok(())
//     }

//     fn withdraw(&mut self, amount: f64) -> Result<f64, BankError> {
//         if amount <= 0.0 {
//             return Err(BankError::InvalidAmount {
//                 amount,
//                 reason: String::from("Amount must be positive"),
//             });
//         }
//         if self.locked {
//             return Err(BankError::AccountLocked);
//         }
//         if amount > self.balance {
//             return Err(BankError::InsufficientFunds {
//                 balance: self.balance,
//                 requested: amount,
//             });
//         }

//         self.balance -= amount;
//         println!("Withdrew ${:.2}. Remaining balance: ${:.2}", amount, self.balance);
//         Ok(amount)
//     }

//     fn lock_account(&mut self) {
//         self.locked = true;
//         println!("Account locked!");
//     }
// }


// fn main() -> Result<(), BankError> {
//     let mut account = BankAccount::new(String::from("Alice"));

//     account.deposit(100.0)?;
//     account.withdraw(50.0)?;

//     match account.withdraw(200.0) {
//         Ok(_) => println!("Withdrawal successful"),
//         Err(e) => println!("Withdrawal failed: {}", e),
//     }

//     account.lock_account();
//     match account.deposit(50.0) {
//         Ok(_) => println!("Deposit successful"),
//         Err(e) => println!("Deposit failed: {}", e),
//     }

//     Ok(())

// }






















































// // use std::net::IpAddr;

// // fn main() {

// //     pub struct Guess {
// //         value: i32,
// //     }

// //     impl Guess {
// //         pub fn new(value: i32) -> Guess {
// //             if value < 1 || value > 100 {
// //                 panic!("Guess value must be between 1 and 100, got {value}.")
// //             }
// //             Guess { value }
// //         }

// //         pub fn value(&self) -> i32 {
// //             self.value
// //         }
// //     }
    
//     // let home: IpAddr = "127.0.0.1"
//     //     .parse()
//     //     .expect("Hardcoded IP address should be valid");

//     // loop {
//     //     let guess: i32 = match guess.trim().parse() {
//     //         Ok(num) => num,
//     //         Err(_) => continue,
//     //     };

//     //     if guess < 1 || guess > 100 {
//     //         println!("The secret number will be between 1 and 100.");
//     //     }

//     //     match guess.cmp(&secret_number){

//     //     }
//     // }

// // }


// // use std::fs::File;

// // fn main() -> Resutl<(), Box<dyn Error>> {
// //     let greeting_file = File::open("hello.txt")?;

// //     Ok(());
// //     // let greeting_file = File::open("hello.txt")?;
// // }

// // fn last_char_of_first_line(text: &str) -> Option<char> {
// //     text.lines().next()?.chars().last();
// // }



// // use std::fs;
// // use std::io;

// // fn main() {

// // }

// // fn read_username_from_file() -> Result<String, io::Error> {
// //     fs::read_to_string("hello.txt")
// // }


// // use std::fs::File;
// // use std::io::{self, Read};

// // fn main() {

// // }

// // fn read_username_from_file() -> Result<String, io::Error> {
// //     let mut username = String::new();

// //     File::open("hello.txt")?.read_to_string(&mut username)?;

// //     Ok(username)
// // }

// // fn read_username_from_file() -> Result<String, io::Error> {
// //     let mut username_file = File::open("hello.txt")?;
// //     let mut username = String::new();
// //     username_file.read_to_string(&mut username)?;
// //     Ok(username);
// // }

// // fn read_username_from_file() -> Result<String, io::Error> {
// //     let username_file_result = File::open("hello.txt");

// //     let mut username_file = match username_file_result {
// //         Ok(file) => file,
// //         Err(e) => return Err(e),
// //     };

// //     let mut username = String::new();

// //     match username_file.read_to_string(&mut username) {
// //         Ok(_) => Ok(username),
// //         Err(e) => Err(e),
// //     }
// // }



// // use std::io::ErrorKind;

// // fn main() {
// //     let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");
// //     // let greeting_file = File::open("hello.txt").unwrap();
// //     // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
// //     //     if error.kind() == ErrorKind::NotFound {
// //     //         File::create("hello.txt").unwrap_or_else(|error| {
// //     //             panic!("Problem creating the file: {error:?};")
// //     //         })
// //     //     } else {
// //     //         panic!("Problem opening the file: {error:?}");
// //     //     }
// //     // });

// //     // let greeting_file_result = File::open("hello.txt");

// //     // let greeting_file = match greeting_file_result {
// //     //     Ok(file) => file,
// //     //     Err(error) => match error.kind() {
// //     //         ErrorKind::NotFound => match File::create("hello.txt") {
// //     //             Ok(fc) => fc,
// //     //             Err(e) => panic!("Problem creating the file: {e:?}"),
// //     //         },
// //     //         _=> {
// //     //             panic!("Problem opening the file: {error:?}");
// //     //         }
// //     //     },
// //     // };
// // }


// // fn main() {
// //     let v = vec![1,2,3];

// //     v[99];
// // }

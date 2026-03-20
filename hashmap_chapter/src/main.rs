use std::collections::HashMap;

fn main() {
    let mut grades = HashMap::new();

    grades.insert(String::from("Alice"), 95);
    grades.insert(String::from("Bob"), 87);
    grades.insert(String::from("Charlie"), 92);

    let alice_grade = grades.get("Alice");
    match alice_grade {
        Some(grade) => println!("Alice scored: {}", grade),
        None => println!("Alice not found!"),
    }

    let david_grade = grades.get("David").copied().unwrap_or(0);
    println!("David's grade: {}", david_grade);

    println!("\nAll grades:");
    for (student, grade) in grades {
        println!("{}: {}", student, grade);
    }
    // println!("{alice_grade:?}");
    // let mut fruit_stand = HashMap::new();

    // fruit_stand.insert(String::from("apple"), 5);
    // fruit_stand.insert(String::from("banana"), 3);
    // fruit_stand.insert(String::from("orange"), 7);

    // println!("Fruit stand: {:?}", fruit_stand);

    // fruit_stand.insert(String::from("apple"), 8);

    // println!("After restock: {:?}", fruit_stand);
    
    // let mut toy_cabinet: HashMap<String, i32> = HashMap::new();

    // let teams = vec![
    //     String::from("Blue"),
    //     String::from("Red"),
    //     String::from("Yellow"),
    // ];
    // let scores = vec![10, 25, 15];

    // let team_scores: HashMap<_,_> = teams.iter().zip(scores.iter()).collect();
    // println!("{:?}", team_scores);

}


// use std::collections::HashMap;
// use std::io::{self, Write};

// fn main() {
//     let mut company: HashMap<String, Vec<String>> = HashMap::new();

//     loop {
//         println!("\n--- Employee Management System ---");
//         println!("Commands:");
//         println!(" Add [name] to [department] ");
//         println!(" List [department]");
//         println!(" List all");
//         println!(" quit");
//         print!["\nEnter command: "];
//         io::stdout().flush().unwrap();

//         let mut input = String::new();

//         io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read line");

//         let input = input.trim();

//         if input == "quit" {
//             break;
//         }

//         process_command(&mut company, input);
//     }
// }

// fn process_command(company: &mut HashMap<String, Vec<String>>, command: &str) {
//     let parts: Vec<&str> = command.split_whitespace().collect();

//     match parts.as_slice() {
//         ["Add", name, "to", department] => {
//             let employees = company
//                 .entry(department.to_string())
//                 .or_insert_with(Vec::new);

//             employees.push(name.to_string());
//             employees.sort();
//             println!("Added {} to {}", name, department);
//         },

//         ["List", department] => {
//             match company.get(*department) {
//                 Some(employees) => {
//                     println!("\n{} department:", department);
//                     for employee in employees {
//                         println!(" - {}", employee);
//                     }
//                 }, 
//                 None => println!("Department '{}' not found", department),
//             }
//         },

//         ["List", "all"] => {
//             if company.is_empty() {
//                 println!("No employees found");
//                 return;
//             }

//             let mut departments: Vec<&String> = company.keys().collect();
//             departments.sort();

//             for dept in departments {
//                 println!("\n{}:", dept);
//                 let employees = company.get(dept).unwrap();
//                 for employee in employees {
//                     println!(" - {}", employee);
//                 }
//             }
//         },
//         _ => println!("Invalid command. Use: Add [name] to [department], List [department], List all, or quit"),
//     }
// }


// use std::collections::HashMap;
// use std::io;

// // Exercise chapter - 8
// fn main() {
//     // First Problem
//     let mut integers = vec![1,2,3,4,5,5,4,4,3];
    
//     integers.sort();

//     let middle_number = integers[integers.len() / 2];

//     println!("{middle_number}");
//     println!("{integers:?}");
//     let mut mode = HashMap::new();

//     for &num in &integers {
//         let count = mode.entry(num).or_insert(0);
//         *count += 1;
//     }
//     println!("{mode:?}");

//     let mut mode2 = integers[0];
//     let mut max_count = 0;

//     for(&num, &count) in &mode {
//         if count > max_count {
//             max_count = count;
//             mode2 = num;
//         }
//     }
//     println!("Mode: {} (appears {} times)", mode2, max_count);

//     //Second Problem
//     println!("{}", to_pig_latin("apple"));
//     println!("{}", to_pig_latin("first"));
//     println!("{}", to_pig_latin("Eagle"));
//     println!("{}", to_pig_latin("word"));

//     // let word = "apple";
//     // let first_char = word.chars().next().unwrap();

//     // match first_char {
//     //     'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U'  => println!("{}-hay", word),
//     //     _ => println!("{}-{}ay", &word[1..], &word[0..1]),
//     // }
//     // println!("{word}");

//     // Third Problem
//     let mut company: HashMap<String, Vec<String>> = HashMap::new();

//     company.insert(String::from("Engineering"), vec![String::from("Sally")]);

//     company.entry(String::from("Sales"))
//             .or_insert_with(Vec::new)
//             .push(String::from("Amir"));

//     println!("{:?}", company);
// } 

// fn to_pig_latin(word: &str) -> String {
//     let first_char = word.chars().next().unwrap();

//     match first_char {
//         'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U' => {
//             format!("{}-hay", word)
//         }
//         _ => {
//             format!("{}-{}ay", &word[1..], &word[0..1])
//         }
//     }
// }

// fn main() {

    // let text = "hello world wonderful world";

    // let mut map = HashMap::new();

    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    //     println!("{count}");
    // }

    // println!("{map:?}");

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);
    // println!("{scores:?}");
    // scores.insert(String::from("Blue"), 25);
    // println!("{scores:?}")

    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");

    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);
    // println!("{:?}", map);
    
    // let mut scores = HashMap::new();
    
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    // println!("{:?}", scores);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name).copied().unwrap_or(0);
    // println!("{}", team_name);
    // println!("{}", score);

    // for (key, value) in &scores {
    //     println!("{key}: {value}");
    // } 

    // println!("Hello, world!");
// }

fn is_palindrome(s: &str) -> bool {
    // let reverse_vec = vec![s.chars().rev()];

    // let reverse_word: String = reverse_vec.into_iter().collect();

    if s[0] == s.rfind() {
        true
    } else {
        false
    }
}

fn main() {}





// fn remove_negative_numbers(v: &[i32]) -> Vec<&i32> {
//     let mut positive_vec = Vec::new();

//     for num in v {
//         if *num >= 0 {
//             positive_vec.push(num);
//         }
//     }
//     positive_vec
// }

// fn main() {
//     let vec = [3, -1, 5, -7, 9];

//     println!("{:?}", remove_negative_numbers(&vec));
// }


// use std::collections::HashMap;
// // 1. It should take a word;
// // 2. Count all the different characters and keep track of them

// fn main() {
//     let mut character_frequency = HashMap::new();

//     let word = String::from("banana");

//     for chars in word.chars() {
//         let count = character_frequency.entry(chars).or_insert(0);
//         *count += 1;
//     }

//     println!("{:?}", character_frequency);
// }

// fn character_frequency(s: &str) -> HashMap::new() {
    
// }






















// fn longest_word(s: &str) -> &str {
//     let longest: Vec<&str> = s.split_whitespace().collect();
//     let mut longest_word = longest[0];
    
//     for word in longest {
//         if word.chars().count() > longest_word.chars().count() {
//             longest_word = word;
//         };

//     }
//     longest_word
// }

// fn main() {
//     let sentence = String::from("rust makes systems programming safe");

//     println!("{}", longest_word(&sentence));
// }







// fn sum_even(v: &[i32]) -> i32 {

//     let mut sum = 0;

//     for num in v {
//         if num % 2 == 0 {
//             sum += num;
//         }
//     }
//     sum
// }

// fn main () {
//     let vec = vec![1,2,3,4,5,6];

//     println!("{}", sum_even(&vec));
// }




































































































// fn main() {
//     println!("Hello, world!");
// }

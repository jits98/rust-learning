fn main() {

    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();

    println!("the first word is: {word}");
    // let s = String::from("hello world");

    // let hello = &s[0..5];
    // let world = &s[6..11];

    // let s = String::from("hello");

    // let len = s.len();

    // let slice = &s[0..len];
    // let slice = &s[..];

    // let s = String::from("hello");

    // let slice = &s[0..2];
    // let slice = &s[..2];

    // let s = String::from("hello");
    // let len = s.len();

    // let slice = &s[3..len];
    // let slice = &s[3..];

    // let mut s = String::from("hello world");

    // let word = first_word(&s);

    // s.clear();
    // println!("{}", word);
    
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

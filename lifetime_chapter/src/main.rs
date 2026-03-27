// #[derive(Debug)]
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().unwrap();
//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };

//     println!("{:?}", i);
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {}



// fn main() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");

//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is {result}");


//     // let x = 5;

//     // let r = &x;
//     // &i32;
//     // &'a i32;
//     // &'a mut i32;

//     // // {
//     // //     let x = 5;
//     // //     r = &x;
//     // // }

//     // println!("r: {r}");
// }

// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
//     // x
//     // if x.len() > y.len() { x } else { y }
// }

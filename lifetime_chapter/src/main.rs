fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {result}");

    // let x = 5;

    // let r = &x;
    // &i32;
    // &'a i32;
    // &'a mut i32;

    // // {
    // //     let x = 5;
    // //     r = &x;
    // // }

    // println!("r: {r}");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    // let x = 5;

    // let r = &x;

    // // {
    // //     let x = 5;
    // //     r = &x;
    // // }

    // println!("r: {r}");
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

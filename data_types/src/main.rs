use std::io;

fn main() {
    let a = [1,2,3,4,5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}")
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
}



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
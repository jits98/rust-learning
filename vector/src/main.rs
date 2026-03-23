fn main() {
    let vector = vec![1,2,2,3,4,4,5];

    println!("{:?}", remove_duplicates(vector));
}

fn remove_duplicates(v: Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for num in v.iter().enumerate() {
        if num != v {
            new_vec.push(num);
        }
    }
    return new_vec;
}


// fn main() {
//     let vector = vec![34, 50, 25, 100, 65];
//     println!("{}", largest(&vector));

// }

// fn largest(v: &Vec<i32>) -> i32 {
//     let mut largest = 0;

//     for num in v {
//         if *num > largest {
//             largest = *num;
//         }
//     }
//     return largest;
// }

// fn main() {
//     let vector = vec![5,10,15,20];
//     println!("{:?}", sum_of_vector(&vector));

    
// }

// fn sum_of_vector(v: &Vec<i32>) -> i32 {
//     let mut sum = 0;
//     for num in v {
//         sum += num;
//     };
//     return sum;
// }

// fn main() {
//     let numbers = vec![1,2,3,4,5];

//     for num in &numbers {
//         println!("{num}");
//     }
// }

    // let classroom_grades = vec![85, 90, 78, 92, 88];

    // println!("All grades:");
    // for grade in &classroom_grades {
    //     println!("Student scored: {}", grade);
    // }

    // let mut allowances = vec![5, 10, 15];
    // for money in &mut allowances {
    //     *money += 2;
    //     println!("New allowance: {}", money);
    // }



    // let fruit_basket = vec!["apple", "banana", "orange", "grape"];

    // let first_fruit = &fruit_basket[0];
    // println!("First fruit: {}", first_fruit);

    // let second_fruit = fruit_basket.get(1);
    // match second_fruit {
    //     Some(fruit) => println!("Second fruit: {}", fruit),
    //     None => println!("No fruit at that position"),
    // }

    // let tenth_fruit = fruit_basket.get(9);
    // println!("Tenth fruit: {:?}", tenth_fruit);


    // let empty_backpack: Vec<i32> = Vec::new();

    // let toy_cars = vec!["red car", "blue car", "yellow car"];

    // let scores = vec![95, 87, 92,100];

    // let mut pocket = Vec::new();
    // pocket.push("marble");
    // pocket.push("coin");
    // pocket.push("key");

    // println!("My pocket has: {:?}", pocket);

    // let mut candy_jar = vec!["chocolate", "lollipop", "gum"];
    // let last_candy = candy_jar.pop();

    // println!("I ate: {:?}", last_candy);
    // println!("Left in jar: {:?}", candy_jar);
// }


// fn main() {
//     // let v: Vec<i32> = Vec::new();
//     // let v = vec![1,2,3];

//     //Updating a Vector
//     // let mut v = Vec::new();

//     // v.push(5);
//     // v.push(6);
//     // v.push(7);
//     // v.push(8);

//     // println!("{:?}", v);

//     // let v2 = vec![1,2,3,4,5];

//     // let third: &i32 = &v2[2];
//     // println!("The third element is {third}");

//     // let third: Option<&i32> = v2.get(2);
//     // match third {
//     //     Some(third) => println!("The third element is {third}"),
//     //     None => println!("There is no third element."),
//     // }

//     // let v = vec![1,2,3,4,5];
//     // let does_not_exist = &v[100];
//     // println!("{}", does_not_exist);
//     // let does_not_exist = v.get(100);
//     // println!("{:?}", does_not_exist);

//     // let mut v = vec![1,2,3,4,5];

//     // let first = &v[0];

//     // v.push(6);

//     // println!("The first element is: {first}");

//     // let v = vec![100,32,57];
//     // for i in &v {
//     //     println!("{i}");
//     // }

//     // let mut v = vec![100,32,57];
//     // for i in &mut v {
//     //     *i += 50;
//     //     println!("{i}");
//     // }

//     // enum SpreadsheetCell {
//     //     Int(i32),
//     //     Float(f64),
//     //     Text(String),
//     // }

//     // let row = vec![
//     //     SpreadsheetCell::Int(3),
//     //     SpreadsheeCell::Text(String::from("blue")),
//     //     SpreadsheetCell::Float(10.12);

//     // ]

    

// }

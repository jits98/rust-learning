fn main() {
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1,2,3];

    //Updating a Vector
    // let mut v = Vec::new();

    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);

    // println!("{:?}", v);

    // let v2 = vec![1,2,3,4,5];

    // let third: &i32 = &v2[2];
    // println!("The third element is {third}");

    // let third: Option<&i32> = v2.get(2);
    // match third {
    //     Some(third) => println!("The third element is {third}"),
    //     None => println!("There is no third element."),
    // }

    // let v = vec![1,2,3,4,5];
    // let does_not_exist = &v[100];
    // println!("{}", does_not_exist);
    // let does_not_exist = v.get(100);
    // println!("{:?}", does_not_exist);

    // let mut v = vec![1,2,3,4,5];

    // let first = &v[0];

    // v.push(6);

    // println!("The first element is: {first}");

    // let v = vec![100,32,57];
    // for i in &v {
    //     println!("{i}");
    // }

    let mut v = vec![100,32,57];
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

}

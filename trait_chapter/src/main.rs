fn main() {
    let s = 3.to_string();
    println!("{}", s);
}











// use std::fmt::Display;

// struct Pair<T> {
//     x: T,
//     y: T,
// }

// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self { x, y }
//     }
// }

// impl<T: Display + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {}", self.x);
//         } else {
//             println!("The largest member is y = {}", self.y);
//         }
//     }
// }



// mod aggregator;

// use aggregator::{NewsArticle, SocialPost, Summary};

// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// // fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// // fn some_function<T, U> (t: &T, u: &U) -> i32 where T: Display + Clone, U: Clone + Debug, {}

// // pub fn notify(item: &(impl Summary + Display)) {}
// // pub fn notify<T: Summary + Display>(item: &T) {}
// // pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
// // pub fn notify<T: Summary>(item1: &T, item2: &T) {}

// fn main() {
//     let post = SocialPost {
//         username: String::from("horse_ebooks"),
//         content: String::from(
//             "of course, as you probably already know, people."
//         ),
//         reply: false,
//         repost: false,
//     };

//     println!("1 new post: {}", post.summarize());

//     let article = NewsArticle {
//         headline: String::from("Penguins win the Stanley Cup Championship!"),
//         location: String::from("Pittsburgh, PA, USA"),
//         author: String::from("Iceburgh"),
//         content: String::from(
//             "The Pittsburgh Penguins once again are the best  
//             hockey team in the NHL."),
//     };

//     println!("New article available! {}", article.summarize());

//     fn returns_summarizable(switch: bool) -> impl Summary {
//         if switch {
//             NewsArticle {
//                 headline: String::from(
//                     "Penguins win the Stanley Cup Championship!",
//                 ),
//                 location: String::from("Pittsburgh, PA, USA"),
//                 author: String::from("Iceburgh"),
//                 content: String::from(
//                     "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
//                 ),
//             }
//         } else {   
//             SocialPost {
//                 username: String::from("horse_ebooks"),
//                 content: String::from(
//                     "of course, as you probably already know, people",
//                 ),
//                 reply: false,
//                 repost: false,
//             }
//         }
//       }
// }


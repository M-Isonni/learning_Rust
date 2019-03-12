use core::fmt::Debug;
use std::fmt::Display;


pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_overload<T: Summary>(item1: T, item2: T) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

pub fn notify_adding_traits<T: Summary + Debug>(item: T) {
    notify(item);
}

fn some_function<T, U>(t: T, u: U) -> i32
    where T: Summary + Clone,
          U: Clone + Debug
{
    let s = t.clone();
    let r = u.clone();
    5
}

//returning traits
//following function can return any type that implement the specified trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

//functions are checked at compile time, so the trait returning only works
//if it always return the same type, the following function won't be compiled
// fn returns_summarizable_different_types(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from("The Pittsburgh Penguins once again are the best
//             hockey team in the NHL."),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


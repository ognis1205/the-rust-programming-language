use std::fmt::Display;

fn main() {
    let x = 5;
    let r = &x;
    println!("r: {}, {}", r, *r);

    let l = String::from("abcd");
    let r = "xyz";
    let res = longest(l.as_str(), r);
    println!("res: {}", res);

    let l = String::from("abcd");
    let r = "xyz";

    let res = longest_with_an_announcement(l.as_str(), r, "Test");
    println!("The longest string is {}", res);
}

fn longest<'a>(lhs: &'a str, rhs: &'a str) -> &'a str {
    if lhs.len() > rhs.len() {
        return lhs;
    } else {
        return rhs;
    }
}

fn longest_with_an_announcement<'a, T>(lhs: &'a str, rhs: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if lhs.len() > rhs.len() {
        return lhs;
    } else {
        return rhs;
    }
}

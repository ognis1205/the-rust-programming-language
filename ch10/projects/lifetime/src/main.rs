fn main() {
    let x = 5;
    let r = &x;
    println!("r: {}, {}", r, *r);

    let l = String::from("abcd");
    let r = "xyz";
    let res = longest(l.as_str(), r);
    println!("res: {}", res);
}

fn longest<'a>(lhs: &'a str, rhs: &'a str) -> &'a str {
    if lhs.len() > rhs.len() {
        return lhs;
    } else {
        return rhs;
    }
}

trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

#[derive(Debug)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("headline"),
        location: String::from("tokyo"),
        author: String::from("ognis1205"),
        content: String::from("content"),
    };
    let tweet = Tweet {
        username: String::from("ognis1205"),
        content: String::from("content"),
        reply: false,
        retweet: false,
    };
    println!("{:?}", article);
    println!("{:?}", tweet);
    println!("article: {}", article.summarize());
    println!("tweet: {}", tweet.summarize());
}

use trait_01::{Summary, NewsArticle};

fn main() {
    // println!("Hello, world!");
    let article = NewsArticle{
        headline: String::from("Penguins win the Stanley championship"),
        author: String::from("Satardekar"),
        content: String::from("The Pittsburgh Penguins once again are the best \
             hockey team in the NHL."),
        location: String::from("Mumbai, MU, MH"),
    };
    println!("News article available: {}", article.summarize());
}

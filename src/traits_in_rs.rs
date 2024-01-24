pub fn traits_in_rs() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summary());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summary());

    // Traits as Parameters
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summary());
    }

    // # Traits Bound Syntax
    pub fn notify_bound<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summary());
    }
}

// #Defining a Trait

// Here, we declare a trait using the trait keyword and
// then the trait’s name, which is Summary
// in this case. We’ve also declared the trait as pub so
// that crates depending on this crate can
//make use of this trait too.

// Inside the curly brackets, we declare
//the method signatures that describe the behaviors
// of the types that implement this trait, which
// in this case is fn summarize(&self) -> String

pub trait Summary {
    fn summary(&self) -> String {
        String::from("(Read More...)")
    }
}

// #Implementing a Trait on a Type

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summary(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

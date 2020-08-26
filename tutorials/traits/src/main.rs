// Trait（トレイト）は他の言語でよくインターフェースと呼ばれる機能に類似している。
trait Summary {
  fn summarize_author(&self) -> String;
  fn summarize(&self) -> String {
    format!("(Read more from {}...)", self.summarize_author())
  }
}

struct NewsArticle {
  headline: String,
  location: String,
  author: String,
  content: String,
}
impl Summary for NewsArticle {
  fn summarize_author(&self) -> String {
    format!("{} ({})", self.author, self.location)
  }
  // オーバーライドしなければ、トレイトで定義された内容が実行される.
  // fn summarize(&self) -> String {
  //   format!("{}, by {}", self.headline, self.summarize_author())
  // }
}

struct Tweet {
  username: String,
  content: String,
  reply: bool,
  retweet: bool,
}
impl Summary for Tweet {
  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
  fn summarize(&self) -> String {
    format!("{}: {}", self.summarize_author(), self.content)
  }
}

fn main() {
  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  };
  println!("1 new tweet: {}", tweet.summarize());

  let news_article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
  };
  println!("New article available! {}", news_article.summarize());
}

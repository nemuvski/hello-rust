// クリスマスキャロルの定番、"The Twelve Days of Christmas"の歌詞を、 曲の反復性を利用して出力する。
// https://doc.rust-jp.rs/book/second-edition/ch03-05-control-flow.html

fn main() {
  for day in 1..13 {
    println!("On the first day of Christmas my true love sent to me");

    if day == 12 {
      println!("Twelve drummers drumming");
    }
    if day >= 11 {
      println!("Eleven pipers piping");
    }
    if day >= 10 {
      println!("Ten lords a-leaping");
    }
    if day >= 9 {
      println!("Nine ladies dancing");
    }
    if day >= 8 {
      println!("Eight maids a-milking");
    }
    if day >= 7 {
      println!("Seven swans a-swimming");
    }
    if day >= 6 {
      println!("Six geese a-laying");
    }
    if day >= 5 {
      println!("Five golden rings");
    }
    if day >= 4 {
      println!("Four calling birds");
    }
    if day >= 3 {
      println!("Three French hens");
    }
    if day >= 2 {
      println!("Two turtle doves");
    }

    if day == 1 {
      println!("A partridge in a pear tree");
    } else {
      println!("And a partridge in a pear tree");
    }

    println!("");
  }
}

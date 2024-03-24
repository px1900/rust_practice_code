use std::cmp::PartialOrd;
use std::fmt::{self, Display, Debug};
mod aggregation;
use crate::aggregation::{Summary, Tweet, NewsArticle};

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl fmt::Display for Point<f32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x={}, y={}", self.x, self.y)
    }
}

impl Point<f32> {
    fn display(&self) {
        println!("{}", self);
    }
}

impl fmt::Display for Point<i32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x={}, y={}", self.x, self.y)
    }
}
impl Point<i32> {
    fn display(&self) {
        println!("{}", self);
    }
}

impl<T> Point<T> {
    fn get_y(&self) -> &T {
        &self.y
    }
}

impl<T: Display+PartialOrd> Point<T> {
    fn get_max(&self) -> &T {
        if self.x > self.y {
            &self.x
        } else {
            &self.y
        }
    }
}



struct NewPoint<T1, T2> {
    x: T1,
    y: T2,
}

impl<T1, T2> NewPoint<T1, T2> {
    fn mixup<Y1, Y2> (self, other: NewPoint<Y1, Y2>) -> NewPoint<T1, Y2> {
        NewPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // To test the largest() function
    let number_list = vec![1, 2, 10, 3, 5];

    let largest_num = largest(&number_list);
    println!("largest number is {largest_num}");

    let char_list = vec!['a', 'e', 'z', 'b'];

    let largest_char = largest(&char_list);
    println!("largest char is {largest_char}");

    // To test struct Point
    let integer_point = Point { x:5, y:10 };
    let float_point = Point { x:1.0, y:2.0 };

    integer_point.display();
    float_point.display();
    println!("y = {}", float_point.get_y() );
    println!("y = {}", integer_point.get_y() );
    println!("max = {}", integer_point.get_max() );

    let new_point1 = NewPoint{
        x: 1,
        y: 2.5,
    };

    let new_point2 = NewPoint{
        x: 'a',
        y: "abcd",
    };

    let new_point3 = new_point1.mixup(new_point2);

    println!("new ponit3 x={}, y={}", new_point3.x, new_point3.y);

    // Trait
    let tweet = Tweet{
        username: String::from("uname"),
        content: String::from("this is conent"),
        reply: false,
        retweet: false,
    };

    println!("tweet's summary is {}", tweet.summarize());

    let news_article = NewsArticle{
        headline: String::from("This is headline"),
        location: String::from("This is location"),
        author: String::from("this is author"),
        content: String::from("this is content"),
    };


    println!("news's summary is {}", news_article.summarize());

    println!("summary is {}", news_article.summarize_with_time());
        
    
    notify(&news_article);

    notify(&returns_summarizable())
}

fn notify(item: &impl Summary) {
    println!("breaking news: {}", item.summarize());
}

fn returns_summarizable() -> impl Summary {
    Tweet{
        username: String::from("uname"),
        content: String::from("this is content"),
        reply: false,
        retweet: false,
    }
}

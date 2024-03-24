use std::collections::HashMap;

fn main() {

    // Vector Related
    let mut v: Vec<i32> = Vec::new();
    v.push(1);

    let v_ele: Option<&i32> = v.get(2);
    match v_ele {
        Some(ele) => println!("third element is {ele}\n"),
        None => println!("Don't have third element\n"),
    }

    let mut v2 = vec![1,2,3,4,5];

    let v_ele: Option<&i32> = v2.get(2);
    match v_ele {
        Some(ele) => println!("third element is {ele}\n"),
        None => println!("Don't have third element\n"),
    }

    let v_ele: &i32 = &v2[2];
    println!("element is {v_ele}\n");

    for i in &mut v2 {
        *i += 50;
    }

    for i in &v2 {
        println!("iterate ele {i}\n");
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let rows = vec![
        SpreadsheetCell::Int(5),
        SpreadsheetCell::Float(3.21),
        SpreadsheetCell::Text(String::from("abcd")),
    ];
    println!("rows size = {}\n", rows.len());

    // String Related
    let data = "initial content";
    let s: String = data.to_string();
    println!("s = {s}");
    let mut s = String::from("abc");
    s.push_str(" def");
    println!("S = {s}");

    s.push('g');
    s.push('h');
    println!("S = {s}");


    let s1 = String::from("s1");
    let s2 = String::from("s2");
    let s3 = String::from("s3");

    // The s1+&s2 will in fact call add(self, s: &str) -> String;
    let s4 = s1+&s2;
    let s5 = s4+"*";
    let s6 = s5+&s3;
    println!("s6={s6}");

    let s1 = String::from("s1");
    let s2 = String::from("s2");
    let s3 = String::from("s3");
    let s6 = format!("{s1}-{s2}-{s3}\n");
    println!("s6 = {s6}");

    for c in s6.chars() {
        println!("c = {c} ");
    }
    
    for c in s6.bytes() {
        println!("c = {c} ");
    }

    // HashMap Related
    
    // All the key in one hashmap should be same type, same thing for value
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 20);

    let score = scores.get(&String::from("blue--")).copied().unwrap_or(-1);
    println!("score = {score}\n");

    for (key, value) in &scores {
        println!("{key} - {value}");
    }


    scores.entry(String::from("blue")).or_insert(50);
    scores.entry(String::from("blue--")).or_insert(50);

    println!("{:?}", scores);
    

    let text = "aa bb cc dd a b aa bb";

    let mut word_map = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", word_map);
}

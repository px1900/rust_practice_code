fn give_ownership() -> String {
    String::from("func:give_ownership")
}

fn take_and_return_ownership(mut s: String) -> String {
    println!("get the ownership of string: {s}\n");
    s.push_str(" (take_and_return_ownership)");
    s
}

// Only have the reference, can't modify it
fn borrow_ownership(s: &String) -> usize {
    s.len()
}

fn borrow_mutable_ownership(s: &mut String) -> usize{
    s.push_str("efgh");
    s.len()
}

fn main() {
    let a = String::from("abc");
    let b = a;
    // println!("a = {a}, b = {b}\n");
    //For now, a is invalid
    println!("b = {b}\n");


    // With clone(), b copy a's content in heap
    let a = String::from("abcd");
    let b = a.clone();
    println!("a = {a}, b = {b}\n");


    // for the simple data type, for example, integer, float, char, tuples,
    // they implement the Copy trait, so we don't need the clone() when assigning them
    let a: u32 = 3;
    let b: u32 = a;
    println!("a = {a}, b = {b}\n");


    // Get ownership given by function
    let s = give_ownership();
    println!("get ownership: {s}\n");
    // Give s to function and get another back
    let s2 = take_and_return_ownership(s);
    println!("new content = {s2}\n");


    let mut s = String::from("abcd");
    // Give s' reference to function, we still own variable s
    let s_len = borrow_ownership(&s);
    println!("s = {s}, len = {s_len}\n");

    // Give s's mutable reference to function
    let s_len = borrow_mutable_ownership(&mut s);
    println!("s = {s}, len = {s_len}\n");

    let mut s = String::from("abcd");
    let s1 = &s; // reader1: unmutable reference
    let s2 = &s; // reader2: the second unmutable reference
    // let s3 = &mut s; // writer1: mutable reference. There would have a problem
    println!("s = {s}, s1 = {s1}, s2 = {s2}\n");
    s.push_str("efgh");


    let mut s = String::from("abcd");
    let s1 = &s; // reader1: unmutable reference
    let s2 = &s; // reader2: the second unmutable reference
    println!("s = {s}, s1 = {s1}, s2 = {s2}\n");
    // Here, s1 and s2 will be released
    let s3 =  &mut s;
    s3.push_str("efgh");
    // Here, can't use s and s3 at the same time: two writers
    println!("s3 = {s3}\n");
}

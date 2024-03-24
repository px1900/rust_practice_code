// In this function, the lifetime is necessary. The reason is that RUST try to inspect
// whether the function is safe or not by checking whether the return value will be a
// hanging pointer. If we don't assign the lifetime, the return value will have same
// lifetime with x or y. It hard to inspect whether the return value will become a hanging
// pointer. If rust just assume the return value will have same lifetime with the shorter
// lifetime's value, then it will hinder the programmer's logic. By explicitly
// demonstrating each variable's lifetime, it would be easier for RUST to check hanging
// pointer, and also make programming easier. 

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime in the struct
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part : &'a str,
}

#[derive(Debug)]
struct NumStruct {
    num : i32,
}

fn main() {
    
    // Longest function
    let string1 = String::from("abcdefkl");
    let string2 = String::from("efgh");

    let result = longest(&string1, &string2);
    println!("the longest string is {result}");

    // Struct Lifetime
    let mut i = ImportantExcerpt {part: "initial"};
    {
        let novel = String::from("this is first sentence. this is the second one");
        let first_sentence = novel.split('.').next().expect("Can't find a '.'");
        i.part = first_sentence;
        println!("{:?}", i);
    }


    let mut ms = NumStruct { num:1 };
    {
        ms.num = 5;
    }
    println!("{:?}", ms);
        

}

fn main() {
    println!("Hello, world!");
    let x:u32 = 5;
    let y:u32 = 6;
    println!("result = {}\n", add_number(x, y));

    let small: u32 = 1;
    let great: u32 = 2;

    if small < great {
        print!("{small} < {great}");
    } else {
        print!("{small} > {great}");
    }

    let num :u32 = if true {32} else {33};
    println!("num = {num}\n");

    // loop and break-return
    let mut count = 0;

    let result = loop {
        count = count + 1;

        if count == 10 {
            break count * 2;
        }
    };

    println!("loop result = {result}\n");

    // Tag multi loops
    let mut count :u32 = 0;
    'count_up: loop {
        println!{"count = {count}\n"}

        let mut remaining: u32 = 10;

        loop {
            println!{"remaining = {remaining}\n"};
            if remaining == 9{
                break;
            }
            if count == 2 {
                break 'count_up;
            }
            remaining-=1;
        }
        count+=1;
    }

    println!("The ending count = {count}\n");

    // while loop
    let mut count: u32 = 0;
    while count <= 1 {
        println!("count = {count}\n");
        count += 1;
    }

    let temp_list = [1, 2, 3, 4];
    for ele in temp_list{
        println!("ele = {ele}\n");
    }

    for i in (1..=4).rev(){
        println!("iterate i = {i}\n");
    }




}

fn add_number(first:u32, second:u32) -> u32 {
    first+second
}

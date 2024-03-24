mod front_of_house;

use crate::front_of_house::{hosting};



fn main() {
    hosting::add_to_waitlist();
    front_of_house::waitlist();
    let res: usize = math_work::add(1,2);
    println!("usize = {res}\n");
}

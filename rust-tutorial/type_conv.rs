use std::convert::From;

#[derive(Debug)]

struct Number {
    value: i32
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number {value: item}
    }
}

fn main() {
    let num: i32 = 56;
    let another_num: Number = num.into();
    println!("The number is : {:?}", another_num);
    println!("The number from i32 is {:?}", Number::from(num));
}

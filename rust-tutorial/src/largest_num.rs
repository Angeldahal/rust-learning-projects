fn largest<T>(list: &[T]) -> &T {
    let mut largest: i64 = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![1, 2, 3, 4, 4, 2, 4, 2];

    let result = largest(&number_list);
    println!("The largest number is {}.", result);

    let char_list = vec!['t', 't', 'j', 'i'];
    let result = larget(&char_list);
    println!("The largest character is {}.", result);
}

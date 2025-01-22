#[derive(Debug)]

struct Point(f32, f32);

fn calculate_area(point1: &Point, point2: &Point) -> f32{
    let Point(x1, y1) = *point1;
    let Point(x2, y2) = *point2;
    let length = (y2 - y1);
    let width = (x2 - x1);
    length * width
}

fn square(point1: &Point, offset: f32) -> Point {
    let Point(x1, y1) = *point1;
    let second_coords = Point(x1 + offset, y1 + offset);
    second_coords
}

fn main() {
    let pt1 = Point(1.0, 1.0);
    let pt2 = Point(0.0, 0.0);
    println!("The area is {}", calculate_area(&pt1, &pt2));
    let second_coordinates: Point = square(&pt2, 4.0);
    println!("The second coordinates of the rectangle is {:?}", square(&pt2, 4.0));
    println!("The area of the new rectangle is {}", calculate_area(&pt2, &second_coordinates));

}

use std::fmt;

struct Structure ((f32, f32), (f32, f32));

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {} \n{}, {}\n", self.0.0, self.0.1, self.1.0, self.1.1)
    }
}

fn transpose(matrix: Structure) -> Structure{
    let Structure((x1, y1), (x2, y2)) = matrix;
    Structure((x1, x2), (y1, y2))
}

fn main() {
    let matrix = Structure((1.0, 2.2), (3.3, 4.5));
    println!("Compare Structures: ");
    println!("Display: \n{}", matrix);
    println!("Reversed: \n{}", transpose(matrix));
}



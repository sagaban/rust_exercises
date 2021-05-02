#[allow(dead_code)]
pub fn run() {

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn dist_from_origin(&self) -> f64 {
        let sum_of_squares = self.x.pow(2) + self.y.pow(2);
        (sum_of_squares as f64).sqrt()
    }

      // fn translate(&mut self, dx: i32, dy: i32) {
      //     self.x += dx;
      //     self.y += dy;
      // }
}

let p1 = Point { x: 24, y: 42 };

fn print_point(point: &Point) {
    println!("x: {}, y: {}", point.x, point.y);
}

print_point(&p1);

let dist = p1.dist_from_origin();

println!("The distance from origin is {}", dist);

enum Expr {
    Null,
    Add(i32, i32),
    Sub(i32, i32),
    Mul(i32, i32),
    Div { dividend: i32, divisor: i32 },
    Val(i32),
}

let quotient = Expr::Div {
    dividend: 10,
    divisor: 2,
};
let sum = Expr::Add(40, 2);

fn print_expr(expr: Expr) {
    match expr {
        Expr::Null => println!("No value"),
        Expr::Add(x, y) => println!("{}", x + y),
        Expr::Sub(x, y) => println!("{}", x - y),
        Expr::Mul(x, y) => println!("{}", x * y),
        Expr::Div {
            dividend: _x,
            divisor: 0,
        } => println!("Divisor is zero"),
        Expr::Div {
            dividend: x,
            divisor: y,
        } => println!("{}", x / y),
        Expr::Val(x) => println!("{}", x),
    }
}

print_expr(sum);
print_expr(quotient);

// ---------------
fn uppercase(c: u8) -> u8 {
    match c {
        b'a'..=b'z' => c - 32,
        _ => c,
    }
}
let letter = 'a' as u8;
let change_to_upper = uppercase(letter);
println!("letter: {}", letter as char);
println!("change_to_upper: {}", change_to_upper as char);

// ---------------
use std::ops::Add;

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, point: Point) -> Self::Output {
        Point {
            x: self.x + point.x,
            y: self.y + point.y,
        }
    }
}

let p01 = Point { x: 1, y: 2 };
let p02 = Point { x: 3, y: 4 };
let p03 = p01 + p02;

print_point(&p03);

// -------------

let array = [1, 2, 3, 4];
let mut sum = 0;
for element in &array {
    sum += *element;
}
println!("Sum: {}", sum);

fn index<T: PartialEq>(slice: &[T], target: &T) -> Option<usize> {
    for (index, element) in slice.iter().enumerate() {
        if element == target {
            return Some(index);
        }
    }
    None
}

let got_index = index(&array, &3);
match got_index {
    Some(ind) => println!("The index position of 3 is {}", ind),
    None => println!("3 vas not in the array"),
}

const ARRAY2: [i32; 10] = [45, 26, 33, 11, 10, 2, 85, 97, 20, 12];

fn min_max(slice: &[i32]) -> Option<(i32, i32)> {
    if slice.is_empty() {
        return None;
    }
    let mut min = slice[0];
    let mut max = slice[0];
    for &element in slice {
        if element < min {
            min = element;
        }
        if element > max {
            max = element;
        }
    }
    Some((min, max))
}
match min_max(&ARRAY2) {
    Some((mi, ma)) => println!("The min is {} and the max is {}", mi, ma),
    None => println!("The array was empty"),
}


fn number_to_string(i: i32) -> String {
    i.to_string()
    // alternative:
    // format!("{}",i)
}

println!("{}", number_to_string(1+2));
}

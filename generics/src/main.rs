// This is actually for generics and traits


struct Point<S,T> { x: S, y: T, }

// Impl for S,T that mixes up 2 points
impl<S,T> Point<S,T> {
    // note the X,Y arg to mixup and the S,Y on the return
    fn mixup<X,Y> (self, other: Point<X,Y>) -> Point<S,Y> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// a specific impl that only works with f64
impl Point<f64,f64> {
    fn distance(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    println!("Hello, world!");
    let int_list = [1, 2, 3,5];
    let largest_int = largest(&int_list);
    println!("{}",largest_int);
    let float_list = [1.2, 1.3, 0.3,0.5];
    let largest_float = largest(&float_list);
    println!("{}",largest_float);

    let point_1 = Point{x: 1.5, y: 1};
    let point_2 = Point {x: 1, y: 1.5};

    let mixed = point_1.mixup(point_2);
    println!("x: {}, y: {}", mixed.x, mixed.y);
    println!("Distance: {}", mixed.distance());
}

// generic function and return type
// The where clause specifies requirements for T
// There is also different notation for this but this seems to be the cleanest for me
fn largest<T>(list: &[T]) -> &T  where T: PartialOrd {

    let mut  largest_val = &list[0];
    for item in list {
        if largest_val < item {
            largest_val = item;
        }
    }
    largest_val
}

// This trait is just to show that it has generic parameter
// and we can substitute correct type im generated methods.
// Stdlib trait with the same semantic is `Into`
trait Convert<T> {
    fn convert(self) -> T;
}

struct Point {
    x: i32,
    y: i32
}

impl Convert<Point> for (i32, i32) {

}


impl Into<Point> for (i32, i32) {
}

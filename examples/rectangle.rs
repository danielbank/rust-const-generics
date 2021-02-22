#![feature(array_chunks)]

#[derive(Debug)]
struct RectangularArray<T, const WIDTH: usize, const HEIGHT: usize> {
    data: [[T; WIDTH]; HEIGHT],
}

fn main() {
    let two_by_three_data = [[1, 2], [3, 4], [5, 6]];
    let two_by_three = RectangularArray::<i32, 2, 3> {
        data: two_by_three_data,
    };
    println!("{:?}", two_by_three);

    let three_by_two_data = [[1, 2, 3], [4, 5, 6]];
    let three_by_two = RectangularArray::<i32, 3, 2> {
        data: three_by_two_data,
    };
    println!("{:?}", three_by_two);
}

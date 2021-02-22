#[derive(Debug)]
struct RectangularArray<T> {
    width: usize,
    height: usize,
    data: Vec<Vec<T>>,
}

fn main() {
    let two_by_three_data = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let two_by_three = RectangularArray::<i32> {
        width: 2,
        height: 3,
        data: two_by_three_data,
    };
    println!("{:?}", two_by_three);

    let three_by_two_data = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let three_by_two = RectangularArray::<i32> {
        width: 3,
        height: 2,
        data: three_by_two_data,
    };
    println!("{:?}", three_by_two);
}

#![feature(array_chunks)]
fn main() {
    let data = [1, 2, 3, 4, 5, 6];

    let sum1 = data.array_chunks().map(|&[x, y]| x * y).sum::<i32>();
    assert_eq!(sum1, (1 * 2) + (3 * 4) + (5 * 6));

    let sum2 = data.array_chunks().map(|&[x, y, z]| x * y * z).sum::<i32>();
    assert_eq!(sum2, (1 * 2 * 3) + (4 * 5 * 6));
}

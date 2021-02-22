#![feature(array_chunks)]
use ndarray_image::{open_gray_image, save_gray_image};
use std::cell::RefCell;

#[derive(Debug)]
struct Kernel<T, const SIZE: usize> {
    elements: [T; SIZE],
}

fn main() {
    let kernel = Kernel::<u8, 9> {
        elements: [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8],
    };

    let image = open_gray_image("./input.jpg").expect("unable to open input image");

    let image = image.map(RefCell::new);
    for (_, chunk) in image.windows((3, 3)).into_iter().enumerate() {
        let mut sum: u8 = 0;
        for (x, y) in chunk.iter().zip(kernel.elements.iter()) {
            sum = sum + (x.into_inner() * y);
        }
        *chunk[[1, 1]].borrow_mut() = sum / 9;
    }
    let image = image.map(|x| *x.into_inner());
    save_gray_image("output.jpg", image.view()).expect("failed to write output");
}

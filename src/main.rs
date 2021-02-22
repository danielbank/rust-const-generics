#![feature(array_chunks)]
use ndarray_image::{open_gray_image, save_gray_image};

#[derive(Debug)]
struct Kernel<T, const SIZE: usize> {
    elements: [T; SIZE],
}

fn main() {
    let kernel = Kernel::<u8, 9> {
        elements: [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8],
    };

    let mut image = open_gray_image("./input.jpg").expect("unable to open input image");

    for (_, mut chunk) in image.exact_chunks_mut((3, 3)).into_iter().enumerate() {
        let mut sum = 0;
        for (x, y) in chunk.iter().zip(kernel.elements.iter()) {
            sum = sum + (x * y);
        }
        chunk[[1, 1]] = sum / 9 as u8;
    }
    save_gray_image("output.jpg", image.view()).expect("failed to write output");
}

use array2::Array2;
use csc411_image::{Read, GrayImage};
use std::env;
use std::io;
use std::io::BufRead;

fn main() {
    let input = env::args().nth(1);
    let arr2= read(input);
}

fn read(input: Option<String>) -> Array2<i8> {
        let copy = input.clone();
        assert!(env::args().len() == 2);
        let img = GrayImage::read(copy.as_deref()).unwrap();
        println!("{:?}",img.width);
        assert!(input.expect("REASON").contains(".pgm"));
        
        //checks if img has a height of 9 elements and if img denominator is 9
        if img.height != 9 || img.width != 9{
            std::process::exit(1);
        }

        let mut vec: Vec<i8> = vec![];
        
        //checks whether or not the pixel values are between 1 and 9
        for pixel in img.pixels {
            if (pixel.value < 1) || (pixel.value > 9){
                std::process::exit(1);
            }
            vec.push(pixel.value as i8)
        }
        
        //checks if vec has 81 elements
        if vec.len() != 81{
            std::process::exit(1);
        }

        //creates an arr2 of the data taken in from the .pgm file
        let arr2: Array2<i8>;
        arr2 = Array2::<i8>::from_row_major(9, 9, vec);

        return arr2;

    }

    fn rotate_180(arr: Array2){
        let rotU = Array2.from_col_major(arr.width, arr.height, arr.iter_row_major.collect().reverse());
    }
    
    fn transpose(arr: Array2){
        let rotT = Array2.from_col_major(arr.height, arr.width, arr.iter_row_major);
    }

    fn rotate_90(arr: Array2){
        let slice_list = [];
        for chunk in arr.data.chunk(arr.width){
            slice_list.push(chunk);
        }
        let flipped = [];
        for i in slice_list.rev(){
            for data in i{
                flipped.push(data);
            }
        }
    }
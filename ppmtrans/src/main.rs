use array2::Array2;
use csc411_image::*;
use std::env;
use std::io;
use std::io::BufRead;

fn main() {
    let input = env::args().nth(1);
    let fname = env::args().nth(2);
    let fname2 = env::args().nth(3);
    let arr2 = read(input);
    
    let rot90: Array2<(imgtype::Rgb)> = rotate_90_row(arr2.clone());
    //write(rot90, fname)
    let rot180: Array2<(imgtype::Rgb)> = rotate_180_row(arr2.clone());

    let mut vec: Vec<imgtype::Rgb> = vec![];
    for pixel in rot90.iter_row_major(){
        vec.push(pixel.2.clone());
    }

    let rot90ppm = RgbImage{pixels: vec, width: rot90.get_width() as u32, height: rot90.get_height() as u32, denominator: 255};
    rot90ppm.write(fname.as_ref().map(|x| &**x)).unwrap_or(());

    vec = vec![];
    for pixel in rot180.iter_row_major(){
        vec.push(pixel.2.clone());
    }
    
    let rot180ppm = RgbImage{pixels: vec, width: rot180.get_width() as u32, height: rot180.get_height() as u32, denominator: 255};
    rot180ppm.write(fname2.as_ref().map(|x| &**x)).unwrap_or(());
}

fn read(input: Option<String>) -> Array2<(imgtype::Rgb)> {
    let copy = input.clone();
    let img = RgbImage::read(copy.as_deref()).unwrap();
    println!("{:?}",img.width);

    let mut vec: Vec<(imgtype::Rgb)> = vec![];
    
    //checks whether or not the pixel values are between 1 and 9
    for pixel in img.pixels {
        vec.push(pixel);
    }

    //creates an arr2 of the data taken in from the .pgm file
    let arr2 = Array2::<(imgtype::Rgb)>::from_row_major(img.width as usize, img.height as usize, vec);

    return arr2;

}

fn rotate_180_row(arr: Array2<(imgtype::Rgb)>) -> Array2<(imgtype::Rgb)>{

    let mut arr_rot = Array2::<(imgtype::Rgb)>::single_val(arr.get_width(), arr.get_height(), imgtype::Rgb{red:255,green:255, blue:255});
    for pixel in arr.iter_row_major(){
        arr_rot.set_index(arr.get_width() - pixel.1 - 1, arr.get_height() - pixel.0 - 1, pixel.2.clone());
    }

    return arr_rot;
    //let rotU = Array2.from_col_major(arr.width, arr.height, arr.iter_row_major.collect().reverse());
}

fn rotate_180_col(arr: Array2<(imgtype::Rgb)>) -> Array2<(imgtype::Rgb)>{

    let mut arr_rot = Array2::<(imgtype::Rgb)>::single_val(arr.get_width(), arr.get_height(), imgtype::Rgb{red:255,green:255, blue:255});
    for pixel in arr.iter_column_major(){
        arr_rot.set_index(arr.get_width() - pixel.1 - 1, arr.get_height() - pixel.0 - 1, pixel.2.clone());
    }

    return arr_rot;
    //let rotU = Array2.from_col_major(arr.width, arr.height, arr.iter_row_major.collect().reverse());
}
    
fn transpose(arr: Array2<(imgtype::Rgb)>){
        
    //let rotT = Array2.from_col_major(arr.height, arr.width, arr.iter_row_major);
}

fn rotate_90_col(arr: Array2<(imgtype::Rgb)>) -> Array2<(imgtype::Rgb)>{
        
    let mut arr_rot = Array2::<(imgtype::Rgb)>::single_val(arr.get_height(), arr.get_width(), imgtype::Rgb{red:255, green:255, blue:255});
    for pixel in arr.iter_column_major(){
        arr_rot.set_index(pixel.0, arr.get_height() - pixel.1 - 1, pixel.2.clone());
    }
        
    return arr_rot;
    }

fn rotate_90_row(arr: Array2<(imgtype::Rgb)>) -> Array2<(imgtype::Rgb)>{
        
    let mut arr_rot = Array2::<(imgtype::Rgb)>::single_val(arr.get_height(), arr.get_width(), imgtype::Rgb{red:255, green:255, blue:255});
    for pixel in arr.iter_row_major(){
        arr_rot.set_index(pixel.0, arr.get_height() - pixel.1 - 1, pixel.2.clone());
    }
        
    return arr_rot;

    /*
    let slice_list = [];
    for chunk in arr.data.chunk(arr.width){
        slice_list.push(chunk);
    }
    let flipped = [];
        for i in slice_list.rev(){
            for data in i{
                flipped.push(data);
            }
        }*/
    }

fn rotate_270_row(arr: Array2<(imgtype::Rgb)>) -> Array2<(imgtype::Rgb)>{
        
    let mut arr_rot = Array2::<(imgtype::Rgb)>::single_val(arr.get_height(), arr.get_width(), imgtype::Rgb{red:255, green:255, blue:255});
    for pixel in arr.iter_row_major(){
        arr_rot.set_index(arr.get_height() - pixel.1 - 1, pixel.0, pixel.2.clone());
    }
        
    return arr_rot;
}

fn rotate_270_col(arr: Array2<(imgtype::Rgb)>) -> Array2<(imgtype::Rgb)>{
        
    let mut arr_rot = Array2::<(imgtype::Rgb)>::single_val(arr.get_height(), arr.get_width(), imgtype::Rgb{red:255, green:255, blue:255});
    for pixel in arr.iter_column_major(){
        arr_rot.set_index(arr.get_height() - pixel.1 - 1, pixel.0, pixel.2.clone());
    }
        
    return arr_rot;
}
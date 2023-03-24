use array2::Array2;
use csc411_image::*;
use std::env;
//use std::io;
//use std::io::BufRead;

fn main() {


    /*
        command example (for now) : cargo run rotation90 row-major f_original.ppm rot_fin.ppm **
    */



    let args: Vec<String> = env::args().collect();

    let rotation = &args[1];
    let major = &args[2];
    let fname = env::args().nth(3);
    let fname2 = env::args().nth(4);
    let arr2 = read(fname);
    let mut rot = Array2::<imgtype::Rgb>::single_val(1, 1, imgtype::Rgb{red:255, green:255, blue:255});

    if  rotation == "rotation90" {
        if major == "row-major" {
            rot = rotate_90_row(arr2.clone());
        }
        else if major == "col-major" {
            rot = rotate_90_col(arr2.clone());
        }
    }
    else if  rotation == "rotation180" {
        if major == "row-major" {
            rot = rotate_180_row(arr2.clone());
        }
        else if major == "col-major" {
            rot = rotate_180_col(arr2.clone());
        }
    }
    else if  rotation == "transpose" {
        if major == "row-major" {
            rot = transpose_row(arr2.clone());
        }
        else if major == "col-major" {
            rot = transpose_col(arr2.clone());
        }
    }
    
    write(rot, fname2);

}

fn read(input: Option<String>) -> Array2<imgtype::Rgb> {
    let copy = input.clone();
    let img = RgbImage::read(copy.as_deref()).unwrap();
    println!("{:?}",img.width);

    let mut vec: Vec<imgtype::Rgb> = vec![];
    
    //checks whether or not the pixel values are between 1 and 9
    for pixel in img.pixels {
        vec.push(pixel);
    }

    //creates an arr2 of the data taken in from the .pgm file
    let arr2 = Array2::<imgtype::Rgb>::from_row_major(img.width as usize, img.height as usize, vec);

    return arr2;

}

fn write(rot: Array2<imgtype::Rgb>, final_destination: Option<String>) {
    let mut vec: Vec<imgtype::Rgb> = vec![];
    for pixel in rot.iter_row_major(){
        vec.push(pixel.2.clone());
    }

    let ppm = RgbImage{pixels: vec, width: rot.get_width() as u32, height: rot.get_height() as u32, denominator: 255};
    ppm.write(final_destination.as_ref().map(|x| &**x)).unwrap_or(());
}

fn rotate_180_row(arr: Array2<imgtype::Rgb>) -> Array2<imgtype::Rgb> {

    let mut arr_rot = Array2::<imgtype::Rgb>::single_val(arr.get_width(), arr.get_height(), imgtype::Rgb{red:255,green:255, blue:255});
    for pixel in arr.iter_row_major(){
        arr_rot.set_index(arr.get_width() - pixel.1 - 1, arr.get_height() - pixel.0 - 1, pixel.2.clone());
    }

    return arr_rot;
}

fn rotate_180_col(arr: Array2<imgtype::Rgb>) -> Array2<imgtype::Rgb> {

    let mut arr_rot = Array2::<imgtype::Rgb>::single_val(arr.get_width(), arr.get_height(), imgtype::Rgb{red:255,green:255, blue:255});
    for pixel in arr.iter_column_major(){
        arr_rot.set_index(arr.get_width() - pixel.1 - 1, arr.get_height() - pixel.0 - 1, pixel.2.clone());
    }

    return arr_rot;
}

fn transpose_row(arr: Array2<imgtype::Rgb>) -> Array2<imgtype::Rgb> {
        
    let mut arr_rot = Array2::<imgtype::Rgb>::single_val(arr.get_height(), arr.get_width(), imgtype::Rgb{red:255, green:255, blue:255});
    for pixel in arr.iter_row_major(){
        arr_rot.set_index( arr.get_width() - pixel.0 - 1, arr.get_height() - pixel.1 - 1, pixel.2.clone());
    }

    return arr_rot;
}

fn transpose_col(arr: Array2<imgtype::Rgb>) -> Array2<imgtype::Rgb> {
        
    let mut arr_rot = Array2::<imgtype::Rgb>::single_val(arr.get_height(), arr.get_width(), imgtype::Rgb{red:255, green:255, blue:255});
    for pixel in arr.iter_column_major(){
        arr_rot.set_index( arr.get_width() - pixel.0 - 1, arr.get_height() - pixel.1 - 1, pixel.2.clone());
    }

    return arr_rot;
}

fn rotate_90_col(arr: Array2<imgtype::Rgb>) -> Array2<imgtype::Rgb>{
        
    let mut arr_rot = Array2::<imgtype::Rgb>::single_val(arr.get_height(), arr.get_width(), imgtype::Rgb{red:255, green:255, blue:255});
    for pixel in arr.iter_column_major(){
        arr_rot.set_index(pixel.0, arr.get_height() - pixel.1 - 1, pixel.2.clone());
    }
        
    return arr_rot;
    }

fn rotate_90_row(arr: Array2<imgtype::Rgb>) -> Array2<imgtype::Rgb>{
        
    let mut arr_rot = Array2::<imgtype::Rgb>::single_val(arr.get_height(), arr.get_width(), imgtype::Rgb{red:255, green:255, blue:255});
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
/*
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

fn rotate_0(arr: Array2<(imgtype::Rgb)>) -> Array2<(imgtype::Rgb)>{
        
    pix1, pix0, data
}

fn rotate_horiz(arr: Array2<(imgtype::Rgb)>) -> Array2<(imgtype::Rgb)>{
        
    pix1, width - pix0 - 1, data
}

fn rotate_vert(arr: Array2<(imgtype::Rgb)>) -> Array2<(imgtype::Rgb)>{
        
    width - pix1 - 1, pix0, data
}
*/
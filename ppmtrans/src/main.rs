use clap::Parser;
use array2::Array2;
use csc411_image::*;
use std::env;
use std::time::Instant;
use std::io;
use std::io::BufRead;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]

struct Args {
    input: Option<String>,

    #[clap(long = "row-major")]
    row_m: bool,
    
    #[clap(long = "col-major")]
    col_m: bool,

    #[clap(long = "transpose")]
    trans: bool,
    
    #[clap(long = "flip", required = false)]
    flip: Option<String>,

    #[clap(short = 'r', long = "rotate")]
    rot: Option<u32>,   
}

fn main() {


    /*
        command example (for now) : cargo run rotation90 row-major f_original.ppm rot_fin.ppm **
    */
    let args = Args::parse();
    let rotate = args.rot;
    let row_major = args.row_m;
    let col_major = args.col_m;
    let flip = args.flip;
    let transpose = args.trans;
    let fname = args.input;

    let arr2 = read(fname);
    let mut rot = Array2::<imgtype::Rgb>::single_val(1, 1, imgtype::Rgb{red:255, green:255, blue:255});

    let h_string = "horizontal".to_string();
    let v_string = "vertical".to_string();
    let mut f_string = "yes".to_string();
    let mut r_num = 1;
    if flip.is_some(){
        f_string = flip.unwrap();
    }
    else if !transpose{
        r_num = rotate.unwrap();


    }

    if  r_num == 0 { //good
        if row_major{
            rot = rotate_0_row(arr2.clone());
        }
        else if col_major {
            rot = rotate_0_col(arr2.clone());
        }
    }
    else if  r_num == 90 { //good
        if row_major{
            rot = rotate_90_row(arr2.clone());
        }
        else if col_major {
            rot = rotate_90_col(arr2.clone());
        }
    }
    else if  r_num == 180 { //good
        if row_major{
            rot = rotate_180_row(arr2.clone());
        }
        else if col_major {
            rot = rotate_180_col(arr2.clone());
        }
    }
    else if r_num == 270 { //good
        if row_major{
            rot = rotate_270_row(arr2.clone());
        }
        else if col_major {
            rot = rotate_270_col(arr2.clone());
        }
    }
    else if transpose { //good
        if row_major {
            rot = transpose_row(arr2.clone());
        }
        else if col_major {
            rot = transpose_col(arr2.clone());
        }
    }
    else if f_string == h_string { //good
        if row_major {
            rot = rotate_horiz_row(arr2.clone());
        }
        else if col_major {
            rot = rotate_horiz_col(arr2.clone());
        }
    }
    else if f_string == v_string { //good
        if row_major {
            rot = rotate_vert_row(arr2.clone());
        }
        else if col_major {
            rot = rotate_vert_col(arr2.clone());
        }
    }
    
    write(rot);

}

fn read(input: Option<String>) -> Array2<imgtype::Rgb> {
    let copy = input.clone();
    let img = RgbImage::read(copy.as_deref()).unwrap();

    let mut vec: Vec<imgtype::Rgb> = vec![];
    
    //checks whether or not the pixel values are between 1 and 9
    for pixel in img.pixels {
        vec.push(pixel);
    }

    //creates an arr2 of the data taken in from the .pgm file
    let arr2 = Array2::<imgtype::Rgb>::from_row_major(img.width as usize, img.height as usize, vec);

    return arr2;

}

fn write(rot: Array2<imgtype::Rgb>) {
    let mut vec: Vec<imgtype::Rgb> = vec![];
    for pixel in rot.iter_row_major(){
        vec.push(pixel.2.clone());
    }

    let ppm = RgbImage{pixels: vec, width: rot.get_width() as u32, height: rot.get_height() as u32, denominator: 255};
    //ppm.write(output_file.as_ref().map(|x| &**x)).unwrap_or(());
    ppm.write(None).unwrap();
}

fn rotate_180_row(arr: Array2<imgtype::Rgb>) -> Array2<imgtype::Rgb> {

    let mut arr_rot = Array2::<imgtype::Rgb>::single_val(arr.get_width(), arr.get_height(), imgtype::Rgb{red:255,green:255, blue:255});
    let now = Instant::now();
    for pixel in arr.iter_row_major(){
        arr_rot.set_index(arr.get_height() - pixel.1 - 1, arr.get_width() - pixel.0 - 1, pixel.2.clone());
    }
    let elapsed = now.elapsed();
    //eprintln!("{:.2?}", elapsed);

    return arr_rot;
}

fn rotate_180_col(arr: Array2<imgtype::Rgb>) -> Array2<imgtype::Rgb> {

    let mut arr_rot = Array2::<imgtype::Rgb>::single_val(arr.get_width(), arr.get_height(), imgtype::Rgb{red:255,green:255, blue:255});
    let now = Instant::now();
    for pixel in arr.iter_column_major(){
        arr_rot.set_index(arr.get_height() - pixel.0 - 1, arr.get_width() - pixel.1 - 1, pixel.2.clone());
    }
    let elapsed = now.elapsed();
    //eprintln!("{:.2?}", elapsed);

    return arr_rot;
}

/// Transposes an Array2 by iterating through each row then moving on to the next
///
/// Creates an Array2 of correctly transposed size of the original and fills it with blank Rgb pixels. Then, iterates through
/// the original Array2 by row and sets each pixel to it's transposed position in the new Array2, then returns transposed Array2.
fn transpose_row(arr: Array2<imgtype::Rgb>) -> Array2<imgtype::Rgb> {
        
    let mut arr_rot = Array2::<imgtype::Rgb>::single_val(arr.get_height(), arr.get_width(), imgtype::Rgb{red:255, green:255, blue:255});
    let now = Instant::now();
    for pixel in arr.iter_row_major(){
        arr_rot.set_index(pixel.0, pixel.1, pixel.2.clone());
        // arr.get_width() - pixel.0 - 1, arr.get_height() - pixel.1 - 1, pixel.2.clone()
    }
    let elapsed = now.elapsed();
    //eprintln!("{:.2?}", elapsed);

    return arr_rot;
}

fn transpose_col(arr: Array2<imgtype::Rgb>) -> Array2<imgtype::Rgb> {
        
    let mut arr_rot = Array2::<imgtype::Rgb>::single_val(arr.get_height(), arr.get_width(), imgtype::Rgb{red:255, green:255, blue:255});
    let now = Instant::now();
    for pixel in arr.iter_column_major(){
        arr_rot.set_index( pixel.0, pixel.1, pixel.2.clone());
    }
    let elapsed = now.elapsed();
    //eprintln!("{:.2?}", elapsed);

    return arr_rot;
}

fn rotate_90_col(arr: Array2<imgtype::Rgb>) -> Array2<imgtype::Rgb>{
        
    let mut arr_rot = Array2::<imgtype::Rgb>::single_val(arr.get_height(), arr.get_width(), imgtype::Rgb{red:255, green:255, blue:255});
    let now = Instant::now();
    for pixel in arr.iter_column_major(){
        arr_rot.set_index(pixel.0, arr.get_height() - pixel.1 - 1, pixel.2.clone());
    }
    let elapsed = now.elapsed();
    //eprintln!("{:.2?}", elapsed);

    return arr_rot;
    }

fn rotate_90_row(arr: Array2<imgtype::Rgb>) -> Array2<imgtype::Rgb>{
        
    let mut arr_rot = Array2::<imgtype::Rgb>::single_val(arr.get_height(), arr.get_width(), imgtype::Rgb{red:255, green:255, blue:255});
    let now = Instant::now();
    for pixel in arr.iter_row_major(){
        arr_rot.set_index(pixel.0, arr.get_height() - pixel.1 - 1, pixel.2.clone());
    }
    let elapsed = now.elapsed();
    //eprintln!("{:.2?}", elapsed);

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

fn rotate_270_row(arr: Array2<imgtype::Rgb>) -> Array2<imgtype::Rgb>{
        
    let mut arr_rot = Array2::<imgtype::Rgb>::single_val(arr.get_height(), arr.get_width(), imgtype::Rgb{red:255, green:255, blue:255});
    let now = Instant::now();
    for pixel in arr.iter_row_major(){
        arr_rot.set_index(arr.get_width() - pixel.0 - 1, pixel.1, pixel.2.clone());
    }
    let elapsed = now.elapsed();
    //eprintln!("{:.2?}", elapsed);
        
    return arr_rot;
}

fn rotate_270_col(arr: Array2<imgtype::Rgb>) -> Array2<imgtype::Rgb>{
        
    let mut arr_rot = Array2::<imgtype::Rgb>::single_val(arr.get_height(), arr.get_width(), imgtype::Rgb{red:255, green:255, blue:255});
    let now = Instant::now();
    for pixel in arr.iter_column_major(){
        arr_rot.set_index(arr.get_width() - pixel.0 - 1, pixel.1, pixel.2.clone());
    }
    let elapsed = now.elapsed();
    //eprintln!("{:.2?}", elapsed);
        
    return arr_rot;
}

fn rotate_0_row(arr: Array2<imgtype::Rgb>) -> Array2<imgtype::Rgb>{

    let arr_rot = arr;
    let now = Instant::now();
    // for pixel in arr.iter_row_major(){
    //     arr_rot.set_index(pixel.1, pixel.0, pixel.2.clone());
    // }
    let elapsed = now.elapsed();
    //eprintln!("{:.2?}", elapsed);
    return arr_rot;
}

fn rotate_0_col(arr: Array2<imgtype::Rgb>) -> Array2<imgtype::Rgb>{

    let arr_rot = arr;
    let now = Instant::now();
    //for pixel in arr.iter_column_major(){
    //    arr_rot.set_index(pixel.1, pixel.0, pixel.2.clone());
    //}
    let elapsed = now.elapsed();
    //eprintln!("{:.2?}", elapsed);
    return arr_rot;
}

fn rotate_horiz_row(arr: Array2<imgtype::Rgb>) -> Array2<imgtype::Rgb>{

    let mut arr_rot = Array2::<imgtype::Rgb>::single_val(arr.get_width(), arr.get_height(), imgtype::Rgb{red:255, green:255, blue:255});
    let now = Instant::now();
    for pixel in arr.iter_row_major(){
        arr_rot.set_index(pixel.1, arr.get_width() - pixel.0 - 1, pixel.2.clone());
    }
    let elapsed = now.elapsed();
    //eprintln!("{:.2?}", elapsed);
    return arr_rot;
}

fn rotate_horiz_col(arr: Array2<imgtype::Rgb>) -> Array2<imgtype::Rgb>{

    let mut arr_rot = Array2::<imgtype::Rgb>::single_val(arr.get_width(), arr.get_height(), imgtype::Rgb{red:255, green:255, blue:255});
    let now = Instant::now();
    for pixel in arr.iter_column_major(){
        arr_rot.set_index(pixel.1, arr.get_width() - pixel.0 - 1, pixel.2.clone());
    }
    let elapsed = now.elapsed();
    //eprintln!("{:.2?}", elapsed);
    return arr_rot;
}

fn rotate_vert_row(arr: Array2<imgtype::Rgb>) -> Array2<imgtype::Rgb>{

    let mut arr_rot = Array2::<imgtype::Rgb>::single_val(arr.get_width(), arr.get_height(), imgtype::Rgb{red:255, green:255, blue:255});
    let now = Instant::now();
    for pixel in arr.iter_row_major(){
        arr_rot.set_index(arr.get_height() - pixel.1 - 1, pixel.0, pixel.2.clone());
    }
    let elapsed = now.elapsed();
    //eprintln!("{:.2?}", elapsed);
    return arr_rot;
}

fn rotate_vert_col(arr: Array2<imgtype::Rgb>) -> Array2<imgtype::Rgb>{

    let mut arr_rot = Array2::<imgtype::Rgb>::single_val(arr.get_width(), arr.get_height(), imgtype::Rgb{red:255, green:255, blue:255});
    let now = Instant::now();
    for pixel in arr.iter_column_major(){
        arr_rot.set_index(arr.get_height() - pixel.1 - 1, pixel.0, pixel.2.clone());
    }
    let elapsed = now.elapsed();
    //eprintln!("{:.2?}", elapsed);
    return arr_rot;
}
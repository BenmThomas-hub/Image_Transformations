pub fn new(input: Vec<String>) -> Vec<Vec<String>>{

    let mut row_count = 0;
    let mut column_count = 0;
    let mut arr2: Vec<Vec<String>>;
    for pixel in input{

        if pixel = "\n" {
            row_count += 1;
        }
        else {
            arr2[row_count][column_count] = pixel as String;
        }
        column_count += 1;
    }
}
/*
pub fn get_width(column: usize) -> Option<_>{

}

pub fn get_height(row: usize) -> Option<_>{

}
*/
pub fn iter_row_major(width: i16, height: i16, arr2: Vec<Vec<i16>>) -> Vec<i16> {

    let mut row_major: Vec<i16>;
    let mut num: i16 = 0;

    for column in width {
        for row in height {
            row_major[num] = arr2[row][column];
            num += 1;
        }
        
    }


}

pub fn iter_column_major(width: i32, height: i32, arr2: Vec<Vec<i16>>) -> Vec<i16> {

    let mut column_major: Vec<i16>;
    let mut num = 0;

    for row in height {
        for column in width {
            column_major[num] = arr2[row][column];
            num += 1;
        }
        
    }

}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

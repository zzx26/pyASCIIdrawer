use pyo3::prelude::*;
use core::str;
use image::GenericImageView;

/// Wrapper for demo_img_rusty function
#[pyfunction]
fn demo_img() -> PyResult<String> {
    Ok(demo_img_rusty())
}

/// Function that does all the heavy lifting
#[pyfunction]
fn draw(path_to: String, symbols: String, desired_height: usize, desired_width: usize, normalize: bool) -> PyResult<String> {
    let mut img_vec = Vec::new();
    if normalize {
        img_vec = normalizer(vec_from_img(&path_to));
    } else {
        img_vec = vec_from_img(&path_to);
    }
    let img_ascii = fitter(img_vec, &symbols, desired_height, desired_width);
    let img_str = formatted_string(&img_ascii);
    Ok(img_str)
}

/// A Python module implemented in Rust.
#[pymodule]
fn asciidrawer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(demo_img, m)?)?;
    m.add_function(wrap_pyfunction!(draw, m)?)?;
    Ok(())
}

fn demo_img_rusty() -> String {
    let path_str_cat = "resources\\cat.png";
    let cat_img = vec_from_img(path_str_cat);
    let cat_ascii = fitter(cat_img, " .,:;blBL@", 40, 60);
    let cat_str = formatted_string(&cat_ascii);
    cat_str
}

//for debug
fn formatted_print(img_vec: & Vec<Vec<u8>>) {
    for i in img_vec {
        println!("{:?}", i);
    }
}

//for debug
fn formatted_print_ascii(char_vec: & Vec<Vec<char>>) {
    for i in char_vec{
        for j in i {
            print!("{}", j);
        }
        print!("\n");
    }
}

fn formatted_string(char_vec: & Vec<Vec<char>>) -> String {
    let mut ret = String::new();
    for i in char_vec{
        for j in i {
            ret.push(*j);
        }
        ret.push('\n');
    }
    ret
}

// reads image and converst to mono vec8 2d
fn vec_from_img(path_str: &str) -> Vec<Vec<u8>> {
    let img = image::open(path_str).expect(&format!("Failed to open image {}", path_str));
    // Get the dimensions of the image
    let (width, height) = img.dimensions();

    // Output Vec
    let mut output: Vec<Vec<u8>> = vec![vec![0; width.try_into().unwrap()]; height.try_into().unwrap()];
    // Iterate over the pixels of the image
    for y in 0..height {
        let y_usize: usize = y.try_into().unwrap();
        for x in 0..width {
            // Get the RGB values of the pixel at (x, y)
            let pixel = img.get_pixel(x, y);
            let x_usize: usize = x.try_into().unwrap();
            output[y_usize][x_usize] = pixel[0] / 3 + pixel[1] / 3 + pixel[2] / 3 ;
        }
    }
    output
}


// change size of an image vec
fn resizer(img_vec: Vec<Vec<u8>>, desired_height: usize, desired_width: usize) -> Vec<Vec<u8>> {
    let old_x = img_vec[0].len();
    let old_y = img_vec.len();

    let mut output: Vec<Vec<u8>> = vec![vec![255; desired_width]; desired_height];
    let new_cell_size_x = old_x;
    let new_cell_size_y = old_y; 
    let new_cell_area = new_cell_size_x*new_cell_size_y;
    for i in 0..desired_height {
        for j in 0..desired_width {
            // compute avg over this iteration
            let mut avg_usize: usize = 0;
            for k in 0..new_cell_size_y {
                for h in 0..new_cell_size_x {
                    let index_x = (j*new_cell_size_x+h) / desired_width;
                    let index_y = (i*new_cell_size_y+k) / desired_height;
                    avg_usize += usize::from(img_vec[index_y][index_x]);
                }
            }
            let avg: u8 = u8::try_from(avg_usize / new_cell_area).unwrap();
            output[i][j] = avg;
        }
    }
    output
}

fn normalizer(img_vec: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut sum: i32 = 0;
    let len_x = i32::try_from(img_vec[0].len()).unwrap();
    for row in & img_vec {
        sum += row.iter().map(|&x| x as i32).sum::<i32>() / len_x;
    }
    let mean = sum / i32::try_from(img_vec.len()).unwrap(); 
    let mut output: Vec<Vec<u8>> = vec![vec![255; img_vec[0].len()]; img_vec.len()];

    for i in 0..img_vec.len() {
        for j in 0..img_vec[0].len() {
            output[i][j] = ((img_vec[i][j] as i32 - mean + 128) * 255 / 256) as u8;
        }
    }
    output
}

fn inds_getter(symbols: &str) -> Vec<usize>{

    let all_inds: [u8; 82] = [32,  33, 35, 36, 37, 38, 40, 41, 42, 43, 44, 45, 46, 48, 49, 50, 51, 
    52, 53, 54, 55, 56, 57, 58, 59, 63, 64, 65, 66, 67, 68, 69, 70, 71, 21, 73, 74, 75, 76, 
    77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 94, 97, 98, 99, 100, 101, 102, 
    103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 
    120, 121, 122, 123, 125];

    let symb_vec: Vec<char> = symbols.chars().collect();
    let mut output: Vec<usize> = Vec::new();

    for symbol in symb_vec {
        if symbol.is_ascii_graphic() || (symbol as u8 == 32) {
            let symbol_u8 = symbol as u8;
            if all_inds.contains(&symbol_u8) {
                // TODO: WTF
                output.push(usize::from(symbol_u8));
            }
        } 
    }
    output
}

fn usize_to_char(input: usize) -> char {
    char::from(input as u8)
}

fn fitter(input: Vec<Vec<u8>>, symbols: &str, desired_height: usize, desired_width: usize) -> Vec<Vec<char>> {
    



    let symbol_size_x: usize = 9;
    let symbol_size_y: usize = 16;

    let new_y = desired_height*symbol_size_y;
    let new_x = desired_width*symbol_size_x;

    let symbols_ind_vec = inds_getter(symbols);

    //iterate over this to count the best symbol to fit
    let mut symbols_img_vec: Vec<Vec<Vec<u8>>> = Vec::new();

    for n in & symbols_ind_vec {
        let path_to_symb = format!("resources\\ascii{}.png", n);
        symbols_img_vec.push(vec_from_img(&path_to_symb));
    }

    let img_to_match: Vec<Vec<u8>> = resizer(input, new_y, new_x);

    let mut output: Vec<Vec<char>> = vec![vec![' '; desired_width]; desired_height];

    for i in 0..desired_height {
        for j in 0..desired_width {
            let first_symbol = & symbols_img_vec[0];
            let mut best_msd: u64 = 0;
            let mut best_symbol_ind: usize = 0;
            for k in 0..symbol_size_y {
                for h in 0..symbol_size_x {
                    best_msd += u64::from(img_to_match[i*symbol_size_y + k][j*symbol_size_x + h].abs_diff(first_symbol[k][h])).pow(2);
                }
            }
            for symbol_ind in 1..symbols_img_vec.len() {
                let mut msd = 0;
                for k in 0..symbol_size_y {
                    for h in 0..symbol_size_x {
                        msd += u64::from(img_to_match[i*symbol_size_y + k][j*symbol_size_x + h].abs_diff(symbols_img_vec[symbol_ind][k][h])).pow(2);
                    }
                }
                if msd < best_msd {
                    best_msd = msd;
                    best_symbol_ind = symbol_ind;
                }
            }
            output[i][j] = usize_to_char(symbols_ind_vec[best_symbol_ind]);
        }
    }

    output
}
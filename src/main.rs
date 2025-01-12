use clap::{Parser, ArgGroup};
use image;
use image::{Luma, Rgb};
use std::path::PathBuf;

const DITHER_MATRIX: [u8; 16] = [
    0,  8,  2, 10,
    12,  4, 14,  6,
    3, 11,  1,  9,
    15,  7, 13,  5
];

const SOBEL_KERNEL_X: [f32; 9] = [
    -1.0, 0.0, 1.0,
    -2.0, 0.0, 2.0,
    -1.0, 0.0, 1.0
];
const SOBEL_KERNEL_Y: [f32; 9] = [
    -1.0, -2.0, -1.0,
    0.0, 0.0, 0.0,
    1.0, 2.0, 1.0
];

#[derive(Parser)]
#[clap(group(ArgGroup::new("conflicts").required(false).multiple(false).args(&["downscale", "upscale", "dither", "invert", "trig", "sobel"])))]
#[command(about = "An example of app usage")]
struct Cli {
    #[arg(short, long, value_name = "FILE", help = "Supported formats: bmp, jpeg, png, ico, tga, tiff, webp, exr")]
    filepath: Option<PathBuf>,

    #[arg(short = 'l', long, value_name = "Scale factor", help = "Downscale image by scale factor (only integer value)")]
    downscale: Option<u32>,

    #[arg(short, long, value_name = "Scale factor", help = "Upscale image by scale factor (only integer value)")]
    upscale: Option<u32>,

    #[arg(short = 'd', long, action = clap::ArgAction::SetTrue, help = "Dither image")]
    dither: Option<bool>,
    
    #[arg(short, long, action = clap::ArgAction::SetTrue, help = "Invert image colors")]
    invert: Option<bool>,

    #[arg(short = 't', long, action = clap::ArgAction::SetTrue, help = "Apply sine on image(idk)")]
    trig: Option<bool>,
    
    #[arg(short = 's', long, action = clap::ArgAction::SetTrue, help = "Apply Sobel filter on image")]
    sobel: Option<bool>
}

fn pos_sin(ff: f64, sf: f64) -> f64 {
    if ff * sf.sin() > 0f64 {
        ff * sf.sin()
    } else if ff * sf.sin() < 0f64 {
        ff * sf.sin() * -1f64
    } else {
        0f64
    }
}

fn main() {
    let cli = Cli::parse();
    let fp = cli.filepath.expect("No file path provided");
    if !fp.exists() {
        panic!("wrong file name(file doesn't exists): {:?}", fp)
    }
    else if cli.downscale.is_some() {
        downscale(fp.to_str().unwrap(), cli.downscale.unwrap())
    }
    else if cli.upscale.is_some() {
        upscale(fp.to_str().unwrap(), cli.upscale.unwrap())
    }
    else if cli.dither.unwrap() {
        dither(fp.to_str().unwrap())
    } 
    else if cli.invert.unwrap() { 
        invert_colors(fp.to_str().unwrap())
    }
    else if cli.trig.unwrap() { 
        trig_rofls(fp.to_str().unwrap())
    }
    else if cli.sobel.unwrap() {
        sobel(fp.to_str().unwrap())            
    }
    else {
        panic!("No args provided!")
    }
}

fn downscale(filename: &str, k: u32) {
    let loaded_image = image::open(filename).unwrap().into_rgb16();
    let w= loaded_image.width();
    let h= loaded_image.height();
    let mut img_buff: image::ImageBuffer<Rgb<u16>, Vec<u16>> = image::ImageBuffer::new(w/k, h/k);
    for (x, y, pixel) in img_buff.enumerate_pixels_mut() {
        let current_pixel = loaded_image.get_pixel(x*k, y*k);
        *pixel = Rgb([current_pixel[0], current_pixel[1], current_pixel[2]])
    }
    img_buff.save(filename.to_string() + "_" + &k.to_string() + "x_c.png").unwrap()
}

fn upscale(filename: &str, k: u32) {
    let loaded_image = image::open(filename).unwrap().into_rgb16();
    let w= loaded_image.width();
    let h= loaded_image.height();
    let mut img_buff: image::ImageBuffer<Rgb<u16>, Vec<u16>> = image::ImageBuffer::new(w*k, h*k);
    for (x, y, pixel) in loaded_image.enumerate_pixels() {
        let current_pixel = img_buff.get_pixel_mut(x*k, y*k);
        if pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0 {
            *current_pixel = Rgb([1, 1, 1])
        }
        *current_pixel = Rgb([pixel[0], pixel[1], pixel[2]])
    }
    let mut clone_of_current_buff = img_buff.clone();
    for _ in 0..k {
        for (x, y, pixel) in img_buff.enumerate_pixels_mut() {
            if pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0 {
                if x == 0 {
                    let last_p = clone_of_current_buff.get_pixel(x, y-1);
                    *pixel = *last_p
                } else if y == 0 {
                    let last_p = clone_of_current_buff.get_pixel(x-1, y);
                    *pixel = *last_p
                } else {
                    if x % k == 0 && y % k != 0 {
                        let last_p = clone_of_current_buff.get_pixel(x, y-1);
                        *pixel = *last_p
                    } else if x % k != 0 && y % k == 0 {
                        let last_p = clone_of_current_buff.get_pixel(x-1, y);
                        *pixel = *last_p
                    } else {
                        let last_p = clone_of_current_buff.get_pixel(x-1, y-1);
                        *pixel = *last_p
                    }
                }
            }
        }
        clone_of_current_buff = img_buff.clone()
    }
    img_buff.save(filename.to_string() + "_" + &k.to_string() + "x.png").unwrap()
}

fn dither(filename: &str) {
    let loaded_image = image::open(filename).unwrap().into_rgb8();
    let w= loaded_image.width();
    let h= loaded_image.height();
    let mut img_buff: image::ImageBuffer<Luma<u8>, Vec<u8>> = image::ImageBuffer::new(w, h);
    for (x, y, pixel) in img_buff.enumerate_pixels_mut() {
        let current_pixel = loaded_image.get_pixel(x, y); // 637.5
        let sum_of_pixels: u32 = u32::from(current_pixel[0]) + u32::from(current_pixel[1]) + u32::from(current_pixel[2]);
        let mapped_pixel = ((sum_of_pixels * 255) / 765) as u8;
        let x_d = x % 4;
        let y_d = y % 4;
        let dither_value = DITHER_MATRIX[(y_d * 4 + x_d) as usize] * 16;
        if mapped_pixel < dither_value {
            *pixel = Luma([0])
        }
        else {
            *pixel = Luma([255])
        }
    }
    img_buff.save(filename.to_string() + "_d.png").unwrap()
}

fn invert_colors(filename: &str) {
    let loaded_image = image::open(filename).unwrap().into_rgb16(); // 65025
    let w= loaded_image.width();
    let h= loaded_image.height();
    let mut img_buff: image::ImageBuffer<Rgb<u16>, Vec<u16>> = image::ImageBuffer::new(w, h);
    for (x, y, pixel) in img_buff.enumerate_pixels_mut() {
        let current_pixel = loaded_image.get_pixel(x, y);
        *pixel = Rgb([65535 - current_pixel[0], 65535 - current_pixel[1], 65535 - current_pixel[2]])
    }
    img_buff.save(filename.to_string() + "_i.png").unwrap()
}

fn trig_rofls(filename: &str) {
    let loaded_image = image::open(filename).unwrap().into_rgb8(); // 255
    let w= loaded_image.width();
    let h= loaded_image.height();
    let mut img_buff: image::ImageBuffer<Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(w, h);
    for (x, y, pixel) in img_buff.enumerate_pixels_mut() {
        let current_pixel = loaded_image.get_pixel(x, y);
        let xf = f64::from(x);
        let yf = f64::from(y);
        let rf = f64::from(current_pixel[0]);
        let gf = f64::from(current_pixel[1]);
        let bf = f64::from(current_pixel[2]);
        *pixel = Rgb([(pos_sin(rf, yf) * 1.15) as u8, (pos_sin(gf, xf) * 1.15) as u8, (bf - (((pos_sin(rf, yf) + pos_sin(gf, xf)) / 2f64)) * 1.15) as u8])
    }
    img_buff.save(filename.to_string() + "_t.png").unwrap()
}

fn sobel(filename: &str) {
    let loaded_image = image::open(filename).unwrap().into_rgb8(); // 255
    let w= loaded_image.width();
    let h= loaded_image.height();
    let mut img_buff: image::ImageBuffer<Luma<u8>, Vec<u8>> = image::ImageBuffer::new(w, h);
    for (x, y, pixel) in img_buff.enumerate_pixels_mut() {
        let current_pixel = loaded_image.get_pixel(x, y);
        let sum_of_pixels: u32 = u32::from(current_pixel[0]) + u32::from(current_pixel[1]) + u32::from(current_pixel[2]);
        let mapped_pixel = ((sum_of_pixels * 255) / 765) as u8;
        *pixel = Luma([mapped_pixel])
    }
    let img_buff_gs_copy = img_buff.clone();
    for (x, y, pixel) in img_buff.enumerate_pixels_mut() {
        let mut mag_x = 0.0;
        let mut mag_y = 0.0;
        for i in 0..3 {
            for j in 0..3 {
                if (x != 0 && y != 0) && (x + i - 1 < w && y + j - 1 < h) {
                    let xn = x + i - 1;
                    let yn = y + j - 1;
                    mag_x += img_buff_gs_copy.get_pixel(xn, yn)[0] as f32 * SOBEL_KERNEL_X[(i * 3 + j) as usize];
                    mag_y += img_buff_gs_copy.get_pixel(xn, yn)[0] as f32 * SOBEL_KERNEL_Y[(i * 3 + j) as usize];
                }
            }
        }
        let mag = (mag_x * mag_x + mag_y * mag_y).sqrt();
        // println!("x: {}, y: {}", mag_x * mag_x, mag_y * mag_y);
        *pixel = Luma([mag as u8])
    }
    img_buff.save(filename.to_string() + "_s.png").unwrap()
}
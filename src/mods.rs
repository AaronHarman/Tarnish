// Written By:  Aaron Harman
// Created On:  12/27/2021
// Filename:    mods.rs
// Purpose:     modification code for Tarnish, an image modification program

use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgba};
use image::io::Reader as ImageReader;
use rand::Rng;

pub enum ModResult {
    Ok(DynamicImage),
    Error(String),
    ArgError(String),
}

// Modifications

pub fn copy(img : DynamicImage, _args : Vec<String>) -> ModResult {
    ModResult::Ok(img)
}

pub fn error_test(_img : DynamicImage, _args : Vec<String>) -> ModResult {
    ModResult::Error("This error is intentional, and meant for testing.".to_string())
}

pub fn argerror_test(_img : DynamicImage, _args : Vec<String>) -> ModResult {
    ModResult::ArgError("This argument error is intentional, and meant for testing.".to_string())
}

pub fn hue_rotate(img : DynamicImage, args : Vec<String>) -> ModResult {
    if args.is_empty() {
        return ModResult::ArgError("Requires a number of degrees.".to_string())
    }
    let deg : i32 = match args[0].parse() {
        Ok(i) => i,
        Err(_) => return ModResult::ArgError("Requires a number of degrees.".to_string())
    };
    ModResult::Ok(img.huerotate(deg))
}

pub fn rgb_replace(img : DynamicImage, args : Vec<String>) -> ModResult {

    if args.len() != 3 {
        return ModResult::ArgError("Requires three hex-format colors, one for each R, G, and B.".to_string())
    }

    let mut new_img = img.clone();

    let (rr,rg,rb) : (f32,f32,f32) = match decode_hex(args[0].clone()) {
        Ok(c) => c,
        Err(e) => return ModResult::ArgError(format!("Red hex color improperly formatted: {}.", e)),
    };
    let (gr,gg,gb) : (f32,f32,f32) = match decode_hex(args[1].clone()) {
        Ok(c) => c,
        Err(e) => return ModResult::ArgError(format!("Green hex color improperly formatted: {}.", e)),
    };
    let (br,bg,bb) : (f32,f32,f32) = match decode_hex(args[2].clone()) {
        Ok(c) => c,
        Err(e) => return ModResult::ArgError(format!("Blue hex color improperly formatted: {}.", e)),
    };

    for pixel in img.pixels() {
        let (r, g, b, a) : (u8,u8,u8,u8) = pixel.2.to_rgba().channels4();
        let (nr,ng,nb) : (f32,f32,f32) = (r as f32/255.0, g as f32/255.0, b as f32/255.0);
        new_img.put_pixel(pixel.0, pixel.1, Rgba([(rr*nr+gr*ng+br*nb) as u8, (rg*nr+gg*ng+bg*nb) as u8, (rb*nr+gb*ng+bb*nb) as u8, a]));
    }

    ModResult::Ok(new_img)
}

pub fn mosaic(img : DynamicImage, args : Vec<String>) -> ModResult {
    if args.len() != 1 {
        return ModResult::ArgError("Requires a number of pieces to make the mosaic out of.".to_string())
    }

    let mut new_img = img.clone();
    let (w,h) = img.dimensions();
    let num : u32 = match args[0].parse() {
        Ok(i) => i,
        Err(_) => return ModResult::ArgError("Requires a number of pieces to make the mosaic out of.".to_string())
    };
    let mut points : Vec<(u32,u32)> = Vec::new();

    // generate all the points to center mosaic pieces on
    let mut rng = rand::thread_rng(); //cache thread_rng for better performance
    for _i in 0..num {
        points.push((rng.gen::<u32>()%w, rng.gen::<u32>()%h));
    }

    // make the mosaic itself
    for pixel in img.pixels() {
        // find the closest point to this pixel
        let mut closest : (f64, (u8,u8,u8,u8)) = (-1.0, (0,0,0,0));
        let mut first = true;
        for point in &points {
            let distance : f64 = (((pixel.0 as i32 - point.0 as i32).pow(2) + (pixel.1 as i32 - point.1 as i32).pow(2)) as f64).sqrt();
            if distance < closest.0 || first {
                first = false;
                closest = (distance, img.get_pixel(point.0,point.1).to_rgba().channels4());
            }
        }
        // copy that color and make this pixel that color
        new_img.put_pixel(pixel.0, pixel.1, Rgba([closest.1.0, closest.1.1, closest.1.2, closest.1.3]));
    }

    ModResult::Ok(new_img)
}

pub fn colorize(img : DynamicImage, args : Vec<String>) -> ModResult {
    fn calc_lum(r : u8, g : u8, b : u8) -> f32 {0.2126*(r as f32) + 0.7152*(g as f32) + 0.0722*(b as f32)}
    if args.len() != 1 {
        return ModResult::ArgError("Requires one hex-format color to recolor the image to.".to_string())
    }

    let mut new_img = img.clone();

    let (cr,cg,cb) : (f32,f32,f32) = match decode_hex(args[0].clone()) {
        Ok(c) => c,
        Err(e) => return ModResult::ArgError(format!("Hex color improperly formatted: {}.", e)),
    };
    let (icr, icg, icb) = (255.0-cr, 255.0-cg, 255.0-cb);

    // Calculate luminance of the color chosen
    let cl = calc_lum(cr as u8,cg as u8,cb as u8);
    let icl = 255.0 - cl;

    for pixel in img.pixels() {
        let (r, g, b, a) : (u8,u8,u8,u8) = pixel.2.to_rgba().channels4();
        let l = calc_lum(r,g,b);
        if l > cl { // closer to white
            let d2w = 1.0-(((255.0-l)/icl)); //the percentage of the distance to white the new color should close compared to the chosen color
            new_img.put_pixel(pixel.0, pixel.1, Rgba([(cr+icr*d2w) as u8, (cg+icg*d2w) as u8, (cb+icb*d2w) as u8, a]));
        } else if l < cl { // closer to black
            new_img.put_pixel(pixel.0, pixel.1, Rgba([(cr*l/cl) as u8, (cg*l/cl) as u8, (cb*l/cl) as u8, a]));
        } else /*l == cl*/ {
            new_img.put_pixel(pixel.0, pixel.1, Rgba([cr as u8, cg as u8, cb as u8, a]));
        }
    }

    ModResult::Ok(new_img)
}

pub fn pallettize(img : DynamicImage, args : Vec<String>) -> ModResult {
    if args.len() != 1 {
        return ModResult::ArgError("Requires one file to use as a pallette.".to_string())
    }
    let pal_img = match open_image(args[0].clone()) {
        Ok(n) => n,
        Err(s) => return ModResult::Error(format!("Error with pallette file: {}",s))
    };

    // Grab every unique color
    let mut pal : Vec<(u8,u8,u8,u8)> = Vec::new();
    for pixel in pal_img.pixels() {
        let col = pixel.2.to_rgba().channels4();
        if !pal.contains(&col) {
            pal.push(col);
        }
    }

    // Replace each image color with the closest pallette color
    let mut new_img = img.clone();
    for pixel in img.pixels() {
        let (r, g, b, a) : (u8,u8,u8,u8) = pixel.2.to_rgba().channels4();
        let mut closest : ((u8,u8,u8,u8), f32) = ((0,0,0,0), f32::MAX);
        for col in &pal {
            let dist = (((r as i32-col.0 as i32).pow(2) + (g as i32-col.1 as i32).pow(2) + (b as i32-col.2 as i32).pow(2)) as f32).sqrt();
            if dist < closest.1 {
                closest = (col.clone(), dist);
            }
        }
        new_img.put_pixel(pixel.0, pixel.1, Rgba([closest.0.0, closest.0.1, closest.0.2, a]));
    }

    ModResult::Ok(new_img)
}


// HELPER FUNCTIONS

// getting the numbers out of a hex format color
fn decode_hex(hex : String) -> Result<(f32,f32,f32), &'static str> {
    let r : f32 = u16::from_str_radix(hex.get(0..2).ok_or("Missing Digits")?, 16).or(Err("Not a Hexadecimal Number"))? as f32;
    let g : f32 = u16::from_str_radix(hex.get(2..4).ok_or("Missing Digits")?, 16).or(Err("Not a Hexadecimal Number"))? as f32;
    let b : f32 = u16::from_str_radix(hex.get(4..6).ok_or("Missing Digits")?, 16).or(Err("Not a Hexadecimal Number"))? as f32;
    return Ok((r,g,b))
}

// opening an image
// CHANGE TO RETURN RESULT/OPTION/SOMETHING
fn open_image(name : String) -> Result<DynamicImage, &'static str> {
    match match ImageReader::open(name) {
        Ok(n) => n,
        Err(_) => return Err("Failed to open file.")
    }.decode() {
        Ok(n) => Ok(n),
        Err(_) => return Err("Failed to read file.")
    }
}

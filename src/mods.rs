// Written By:  Aaron Harman
// Created On:  12/27/2021
// Filename:    mods.rs
// Purpose:     modification code for Tarnish, an image modification program

use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgba};

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
    // a helper function for getting the numbers out of a hex format color
    fn decode_hex(hex : String) -> Result<(f32,f32,f32), &'static str> {
        let r : f32 = u16::from_str_radix(hex.get(0..2).ok_or("Missing Digits")?, 16).or(Err("Not a Hexadecimal Number"))? as f32;
        let g : f32 = u16::from_str_radix(hex.get(2..4).ok_or("Missing Digits")?, 16).or(Err("Not a Hexadecimal Number"))? as f32;
        let b : f32 = u16::from_str_radix(hex.get(4..6).ok_or("Missing Digits")?, 16).or(Err("Not a Hexadecimal Number"))? as f32;
        return Ok((r,g,b))
    }

    if args.len() != 3 {
        return ModResult::ArgError("Requires three hex-format colors, one for each R, G, and B.".to_string())
    }

    let mut new_img = img.clone();

    for pixel in img.pixels() {
        let (r, g, b, a) : (u8,u8,u8,u8) = pixel.2.to_rgba().channels4();
        let (nr,ng,nb) : (f32,f32,f32) = (r as f32/255.0, g as f32/255.0, b as f32/255.0);
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
        new_img.put_pixel(pixel.0, pixel.1, Rgba([(rr*nr+gr*ng+br*nb) as u8, (rg*nr+gg*ng+bg*nb) as u8, (rb*nr+gb*ng+bb*nb) as u8, a]));
    }

    ModResult::Ok(new_img)
}

// Written By:  Aaron Harman
// Created On:  12/27/2021
// Filename:    mods.rs
// Purpose:     modification code for Tarnish, an image modification program

use image::io::Reader as ImageReader;
use image::DynamicImage;

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

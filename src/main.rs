// Written By:  Aaron Harman
// Created On:  12/27/2021
// Filename:    main.rs
// Purpose:     argument and flag handling for Tarnish, an image modification program

mod mods;

use std::env;
use std::process::exit;
use std::collections::VecDeque;
use image::io::Reader as ImageReader;
use image::DynamicImage;

fn main() {
    println!("");
    args();
}

fn args() {
    let mut args : VecDeque<String> = env::args().collect();
    args.pop_front(); // get rid of the name of the executable
    // handle flags
    while args.len() > 0 {
        let next = args.pop_front().unwrap();
        if next.starts_with("-") {
            // match statement for flags
        } else {
            args.push_front(next); // put the one we just grabbed back
            break; // move on to arguments
        }
    }
    // grab the two filenames
    let (oldfile, newfile) = (args.pop_front().unwrap(), args.pop_front().unwrap()); // PLEASE PUT IN PROPER ERROR HANDLING HERE
    // grab name of the command
    let command : fn(DynamicImage, Vec<String>)->mods::ModResult = match match args.pop_front() {
        Some(n) => n,
        None => {
            print_error("No Command Name Found");
            exit(1);
        }
    }.as_str() {
        "copy" => mods::copy,
        "errortest" => mods::error_test,
        "argerrortest" => mods::argerror_test,
        "huerotate" => mods::hue_rotate,
        _ => {
            print_error("Invalid Command.");
            exit(1);
        },
    };
    // handle arguments
    //mods::test(args[0].clone(), args[1].clone());
    let mut img = open_image(oldfile);
    img = match command(img, args.into_iter().collect()) {
        mods::ModResult::Ok(i) => i,
        mods::ModResult::Error(s) => {
            print_error(&s);
            exit(1);
        },
        mods::ModResult::ArgError(s) => {
            print_argerror(&s);
            exit(1);
        },
    };
    save_image(img, newfile);

}

fn print_error(text : &str) {
    eprintln!("\u{1b}[1;31mERROR:\u{1b}[0m {}\n",text);
}

fn print_argerror(text : &str) {
    eprintln!("\u{1b}[1;33mARGUMENT ERROR:\u{1b}[0m {}\n",text);
}

fn open_image(name : String) -> DynamicImage {
    match match ImageReader::open(name) {
        Ok(n) => n,
        Err(_) => {
            print_error("Failed to open file.");
            exit(1);
        }
    }.decode() {
        Ok(n) => n,
        Err(_) => {
            print_error("Failed to decode file.");
            exit(1);
        }
    }
}

fn save_image(img : DynamicImage, name : String) {
    match img.save(name.clone()) {
        Ok(_) => {
            println!("\u{1b}[1;32mCOMPLETE:\u{1b}[0m Saved successfully to {}", name);
        },
        Err(_) => {
            print_error("Failed to save file.");
            exit(1);
        }
    }
}

// Copyright (c) 2023 Jacob Allen Morris
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod built;
mod compiler;
mod console;
mod interpreter;
mod reader;
mod writer;

use owo_colors::OwoColorize;
use std::{env, fs, path::PathBuf, process::exit};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let mut non_flag_args: Vec<String> = Vec::new();
    let mut cells: u32 = 30000;
    if args.len() > 1 {
        for i in 0..args.len() {
            if args[i] == "-h" || args[i] == "--help" {
                console::print_main_help();
                exit(0);
            }
        }
        for i in 0..args.len() {
            if args[i] == "-c" || args[i] == "--cells" {
                if args.len() > i + 1 {
                    let value = args[i + 1].parse::<u32>();
                    if value.is_ok() {
                        cells = value.unwrap();
                    } else {
                        console::print_error_message("Non integer value supplied to -c or --cells");
                        exit(0);
                    }
                } else {
                    console::print_error_message("No cell value was supplied");
                    exit(0);
                }
            } else {
                if args[0] != args[i] {
                    non_flag_args.push(args[i].clone());
                }
            }
        }
        if non_flag_args.len() == 2 {
            let input_src = reader::read_src(PathBuf::from(non_flag_args[0].as_str()));
            if input_src.clone().is_err() {
                if input_src.clone().err().unwrap() == "File couldn't be found" {
                    console::print_error_message("File couldn't be found");
                }
            }
            let input_src = input_src.unwrap();
            let interpreted = interpreter::interpret(&input_src);
            let interpreted = interpreter::to_src(interpreted);
            let written_file = writer::write_src_file(interpreted);
            let written_file = written_file.to_str().unwrap();
            let output = compiler::compile_file(written_file, non_flag_args[1].as_str());
            if output.clone().is_err() {
                if output.clone().err().unwrap() == "File couldn't be found" {
                    console::print_error_message("File couldn't be found");
                    exit(0);
                } else if output.clone().err().unwrap() == "Output folder doesn't exist" {
                    console::print_error_message("Output folder doesn't exist");
                    exit(0);
                }
            }
            println!("{}", "Complete!".fg_rgb::<0, 200, 0>());
            exit(0);
        } else if non_flag_args.len() == 1 {
            let input_src = reader::read_src(PathBuf::from(non_flag_args[0].as_str()));
            if input_src.clone().is_err() {
                if input_src.clone().err().unwrap() == "File couldn't be found" {
                    console::print_error_message("File couldn't be found");
                }
            }
            let input_src = input_src.unwrap();
            let interpreted = interpreter::interpret(&input_src);
            let interpreted = interpreter::to_src(interpreted);
            let written_file = writer::write_src_file(interpreted);
            let written_file = written_file.to_str().unwrap();
            let mut output_file = PathBuf::from(non_flag_args[0].clone());
            output_file.set_extension("");
            let output = compiler::compile_file(written_file, output_file.to_str().unwrap());
            if output.clone().is_err() {
                if output.clone().err().unwrap() == "File couldn't be found" {
                    console::print_error_message("File couldn't be found");
                } else if output.clone().err().unwrap() == "Output folder doesn't exist" {
                    console::print_error_message("Output folder doesn't exist");
                }
            }
            fs::remove_file(PathBuf::from(written_file)).unwrap();
            println!("{}", "Complete!".fg_rgb::<0, 200, 0>());
            exit(0);
        } else {
            console::print_error_message("Invalid amount of arguments");
            exit(0);
        }
    } else {
        console::print_main_help();
        exit(0);
    }
}

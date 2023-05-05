// Copyright (c) 2023 Jacob Allen Morris
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use super::built;
use owo_colors::OwoColorize;

pub fn print_main_help() {
    println!(
        "{}{}{}{}",
        "Brainf".fg_rgb::<179, 147, 203>(),
        "**".fg_rgb::<255, 34, 0>(),
        "k Compiler".fg_rgb::<179, 147, 203>(),
        format!(
            " v{}.{}.{}",
            built::built_info::PKG_VERSION_MAJOR,
            built::built_info::PKG_VERSION_MINOR,
            built::built_info::PKG_VERSION_PATCH
        )
        .fg_rgb::<255, 34, 0>()
    );
    println!("{}", " * [Help Section]".fg_rgb::<179, 147, 203>());
    println!("");
    println!(
        "{} {} {} {}",
        " ** bfc".fg_rgb::<179, 147, 203>(),
        "[FLAGS]".fg_rgb::<200, 200, 0>(),
        "<INPUT FILE>".fg_rgb::<0, 200, 0>(),
        "<OUTPUT FILE>".fg_rgb::<0, 200, 0>()
    );
    println!("");
    println!("");
    println!("{}", "** Flags:".fg_rgb::<179, 147, 203>());
    println!(
        "{} {}",
        "    --help or -h:".fg_rgb::<179, 147, 203>(),
        "Get help info".fg_rgb::<200, 200, 0>()
    );
    println!(
        "{} {}",
        "    --cells <CELLS> or -c <CELLS>:".fg_rgb::<179, 147, 203>(),
        "Get help info".fg_rgb::<200, 200, 0>()
    );
    println!(
        "          {}",
        "<CELLS> default is 30,000".fg_rgb::<200, 200, 0>()
    );
}

pub fn print_error_message(error: &str) {
    println!(
        "{}{}{}{}",
        "Brainf".fg_rgb::<179, 147, 203>(),
        "**".fg_rgb::<255, 34, 0>(),
        "k Compiler".fg_rgb::<179, 147, 203>(),
        format!(
            " v{}.{}.{}",
            built::built_info::PKG_VERSION_MAJOR,
            built::built_info::PKG_VERSION_MINOR,
            built::built_info::PKG_VERSION_PATCH
        )
        .fg_rgb::<255, 34, 0>()
    );
    println!("{}", " * [Error]".fg_rgb::<255, 0, 0>());
    println!("    {}", error.to_string().fg_rgb::<200, 200, 0>());
}
// Copyright (c) 2023 Jacob Allen Morris
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use normpath::PathExt;
use std::{
    env, fs,
    path::PathBuf,
    process::{Command, Stdio},
};
use owo_colors::OwoColorize;

#[cfg(target_os = "windows")]
pub fn compile_file(input_file: &str, output_file: &str) -> Result<String, String> {
	println!("{}", "Compiling brainf**k code...".fg_rgb::<200, 0, 0>());
    use std::os::windows::process::CommandExt;

    let input_file = PathBuf::from(input_file);
    let input_file = {
        let tmp = input_file.normalize();
        if tmp.is_err() {
            return Err("File couldn't be found".into());
        } else {
            tmp.unwrap()
        }
    };
    let input_file = input_file.as_path();

    let output_file = PathBuf::from(output_file);
    {
        let mut tmp_output_dir = output_file.clone();
        tmp_output_dir.pop();
        if fs::canonicalize(tmp_output_dir).is_err() {
            return Err("Output folder doesn't exist".into());
        }
    }
    let output_file = output_file.normalize_virtually().unwrap();
    let output_file = output_file.as_path();

    let mut bash_path = PathBuf::from(env::current_exe().unwrap());
    bash_path.pop();
    bash_path.push("msys64\\usr\\bin\\bash.exe");
    let bash_path = bash_path.normalize().unwrap();
    let bash_path = bash_path.as_path();

    let executing = Command::new("cmd.exe")
        .creation_flags(0x08000000)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("/k")
        .arg(
            format!(
                "{} -c 'PATH=/usr/bin:/mingw64/bin ; gcc {} -Os -o {} ; strip {}.exe'",
                bash_path.display().to_string(),
                input_file.display().to_string().replace("\\", "/"),
                output_file.display().to_string().replace("\\", "/"),
                output_file.display().to_string().replace("\\", "/")
            )
            .as_str(),
        )
        .output()
        .unwrap();
    let ostdout = String::from_utf8(executing.stdout).unwrap();
    Ok(ostdout.clone())
}

#[cfg(target_os = "linux")]
pub fn compile_file(input_file: &str, output_file: &str) -> Result<String, String> {
	println!("{}", "Compiling brainf**k code...".fg_rgb::<200, 0, 0>());
    let input_file = PathBuf::from(input_file);
    let input_file = {
        let tmp = input_file.normalize();
        if tmp.is_err() {
            return Err("File couldn't be found".into());
        } else {
            tmp.unwrap()
        }
    };
    let input_file = input_file.as_path();

    let mut output_file = PathBuf::from(output_file);
    let output_file = {
        let mut tmp_output_dir = output_file.clone();
        tmp_output_dir.pop();
        if fs::canonicalize(tmp_output_dir.clone()).is_err() {
            return Err("Output folder doesn't exist".into());
        }
        let mut tmp_output_file = tmp_output_dir.clone().normalize().unwrap();
        let mut tmp_output_file = tmp_output_file.into_path_buf();
        let tmp_output_file_name = output_file.as_path();
        let tmp_output_file_name = tmp_output_file_name.file_name();
        tmp_output_file.push(tmp_output_file_name.unwrap().to_owned());
        tmp_output_file
    };

    let executing = Command::new("/bin/bash")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&[
            "-c",
            format!(
                "gcc {} -Os -o {} -lncurses ; strip {}",
                input_file.display(),
                output_file.display(),
                output_file.display()
            )
            .as_str(),
        ])
        .spawn()
        .unwrap();

    let output = executing.wait_with_output();
    Ok(String::from_utf8(output.unwrap().stdout).unwrap())
}

#[cfg(all(not(target_os = "windows"), not(target_os = "linux")))]
pub fn compile_file() {}

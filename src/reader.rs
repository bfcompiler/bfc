// Copyright (c) 2023 Jacob Allen Morris
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{fs::File, io::Read, path::PathBuf};

pub fn read_src(src: PathBuf) -> Result<String, String> {
	let input = File::open(src);
	if input.is_err() {
		return Err("File couldn't be found".into());
	}
	let mut input = input.unwrap();
	let mut output = String::new();
	input.read_to_string(&mut output).unwrap();
	Ok(output)
}
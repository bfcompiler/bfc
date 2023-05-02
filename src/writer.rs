// Copyright (c) 2023 Jacob Allen Morris
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{io::Write, fs::File, env::temp_dir, path::PathBuf};
use rand::Rng;

pub fn write_src_file(src: String) -> PathBuf {
	let mut tmp_file = temp_dir();
	let mut rng = rand::thread_rng();
	let mut file_name = String::new();
	for i in 0..32 {
		if rng.gen::<bool>() {
			let letter = rng.gen_range(b'A' as u8..b'Z') as char;
			file_name.push(letter);
		} else {
			let letter = rng.gen_range(b'a' as u8..b'z') as char;
			file_name.push(letter);
		}
	}
	file_name.push_str(".c");
	tmp_file.push(file_name);
	let mut output_file = File::create(&tmp_file).unwrap();
	output_file.write_all(src.as_bytes()).unwrap();
	tmp_file
}
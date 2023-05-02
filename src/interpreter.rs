// Copyright (c) 2023 Jacob Allen Morris
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use super::built;

static INTERPRETER_SRC: &str = include_str!("../resources/interpreter.c");

#[derive(Debug, Clone)]
pub struct InterpretedData {
	pub src: String,
	pub src_as_array_str: String,
	pub src_as_bytes: Vec<u8>,
	pub cell_count: u32,
	pub loop_size: u32
}

impl InterpretedData {
	pub fn new() -> InterpretedData {
		InterpretedData {
			src: "".into(),
			src_as_array_str: "".into(),
			src_as_bytes: Vec::new(),
			cell_count: 30000,
			loop_size: 512
		}
	}
}

pub fn interpret(src: &str) -> InterpretedData {
	let mut data = InterpretedData::new();
	data.src = src.clone().into();
	let mut raw_bytes = Vec::new();
	let mut raw_bytes_as_str = String::new();
	for i in 0..src.len() {
		let elem = String::from(&src[i..i + 1]);
		let elem = elem.as_str();
		match elem {
			">" => {
				raw_bytes.push(0);
				raw_bytes_as_str.push_str("0, ");
			},
			"<" => {
				raw_bytes.push(1);
				raw_bytes_as_str.push_str("1, ");
			},
			"+" => {
				raw_bytes.push(2);
				raw_bytes_as_str.push_str("2, ");
			},
			"-" => {
				raw_bytes.push(3);
				raw_bytes_as_str.push_str("3, ");
			},
			"[" => {
				raw_bytes.push(4);
				raw_bytes_as_str.push_str("4, ");
			},
			"]" => {
				raw_bytes.push(5);
				raw_bytes_as_str.push_str("5, ");
			},
			"." => {
				raw_bytes.push(6);
				raw_bytes_as_str.push_str("6, ");
			},
			"," => {
				raw_bytes.push(7);
				raw_bytes_as_str.push_str("7, ");
			},
			_ => {}
		}
	}
	let raw_bytes_as_str = &raw_bytes_as_str[0..raw_bytes_as_str.len() - 2];
	let raw_bytes_as_str = raw_bytes_as_str.into();
	data.src_as_bytes = raw_bytes;
	data.src_as_array_str = raw_bytes_as_str;
	data
}

pub fn to_src(data: InterpretedData) -> String {
	let output = INTERPRETER_SRC.to_string();
	let output = output.replace("#define DATA_SIZE 0", format!("#define DATA_SIZE {}", data.src_as_bytes.len()).as_str());
	let output = output.replace("#define RAW_DATA {0}", format!("#define RAW_DATA {{{}}}", data.src_as_array_str).as_str());
	let output = {
		let mut ret = output.clone();
		if built::built_info::CFG_OS != "windows" {
			ret = output.replace("#define IS_WINDOWS 1", "#define IS_WINDOWS 0");
		}
		ret
	};
	output
}
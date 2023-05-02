// Copyright (c) 2023 Jacob Allen Morris
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

fn main() {
	built::write_built_file().expect("Failed to acquire build-time information");
}
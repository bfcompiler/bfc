// Copyright (c) 2023 Jacob Allen Morris
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub mod built_info {
   include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
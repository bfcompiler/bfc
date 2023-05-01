#!/bin/bash
# Copyright (c) 2023 Jacob Allen Morris
# 
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

if [[ ! -d "output" ]] ; then
	mkdir output
fi

cp target/x86_64-unknown-linux-gnu/release/bfc output/bfc-x86_64-unknown-linux-gnu
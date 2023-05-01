#!/bin/bash
# Copyright (c) 2023 Jacob Allen Morris
# 
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

if [[ ! -d "output" ]] ; then
	mkdir output
fi

cp target/i686-unknown-linux-gnu/release/bfc output/bfc-i686-unknown-linux-gnu
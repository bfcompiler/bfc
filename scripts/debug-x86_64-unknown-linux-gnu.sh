#!/bin/bash
# Copyright (c) 2023 Jacob Allen Morris
# 
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

if [[ ! -d "output" ]] ; then
	mkdir output
fi
if [[ ! -d "output/debug" ]] ; then
	mkdir output/debug
fi
cp target/x86_64-unknown-linux-gnu/debug/bfc output/debug/bfc
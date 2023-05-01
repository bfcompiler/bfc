# Copyright (c) 2023 Jacob Allen Morris
# 
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

if (!(Test-Path -Path "output")) {
	New-Item -Path "output" -ItemType Directory
}
if (!(Test-Path -Path "output/debug")) {
	New-Item -Path "output/debug" -ItemType Directory
}
Copy-Item -Path "target/x86_64-pc-windows-msvc/debug/bfc.exe" -Destination "output/debug/bfc.exe"
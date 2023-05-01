# Copyright (c) 2023 Jacob Allen Morris
# 
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

if (!(Test-Path -Path "output")) {
	New-Item -Path "output" -ItemType Directory
}
Copy-Item -Path "target/i686-pc-windows-msvc/release/bfc.exe" -Destination "output/bfc-i686-pc-windows-msvc.exe"
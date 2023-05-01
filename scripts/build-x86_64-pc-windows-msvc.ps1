# Copyright (c) 2023 Jacob Allen Morris
# 
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

if (!(Test-Path -Path "output")) {
	New-Item -Path "output" -Type Directory
}
Copy-Item -Path "target/x86_64-pc-windows-msvc/release/bfc.exe" -Destination "output/bfc-x86_64-pc-windows-msvc.exe"
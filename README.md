# Corvidae 
Quick Command Line File Encryption with Steganometric Capabilities

## Intended Use
- Mess around and have fun 
- Hide Stuff
- Secret messages to your friends
- Use it on ur .env secrets file and then upload it to github and nobody will know!
## Installation
For installation it is Important you have Rust & Cargo installed on your machine so you can compile from source 
- [rust & cargo](https://www.rust-lang.org/tools/install)

Once you have Rust and Cargo compile the program using `cargo build --release` you should copy the absolute path of `target/release/corvidae` and alias it under whatever name you prefer in the terminal `alias corvidae="~/target/release/corvidae"`

## Methods
The current realease on has two methods *there will be more soon*
- "-e" Encrypts a file
- "-d" De-Encrypts a file
- "-p" Embeds file contents into PNG
- "-g" Extracts PNG contents into a file
#### The methods are called as shown below:
`corvidae pa$sword_no_spaces_allowed123@# fileToModify.txt -e`
> you must be in the same directory as the file or state and absolute path for things to work properly, also the password shouldn't be longer than the file you are encrypting *inputing the incorrect password to decrypt can cause the program to fail this is intended Behavior






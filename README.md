[![Build Status](https://travis-ci.org/ItinoseSan/ru-gnu-core-utils.svg?branch=master)](https://travis-ci.org/ItinoseSan/ru-gnu-core-utils)
# ru-gnu-core-utils
Implementation of a part of GNU Core Utilities in rust-lang
# Usage
### 1.clone this repository and move to the directory.
```
git clone git@github.com:ItinoseSan/ru-gnu-core-utils.git
```
```
cd ru-gnu-core-utils
```
### 2.Set up use crate
```
cargo build
```
### 3.Run src
 This time, I implemated ```pwd, ls, mkdir, echo``` :)
```
cargo run
```
# Demo
```
mkdir
Error: Position(Alpha, [10])
mkdir test
ls
./.git
./Cargo.toml
./.vscode
./.gitignore
./Cargo.lock
./src
./test
./target
pwd
/home/username/my_projects/rust_app/linux-gnu-core-util
```
# Help
I need to good implementation. Please tell me.

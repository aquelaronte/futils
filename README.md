# File utils

This project provides utilities for file management via the terminal. It includes basic utilities for creating, searching, and managing files and folders.

## Features
- Show files and folders
- Search files and folders on a specified path
- Create files and folders

## Installation
### Requirements
To install futils, ensure the following requirements are met:
- Cargo's environment variable should be set in your shell configuration.

To install futils, run the following commands:
```bash
git clone https://github.com/aquelaronte/futils.git
cd futils/
./install.sh
```
The install.sh script sets up the necessary environment and installs futils in your system.

## Usage
See fs --help
```bash
Arguments:
  [PATH]  [default: ./]

Options:
  -r, --regex <REGEX>          [default: .*]
  -s, --search <SEARCH>        [default: ]
  -f, --file-type <FILE_TYPE>  [default: *]
      --add <ADD>              [default: ]
  -h, --help                   Print help
```
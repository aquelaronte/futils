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

### Examples
```bash
# this command will list all files from cwd
fs

# this command will retrieve all files that contains word "Car" from cwd (examples: Cargo.toml, rawCar.js, wildCards/)
fs -s Car

# this command will create a folder and list files from cwd (new folder or file included)
fs --add folder/

# this command will list all files and folders from ../../
fs ../../

# this command will list all files and folders that contains word "Car" from ../../
fs ../../ -S Car

# this command will create a file named app.ts inside src/ folder and will list all json files from src/ folder that containts "ot" on its name (example: notes.json, root.json)
fs src --add app.ts -f json -S ot
```

### Important
`--add` flag doesn't create folders recursively, by example: `fs src/config --add init.ts` will not works if config folder is not created, also `fs src --add config/init.ts` will not work in that case too

## Program Flow

When the user executes fs, it follows a specific flow to list files:
1. Listing Files: Initially, fs retrieves all files from the specified path. If no path is provided by the user, it defaults to ./.
	2.	Adding Files or Folders:
	•	If the user uses the --add flag, fs creates a folder or a file. To create a folder, append a slash / at the end of the folder’s name. The created file is then added to the list of files retrieved in step 1. If the flag’s argument is empty, fs does not create anything.
	3.	Filtering by Regex (-r or --regex):
	•	If the user specifies the -r or --regex flag, fs filters out files whose names do not match the provided regular expression. By default, this regex is .*.
	4.	Filtering by Search (-s or --search):
	•	When the -s or --search flag is used, fs applies a case-sensitive regular expression (.*<SEARCH>.*) to filter files by their names. Files that do not match this criterion are removed from the list filtered in step 3.
	5.	Filtering by File Type (-f or --file-type):
	•	If the -f or --file-type flag is used, fs compares the argument of the flag with the file extensions of the remaining files after steps 3 and 4.

In summary, the flow of fs can be summarized as follows:
	•	Files created using the --add flag are included in the list of files processed by fs.
	•	If a created file’s name does not match any of fs’s filters, it is excluded from further processing.
	•	The sequence of operations is: add -> regex -> search -> file type -> files list.

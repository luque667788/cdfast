#!/bin/bash
cdf () {
# Call the Rust program
$HOME/cdfast/cdtree

# Read the directory path from the file
new_dir=$(<$HOME/cdfast/selected_directory.txt)

# Change the directory to the one read from the file
cd "$new_dir"
echo "$new_dir"
}


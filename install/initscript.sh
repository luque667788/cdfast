#!/bin/bash
cdf () {
# Call the Rust program
$HOME/prog/rust/cdtree/target/debug/cdtree

# Read the directory path from the file
new_dir=$(<$HOME/prog/rust/cdtree/target/debug/selected_directory.txt)

# Change the directory to the one read from the file
cd "$new_dir"
echo "$new_dir"
}


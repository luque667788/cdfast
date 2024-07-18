#!/bin/bash

# Create the cdfast folder in the home directory
echo "Creating cdfast folder..."
mkdir ~/cdfast

# Copy the cdtree and script.sh files to cdfast
echo "Copying files to home/cdfast..."
cp initscript.sh cdtree ~/cdfast
chmod +x ~/cdfast/cdtree
chmod +x ~/cdfast/initscript.sh

# Add sourcing of initscript.sh to .bashrc
echo "Updating .bashrc..."
if ! grep -q "source ~/cdfast/initscript.sh" ~/.bashrc; then
    echo "source ~/cdfast/initscript.sh" >> ~/.bashrc
fi

# Source .bashrc to save the changes
echo "Reloading .bashrc..."
source ~/.bashrc

echo "setup.sh has been executed successfully"
echo "cdfast has been installed in the home directory"
echo "To use cdfast, type 'cdf' in the terminal"
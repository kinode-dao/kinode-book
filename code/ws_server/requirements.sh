#!/bin/bash

# Check if pip is installed and install it if it is not.
if ! command -v pip &> /dev/null
then
    echo "pip could not be found, installing..."
    python -m ensurepip --upgrade
fi

# Update pip to the latest version.
python -m pip install --upgrade pip

# Install packages from the requirements.txt file.
if [ -f "requirements.txt" ]; then
    echo "Installing packages from requirements.txt..."
    pip install -r requirements.txt
else
    echo "Error: requirements.txt does not exist."
    exit 0
fi

echo "Package installation complete."

#!/bin/bash

crossplatform_realpath_inner() {
    python -c "import os; print(os.path.realpath('$1'))"
}

crossplatform_realpath() {
    if [ -e "$1" ] || [ -L "$1" ]; then
        crossplatform_realpath_inner "$1"
    else
        return 1
    fi
}

# Check if pip is installed and install it if it is not.
if ! command -v pip &> /dev/null
then
    echo "pip could not be found, installing..."
    python -m ensurepip --upgrade
fi

# Update pip to the latest version.
python -m pip install --upgrade pip

# Install packages from the requirements.txt file.
script_dir=$(dirname "$(crossplatform_realpath "${BASH_SOURCE[0]}")")
requirements_path="${script_dir}/requirements.txt"
if [ -f "$requirements_path" ]; then
    echo "Installing packages from requirements.txt..."
    pip install -r $requirements_path
else
    echo "Error: requirements.txt does not exist."
    exit 0
fi

echo "Package installation complete."

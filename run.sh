#!/bin/zsh

echo "\033[0;32mRunning the file analysis of directory: $1"

find ~/Desktop -type f -exec ls -la {} \; | awk '{print $9 "::" $5}' | python3 py/main.py -f output.pdf | cargo run --quiet -- --parts 10 > analysis.txt

echo "\033[0;32mAnalysis of directory $1 is done. The output is in analysis.txt and output.pdf"


echo "\033[0;35mShowing the output.pdf"
cat analysis.txt

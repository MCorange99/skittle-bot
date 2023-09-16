#!/usr/bin/bash
#!
#! Counts lines of code in the whole project. for fun :p
#!

if [ -x "$(command -v cloc)" ]; then
    cloc --exclude-dir=target,.vscode,.git .
else 
    echo "cloc doesnt exist, please install it"
    exit 1
fi
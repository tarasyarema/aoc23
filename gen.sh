#!/bin/bash

if [ -e "$1" ]; then
    echo "Folder for day $1 already exists, GTFO"

    exit 0
fi

echo "Creating folder for day $1"

mkdir $1
cd $1

cargo init --name aoc23-$1
cp ../template.rs src/main.rs

echo "Donillo!"

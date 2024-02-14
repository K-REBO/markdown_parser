#!/bin/bash


cargo fmt
echo -e "\n" >> progress.txt
date >> progress.txt
tokei >> progress.txt

#!/bin/bash

# Z1
directory=$1
mkdir $directory

cd $directory
mkdir dir1
mkdir dir1/dir1.1
touch dir1/text1.txt
mkdir dir2
mkdir dir2/dir2.1/
touch dir2/dir2.1/file1.csv
mkdir dir3
mkdir dir3/dir3.1/
mkdir dir3/dir3.2/
touch dir3/dir3.2/file2.csv
mkdir Aa
mkdir Bb
mkdir Cc

mv dir1/text1.txt dir1/dir1.1/
cp dir2/dir2.1/file1.csv dir1/
mv Aa Aa1
ls -d *[12]
du -d 1
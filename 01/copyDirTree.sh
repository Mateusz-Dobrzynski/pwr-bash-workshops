#!/bin/bash

# Z2
sourceDirectory=$1
destinationDirectory=$2

cp -r $sourceDirectory $destinationDirectory
cd $destinationDirectory
ls -l
tree -s
pwd
cd -
rm -rf $destinationDirectory
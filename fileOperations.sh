#!/bin/bash

randomLinesAssignment () {
    source=$1
    dest1=$2
    dest2=$3
    if [ ! -r $source ]; then
    # -r – check if user has read permissions for the file
        echo "File $source is not readable" >&2
        exit -1
    fi
    if [ ! -e  $dest1 ]; then
    # -e – check if file exists
        touch $dest1
    elif [ ! -w $dest1 ]; then
        echo "File $dest1 is not writeable" >&2
        exit -1
    else
        echo "" > $dest1
    fi
    if [ ! -e  $dest2 ]; then
        touch $dest2
    elif [ ! -w $dest2 ]; then
        echo "File $dest2 is not writeable" >&2
        exit -1
    else
        echo "" > $dest2
    fi

    while IFS= read -r line; do # iterate through lines of a file
        file=$(("$RANDOM % 2 + 1"))
        if [ $file == "1" ]; then
            echo $line >> $dest1
        else
            echo $line >> $dest2
        fi
    done < "$source"
}

directioryCreation () {
    directory=$1
    treeFile=$2
    if [ ! -d $directory ]; then
        echo "$1 is not a directory" >&2
        exit -1
    elif [ ! -w $directory ]; then
        echo "$1 is not writeable" >&2
        exit -1
    fi
    if [ ! -r $treeFile ]; then
        echo "$2 is not readable" >&2
        exit -1
    fi

    multipleDirsPattern='\/'
    while IFS= read -r line; do
        if [[ $line =~ $multipleDirsPattern ]]; then
            mkdir -p "$directory/$line"
        else
            mkdir "$directory/$line"
            touch "$directory/$line/file.txt"
        fi
    done < "$treeFile"
    
    for subdir in $(find $directory -type d); do
        touch $subdir/file.txt
    done

    tree $directory
}

randomLinesAssignment $1 $2 $3
directioryCreation $1 $2
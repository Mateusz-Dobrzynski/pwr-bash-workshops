#!/bin/bash

fileStats() {
    inputFile=$1
    outputFile=$2
    checkFilePrivileges $inputFile $outputFile
    words=$(cat $inputFile | tr "[[:blank:]]" '\n' | sort | tr '\n' ' ')
    countWordOccurrences "$words" > $outputFile
}

checkFilePrivileges() {
    inputFile=$1
    outputFile=$2
    if [ ! $# == 2 ]; then
        echo "Invalid arguments count (must me 2)" >&2
        exit -1
    fi
    if [ ! -r $inputFile ]; then
        echo "$inputFile is not readable or does not exist" >&2
        exit -1
    fi
    if [ -e $outputFile ]; then
        if [ ! -w $outputFile ]; then
            echo "$outputFile is not writeable" >&2
            exit -1
        fi
    else
        if touch $outputFile; then 
            continue
        else
            echo "$outputFile cannot be created" >&2
            exit -1
        fi
    fi
}

countWordOccurrences() {
    words=$(echo $1 | tr ' ' '\n')
    uniqueWords=$(echo $words | tr ' ' '\n' | sort -u | tr '\n' ' ')
    declare -A associativeArray 
    for unique in $uniqueWords; do
        if [[ $unique =~ \(|\) ]]; then
            unique=$(echo $unique | tr '(' '\\(')
            unique=$(echo $unique | tr ')' '\\)')
        fi
        pattern="(?<=^|\s)$unique(?=\s|$)"
        wordCount=$(echo $words | grep -oP $pattern | wc -l)
        associativeArray+=(["$unique"]="$wordCount")
    done
    for entry in "${!associativeArray[@]}"; do
        echo "$entry" "${associativeArray[$entry]}";
    done | sort -rn -k2
}

fileStats $1 $2
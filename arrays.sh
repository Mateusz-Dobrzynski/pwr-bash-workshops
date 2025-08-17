#!/bin/bash

bubbleSort () {
    tablica=()
    intPattern='^[0-9]+$'
    for element in $@
    do 
        if ! [[ $element =~ $intPattern ]] ; then
            echo 'Arguments must be integers'
            exit -1
        fi
        tablica+=("$element") # Append to array
    done
    for ((i=0; $i < $# ; i++));
    do
        for ((j=0; $j < $# ; j++));
        do
            if [[ ${tablica[$j]} -gt ${tablica[$i]} ]]; then
                temp=${tablica[$i]}
                tablica[$i]=${tablica[$j]}
                tablica[$j]=$temp
            fi
        done
    done
    echo ${tablica[@]} # print all array elements
}

multiplicationTab () {
    if [ $# != 2 ]; then
        echo "Invalid arguments count (2 required)" >&2
        exit -1
    fi
    intPattern='^[0-9]+$'
    for element in $@
    do 
        if ! [[ $element =~ $intPattern ]] ; then
            echo 'Arguments must be positive integers'
            exit -1
        fi
    done
    mainArray=()
    for ((i=0; i <= $1; i++)); do
        array=()
        for ((j=0; j <= $2; j++)); do
            multiplication=$((i * j))
            array+=($multiplication)
        done
        mainArray+=($array)
        echo ${array[@]}
    done
}

dictMultiplicationTab () {
    if [ $# != 2 ]; then
        echo "Invalid arguments count (2 required)" >&2
        exit -1
    fi
    intPattern='^[0-9]+$'
    for element in $@
    do 
        if ! [[ $element =~ $intPattern ]] ; then
            echo 'Arguments must be positive integers'
            exit -1
        fi
    done
    declare -A mainArray # declare an associative array
    for ((i=0; i <= $1; i++)); do
        array=()
        for ((j=0; j <= $2; j++)); do
            multiplication=$((i * j))
            array+=($multiplication)
        done
        mainArray+=(["$i"]="${array[@]}")
        echo ${array[@]}
    done   
}

integration () {
    if [ $# -lt 3 ]; then
        echo "Invalid arguments count (3 required)" >&2
        exit -1
    fi
    realPattern='^[0-9]+(\[.,][0-9]+)?$'
    for element in $@
    do 
        if ! [[ $element =~ $realPattern ]] ; then
            echo 'Arguments must be real numbers'
            exit -1
        fi
    done
    integral=0
    for ((i=1; i<$#; i++)); do
        j=$((i+1))
        area=$(echo "(${!i} + ${!j}) / 2" | bc -l)
        integral=$(echo $area + $integral | bc -l)
    done
    echo $integral
}

bubbleSort $@
echo ''
dictMultiplicationTab $1 $2
echo ''
integration $@
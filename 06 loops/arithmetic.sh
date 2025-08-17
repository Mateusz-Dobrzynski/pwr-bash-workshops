#!/bin/bash

ulimit -u 2000 # failsafe in case of a bad recursion

division () {
    if [ $# != 2 ]; then # $# – number of arguments passed to this function
        echo "Invalid arguments count (2 required)" >&2
        # >&2 – write to stderr
        exit -1
    fi
    numerator=`echo "$1 + $2" | bc`
    denominator=`echo "$1 - $2" | bc`
    if [ $denominator == 0 ]; then
        echo "Denominator ($1 - $2) must not equal zero" >&2
        exit -1
    fi
    echo "$numerator / $denominator" | bc
}

# bc – program parsing strings as math expressions

factorialRec () {
    if [ $# != 1 ]; then
        echo "Invalid arguments count (1 required)" >&2
        exit -1
    fi
    number=$1
    if [ $number == 1 -o $number == 0 ]; then
        echo 1
        return 0
    else
        prev=$(factorialRec $(($number - 1)))
        echo $(($number * $prev))
    fi
}

# $((expression)) – native syntax for parsing math expressions

factorialIter () {
    if [ $# -ne 1 ]; then
        echo "Invalid arguments count (1 required)" >&2
        exit -1
    fi
    factorial=$1
    if [ $number == 1 -o $number == 0 ]; then
        echo 1
        return 0
    fi
    for ((i=($1-1); i>=1; i--)); do
        factorial=$(($factorial * $i))
    done
    echo $factorial
}

sumArgs () {
    sum=0
    for i in $@; do
        int_pattern='^[0-9]+$'
        if ! [[ $i =~ $int_pattern ]] ; then # =~ – bash operator for regex matching
            echo "error: $i is not an int" >&2
            exit -1
        fi
        sum=$(($sum + $i));
    done
    echo $sum
}



division $1 $2 
factorialRec $1
factorialIter $1
sumArgs $@

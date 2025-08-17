searchedFile=$1

# 1
# Find valid hexadecimal numbers
grep -Po '(?<=[ \n]|[^+-]|^)0x[A-F0-9a-f]+(?=[, ]+|\n|$)' $searchedFile


# 2
# Find valid email addresses
grep -Po '(?<=[ ]|\n|^)+[A-Za-z,.;:0-9]+@([a-zA-Z-0-9]+\.)+[a-zA-Z-0-9]+' $searchedFile

# 3 Find valid floating point numbers (including shorthands like .1)
grep -Po '[-+]?\d*\.\d+' $searchedFile


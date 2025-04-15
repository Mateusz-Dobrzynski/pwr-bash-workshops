search_dir=$1
exec_dir=$2

# 1
# Create a directory
echo 1
mkdir $exec_dir

# 2
# Find files with filenames ending with '.txt'
echo 2
find $search_dir -type f -name '*.txt'

# 3
# Find directories with names starting with 'A' or 'B'
# (case insensitive)
echo 3
find $search_dir -type d \( -iname 'A*' -o -iname 'B*'  \)

# 4
# Find files with execution permissions
# and ending with '.sh'
echo 4
find $search_dir -type f -name '*.sh' -perm /111

# 5
# Find empty files owned by student
echo 5
find $search_dir -type f -empty -user student

# 6
# Find symlinks
echo 6
find $search_dir -type l

# 7
# Find files owned by group student
# with size below 1 MB
echo 7
find $search_dir -type f -group student -size -1M

# 8
# Find executable files with
# either SUID or SGID set up
echo 8
find $search_dir -type f \( -perm /2000 -o -perm /4000 \) -perm /111

# 9
# Find directories with sticky bit set up
echo 9
find $search_dir -type d -perm -1000

# 10
# Find files modified within last hour
echo 10
find $search_dir -type f -mmin -60

# 11
# Find block or character devices
echo 11
find /dev -type c -o -type b

# 12
echo 12
# Find and delete all empty directories
find $search_dir -type d -empty -exec rm -rd {} \;

# 13
# Find files with size exceeding 1 MB
# and move them to $exec_dir
echo 13
find $search_dir -type f -size +1M -exec mv {} $exec_dir \;

# 14
# Find directories with names starting with 'Z'
# and copy them recursively to $exec_dir
echo 14
find $search_dir -type d -name 'Z*' -exec cp -R {} $exec_dir \;
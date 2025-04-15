#!/bin/bash

# Z1
mkdir $1

# Z2
cd $1
mkdir D1
mkdir D2
touch D2/F1.txt
mkdir D3
touch D3/F1.txt
mkdir D4
touch D4/F1.txt

# Z3
ln -s /etc/passwd D1

# Z4
readlink D1/passwd

# Z5
ln D3/F1.txt D2/F2.txt

# Z6
chmod 600 D2/F2.txt

# Z7
chown student D2/F2.txt

# Z8
chgrp student D3

# Z9
chmod a-x D3
cd D3

# Z10
chmod a-w D2
touch D2/X.txt

# Z11
chmod a-r D4
ls D4

# Z12
chmod +t D4

# Z13
touch D1/scr1.sh
chmod u+x D1/scr1.sh
chmod g+x D1/scr1.sh
chmod o-x D1/scr1.sh

# Z14
chmod g+s D1/scr1.sh
chmod u+s D1/scr1.sh
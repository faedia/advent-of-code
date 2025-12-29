day=$1

gfortran ${day}/${day}.f90 -o ${day}/${day} -O3 -g

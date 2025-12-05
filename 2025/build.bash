day=$1

flang-20 ${day}/${day}.f90 -o ${day}/${day} -O3 -g

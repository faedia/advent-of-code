day=$1

pushd ${day}
sudo perf record -F max -g -- ./${day}
sudo perf script -f | ../../../FlameGraph/stackcollapse-perf.pl --inline --srcline --context > out.perf-folded
../../../FlameGraph/flamegraph.pl --title "${day} Flame Graph" --countname "samples" out.perf-folded > ${day}_flamegraph.svg

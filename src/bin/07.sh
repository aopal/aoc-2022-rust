#!/bin/bash

input_file=$(realpath $1)

rm -r day07
mkdir -p day07
cd day07
pushd "."

# build directory structure
while read line; do
  is_cd=$(echo "$line" | egrep -o "^\\$ cd [^/]+")
  is_dir=$(echo "$line" | egrep -o "^dir")
  fsize=$(echo "$line" | egrep -o "^\d+")
  fname=$(echo "$line" | egrep -o "\S+$")

  if [[ ! -z "$is_dir" ]]; then
    echo "creating dir $fname"
    mkdir $fname
  elif [[ ! -z "$is_cd" ]]; then
    cd $fname
    echo "cd-ing to $(pwd)"
  elif [[ ! -z "$fsize" ]]; then
    echo "writing file $fname"
    perl -E "say 'a' x ($fsize-1)" > $fname
  fi
done < $input_file

popd

# answer problems
part_one_size="0"
total_size=$(ls -lR | grep "^-" | awk '{ total += $5 }; END { print total }')
size_to_free=$((30000000 - 70000000 + $total_size))
smallest_sufficient_free="$total_size"

for dir in $(ls -lR | egrep -o "\./[^:]+"); do
  size=$(ls -lR $dir | grep "^-" | awk '{ total += $5 }; END { print total }')

  if (( $size <= 100000 )); then
    part_one_size=$((part_one_size + size))
  fi

  if (( $size >= $size_to_free && $size < $smallest_sufficient_free )); then
    smallest_sufficient_free=$size
  fi
done

echo "part 1: $part_one_size"
echo "part 2: $smallest_sufficient_free"

#!/bin/bash

in_dir_recipes="intermediate/md/recipes"
out_dir_recipes="intermediate/html/recipes"

build () {
  filename="$(basename -- $1)"
  echo "($3/$4) building $filename"
  out_file=$(echo "$2/$filename" | sed 's/md/html/g')
  pandoc -f markdown -t html "$file" -o "$out_file" --section-divs=true
}

build_dir () {
  dir_num=$(ls -l $1 | wc -l)
  dir_cnt=1
  for file in $1/*.md
  do 
    build $file $2 $dir_cnt $dir_num
    dir_cnt=$((dir_cnt+1))
  done
}

echo "bulding recipes"
build_dir $in_dir_recipes $out_dir_recipes

#!/bin/bash

in_dir_recipes="intermediate/md/recipes"
in_dir_tags="intermediate/md/tags"
out_dir_recipes="intermediate/html/recipes"
out_dir_tags="intermediate/html/tags"

build () {
  filename="$(basename -- $1)"
  echo "building $filename $3/$4"
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
echo "building tags"
build_dir $in_dir_tags $out_dir_tags

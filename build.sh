#!/bin/bash

in_dir_recipes="intermediate/recipes"
in_dir_tags="intermediate/tags"
out_dir_recipes="html/recipes"
out_dir_tags="html/tags"
css_file="../main.css"

build () {
  filename="$(basename -- $1)"
  echo "building $filename"
  out_file=$(echo "$2/$filename" | sed 's/md/html/g')
  pandoc -f markdown -t html "$file" -o "$out_file" --css="$css_file" --standalone
}

build_dir () {
  for file in $1/*.md
  do 
    build $file $2
  done
}
echo "bulding recipes"
build_dir $in_dir_recipes $out_dir_recipes
echo "building tags"
build_dir $in_dir_tags $out_dir_tags

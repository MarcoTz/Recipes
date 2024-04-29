outdir_recipe="./html/recipes"
outdir_tag="./html/tags"
indir="./intermediate"

for file in $indir/*.md
do 
 	echo "building html for $file"
	filename="$(basename -- $file)"

  if [[ $filename == tag_* ]] 
  then
    outdir=$outdir_tag
  else
    outdir=$outdir_recipe
  fi

	newfile=$(echo "$outdir/$filename" | sed 's/md/html/g')
	pandoc -f markdown -t html "$indir/$filename" -o $newfile --css="../main.css" --standalone
done

outdir="./html"
indir="./Recipes"

for file in $indir/*.md
do 
	echo "building html for $file"
	filename="$(basename -- $file)"
	newfile=$(echo "$outdir/$filename" | sed 's/md/html/g')
	pandoc -f markdown -t html "$indir/$filename" -o $newfile --css="main.css" --standalone
done

outdir="./html"
indir="./Recipes"

for file in $indir/*.md
do 
	echo "building html for $file"
	filename="$(basename -- $file)"
	newfile=$(echo "$outdir/$filename" | sed 's/md/html/g')
	pandoc "$indir/$filename" -o $newfile 
done

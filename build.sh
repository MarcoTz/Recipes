outdir="./html"

for file in *.md 
do 
	newfile=$(echo $file | sed 's/.md/.html/g')
	pandoc $file -o "$outdir/$newfile"
done

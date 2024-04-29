INDEXFILE="html/index.html"
HTMLDIR="./html"
RECIPEDIR="./Recipes"

echo "generating index.html"

touch $INDEXFILE
echo "<!doctype html>" > $INDEXFILE
echo "<html>" >> $INDEXFILE

echo "<head>" >> $INDEXFILE
echo "<title>Recipes</title>" >> $INDEXFILE
echo "<link rel=\"stylesheet\" href=\"main.css\"/>" >> $INDEXFILE
echo "</head>" >> $INDEXFILE

echo "<body>" >> $INDEXFILE

echo "<h1>Recipes</h1>" >> $INDEXFILE
echo "<ul>" >> $INDEXFILE

for file in $RECIPEDIR/*.md 
do
	filename="$(basename -- $file)"
	echo "<li>" >> $INDEXFILE
	HTMLNAME=$(echo "$filename" | sed 's/md/html/g')
	echo "<a href=\"$HTMLNAME\">" >> $INDEXFILE
	RECIPENAME=$(cat $file | sed '1!d' | sed 's/% //g' >> $INDEXFILE)
	echo "$RECIPENAME" >> $INDEXFILE
	echo "</a>" >> $INDEXFILE
	echo "</li>" >> $INDEXFILE
done

echo "</ul>">> $INDEXFILE
echo "</body>">> $INDEXFILE
echo "</html>" >> $INDEXFILE

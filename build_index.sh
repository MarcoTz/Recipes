INDEXFILE="./html/index.html"
RECIPEDIR="./Recipes"
TAGDIR="./html/tags"

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
	HTMLNAME=$(echo "recipes/$filename" | sed 's/md/html/g')
	echo "<a href=\"$HTMLNAME\">" >> $INDEXFILE
	RECIPENAME=$(cat $file | sed '1!d' | sed 's/% //g' >> $INDEXFILE)
	echo "$RECIPENAME" >> $INDEXFILE
	echo "</a>" >> $INDEXFILE
	echo "</li>" >> $INDEXFILE
done

echo "</ul>">> $INDEXFILE

echo "<h2>Tags</h2>" >> $INDEXFILE
echo "<ul>"  >> $INDEXFILE

for file in $TAGDIR/*.html
do
  filename="$(basename -- $file)"
  echo "<li>" >> $INDEXFILE
  tagname=$(echo $filename | sed 's/tag_//g')
  echo "<a href=\"tags/$filename\"> $tagname </a>" >> $INDEXFILE
  echo "</li>">> $INDEXFILE
done 

echo "</ul>" >> $INDEXFILE
echo "</body>">> $INDEXFILE
echo "</html>" >> $INDEXFILE

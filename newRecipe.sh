FILENAME="Recipes/$1.md"
echo "# $1" > $FILENAME 
echo "" >> $FILENAME
echo "## Ingredients" >> $FILENAME
echo "" >> $FILENAME
echo "## Steps" >> $FILENAME
echo "" >> $FILENAME
echo "## Notes" >> $FILENAME
echo "" >> $FILENAME
echo "#### Tags" >> $FILENAME
$EDITOR $FILENAME


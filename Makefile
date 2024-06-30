.PHONY: build clean new full

BUILDPY = "build.py"
NEWSH = "./newRecipe.sh"
SCRAPERPY = "./scraper.py"

build: 
	python $(BUILDPY)

# call with make new -name=Name
new: 
	$(NEWSH) $(name)
	
clean: 
	rm -rf intermediate
	rm -rf html/recipes
	rm -rf html/tags
	rm -f html/index.html

full: clean build

scrape:
	venv/bin/python $(SCRAPERPY)

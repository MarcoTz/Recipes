.PHONY: build clean full

BUILDPY = "build.py"
build: 
	python $(BUILDPY)

clean: 
	rm -rf intermediate
	rm -rf html/recipes
	rm -rf html/tags
	rm -f html/index.html

full: clean build

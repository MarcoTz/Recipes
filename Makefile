.PHONY: build clean full

BUILDPY = "build.py"
build: 
	python $(BUILDPY)

clean: 
	rm -rf html/*.html
	rm -rf html/recipes/*.html
	rm -rf html/tags/*.html
	rm -rf intermediate/html/recipes/*.html
	rm -rf intermediate/html/tags/*.html
	rm -rf intermediate/md/recipes/*.md
	rm -rf intermediate/md/tags/*.md

full: clean build

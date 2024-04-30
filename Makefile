.PHONY: build clean full

BUILDPY = "build.py"
build: 
	python $(BUILDPY)

clean: 
	rm -rf html/*.html

full: clean build

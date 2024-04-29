.PHONY: build clean full

BUILDSH = "./build.sh"
INDEXSH = "./build_index.sh"

build: 
	$(BUILDSH)
	$(INDEXSH)

clean: 
	echo "removing html files"
	rm -rf html/*.html

full: clean build

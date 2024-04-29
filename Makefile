.PHONY: build clean full

BUILDSH = "./build.sh"
INDEXSH = "./build_index.sh"

build: 
	$(BUILDSH)
	$(INDEXSH)

clean: 
	echo "removing html files"
	rm -r html/*.html

full: clean build

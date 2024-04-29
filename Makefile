.PHONY: build clean full

TAGSPY  = "./build_tags.py"
BUILDSH = "./build.sh"
INDEXSH = "./build_index.sh"

prepare: 
	echo "making sure all required directories exist"
	mkdir -p html
	mkdir -p html/tags
	mkdir -p html/recipes
	mkdir -p intermediate

build: prepare 
	echo "preparing tags"
	python $(TAGSPY)
	echo "building html"
	$(BUILDSH)
	$(INDEXSH)

clean: 
	echo "removing html files"
	rm -rf html/*.html
	echo "removing intermediate markdown"
	rm -rf intermediate/*

full: clean build

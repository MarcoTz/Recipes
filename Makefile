.PHONY: build clean

BUILDSH = "./build.sh"
build: 
	$(BUILDSH)

clean: 
	rm -r html/*.html

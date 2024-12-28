NEWSH = "./newRecipe.sh"

.PHONY: render
render:
	cargo run -p render_recipes --bin render_recipes

# call with make new -name=Name
new: 
	$(NEWSH) $(name)
	

.PHONY: test 
test:
	cargo test --all --no-fail-fast

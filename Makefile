render:
	cargo run -p render_recipes --bin render_recipes

.PHONY: test 
test:
	cargo test --all --no-fail-fast

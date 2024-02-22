default:
	just --list
all: compile html css

html:
	npx prettier test.html --write
compile:
	cargo test test_blog_build -- --nocapture
css:
	tailwind -i ./input.css -o ./output.css

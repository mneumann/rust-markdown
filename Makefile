test: markdown.rs
	rustc --test -O -o test markdown.rs

runbench: test
	./test --bench

runtest: test
	./test

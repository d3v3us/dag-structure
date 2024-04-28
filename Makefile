run:
	cd dag_cli && cargo run ../dag_cli/database.txt -r
test:
	cargo test

PHONY: run test
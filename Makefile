.phony:

env:
	export $(shell cat .env | grep -v ^#)

run:
	python3 main.py

batch:
	python3 bot/main.py

setup-db:
	diesel setup --database-url=twisort.db

migrate:
	diesel migration run --database-url=twisort.db

migrate-new:
	diesel migration generate $(name) --database-url=twisort.db

migrate-rollback:
	diesel migration redo --database-url=twisort.db


build:
	cargo build

test:
	cargo test

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

ci-test:
  cargo clippy --all-targets --all-features -- -D warnings
.phony:

env:
	export $$(cat .env | grep -v ^#)

run:
	python3 main.py

batch:
	python3 bot/main.py

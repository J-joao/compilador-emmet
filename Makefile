.PHONY: clean

all:
	rustc src/main.rs -o build/main

clean:
	rm build/main
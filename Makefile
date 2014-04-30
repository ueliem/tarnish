test:
	echo "testing..."
build:
	mkdir -p bin
	rustc src/main.rs -o bin/main -L lib/rust-http/build/
clean:
	rm -rf bin/*
make run:
	./bin/main

test:
	echo "testing..."
build:
	rustc src/main.rs -o bin/main
clean:
	rm -rf bin/*
make run:
	./bin/main

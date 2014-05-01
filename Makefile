test:
	echo "testing..."
build:
	mkdir -p bin
	rustc src/lib.rs -L lib/rust-http/build/ --out-dir bin/
clean:
	rm -rf bin/*

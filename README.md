Tarnish
===========
[![Build Status](https://travis-ci.org/ueliem/tarnish.svg?branch=master)](https://travis-ci.org/ueliem/tarnish)

A web framework for Rust, inspired by Expressjs.

The travis ci for this repository will continue to fail to build until the ppa:hansjorg/rust rust-nightly contains the libregex native regular expressions library for Rust.

Building Tarnish
===========
Tarnish is running on the latest version of Rust available, currently 0.11 pre-nightly (f77784b 2014-04-29 19:46:44 -0700). It was tested on Mavericks and Ubuntu 12.04.

Assuming you already have the latest version of Rust installed, the following will build the dependencies:

```
git submodule init
git submodule update
cd lib/rust-http
./configure
make
```

Then in the tarnish folder,
```
make build
```

will build Tarnish itself.

The instructions given here compile rust-http without SSL. To compile with SSL support, there is more information at the [chris-morgan/rust-http](https://github.com/chris-morgan/rust-http) repository.

Contributing
===========
Any contributions are welcome! Please fork this repository and send a pull request if you have any code, or submit an issue if you notice a problem.

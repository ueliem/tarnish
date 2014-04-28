#![feature(phase)]
#[phase(syntax)]
extern crate regex_macros;
extern crate regex;

mod Urlparse {
	pub struct UrlParseResult {
		pub scheme: ~str,
		pub authority: ~str,
		pub path: ~str,
		pub query: ~str,
		pub fragment: ~str
	}
	pub fn urlparse(urlstr: &str) -> UrlParseResult {//http://stackoverflow.com/questions/6168260/how-to-parse-a-url
		//Index 5 can then be matched against the routes to choose what/how to serve.
		let re = regex!(r"^(([^:/?#]+):)?(//([^/?#]*))?([^?#]*)(\?([^#]*))?(#(.*))?");
		let cap = re.captures(urlstr).unwrap();// {
			println!("0: {} 1: {} 2: {} 3: {} 4: {} 5: {} 6: {} 7: {} 8: {} 9: {}", cap.at(0), cap.at(1), cap.at(2),
				cap.at(3), cap.at(4), cap.at(5), cap.at(6), cap.at(7), cap.at(8), cap.at(9));
			return UrlParseResult{scheme: cap.at(2).to_owned(), authority: cap.at(4).to_owned(), path: cap.at(5).to_owned(), query: cap.at(7).to_owned(), fragment: cap.at(9).to_owned()};
	}
}

fn main() {
	let output = ::Urlparse::urlparse("http://burntsushi.net/rustdoc/regex/");
	println!("{}", output.authority);
}

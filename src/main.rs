#![feature(phase)]
#[phase(syntax)]
extern crate regex_macros;
extern crate regex;

extern crate time;
extern crate http;

use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::Writer;

use http::server::{Config, Server, Request, ResponseWriter};
use http::headers::content_type::MediaType;

#[deriving(Clone)]
struct RustServer {
	pub portnum: u16,
}

impl Server for RustServer {
    fn get_config(&self) -> Config {
        Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: self.portnum } }//This line confused me inititally. This is an override.
		//Thus on a call, within the server_forever, this gives the port to start the sever listening on.
    }

    fn handle_request(&self, _r: &Request, w: &mut ResponseWriter) {
        w.headers.date = Some(time::now_utc());
        w.headers.content_length = Some(14);
        w.headers.content_type = Some(MediaType {
            type_: StrBuf::from_str("text"),
            subtype: StrBuf::from_str("plain"),
            parameters: vec!((StrBuf::from_str("charset"), StrBuf::from_str("UTF-8")))
        });
        w.headers.server = Some(StrBuf::from_str("Example"));

        let parseout = ::Urlparse::urlparse(_r.request_uri.to_str());//Split the URL properly, then compare path to routes.
		w.write(parseout.path.to_str().as_bytes()).unwrap();
		println!("{}", _r.request_uri);
    }
}

impl RustServer {
	pub fn start(&self) {
		//Set port/settings, then begin the server.
		//self.portnum = portnum;
		self.serve_forever();
	}
}

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
		let cap = re.captures(urlstr).unwrap();
		return UrlParseResult{scheme: cap.at(2).to_owned(), authority: cap.at(4).to_owned(), path: cap.at(5).to_owned(), query: cap.at(7).to_owned(), fragment: cap.at(9).to_owned()};
	}
}

fn main() {
	let serv = RustServer{portnum: 8001};
	serv.start();
}

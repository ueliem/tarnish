use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::Writer;

use http::server::{Config, Server, Request, ResponseWriter};
use http::headers::content_type::MediaType;

use time;
use Urlparse;

#[deriving(Clone)]
pub struct RustServer {
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

        let parseout = Urlparse::urlparse(_r.request_uri.to_str());//Split the URL properly, then compare path to routes.
        w.headers.content_length = Some(parseout.path.to_str().as_bytes().len());
        w.write(parseout.path.to_str().as_bytes()).unwrap();
        println!("{}", parseout.path.to_str().as_bytes().len());
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

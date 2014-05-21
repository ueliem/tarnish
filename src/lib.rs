#![crate_id = "tarnish"]
#![desc = "Tarnish, a web framework."]
#![crate_type = "dylib"]
#![crate_type = "rlib"]
#![feature(managed_boxes)]

#![feature(phase)]
#[phase(syntax)]
extern crate regex_macros;
extern crate regex;

extern crate time;
extern crate http;

//Making mods public interestingly prevents dead code warnings from the sub-files.
pub mod Urlparse;//Parses URL using native regular expressions library, and outputs a struct with the components.
pub mod rustserver;//Contains the actual base RustServer.
pub mod request;
pub mod response;
pub mod router;

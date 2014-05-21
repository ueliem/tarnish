use http::server::ResponseWriter;
//use http::headers::content_type::MediaType;

pub struct Response<'r> {
    pub response_writer: &'r ResponseWriter<'r>,
}

/*impl<'r> Response<'r> {
    fn send<'r>() {

    }
}
*/

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

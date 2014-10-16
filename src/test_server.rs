extern crate url;

use std::collections::HashMap;
use std::io::net::ip::{IpAddr, Ipv4Addr};
use std::io::MemWriter;

use hyper::header::common::ContentLength;
use hyper::method::{Get, Post};

use hyper::net::{HttpAcceptor, Streaming, HttpStream};

use hyper::server::{Listening, Incoming, Server, Handler, response};
use hyper::server::request::Request;

use hyper::status;

use hyper::uri::AbsolutePath;

use serialize::json::ToJson;

use self::url::form_urlencoded;

pub const ADDR: IpAddr = Ipv4Addr(127, 0, 0, 1);

type Response = response::Response<Streaming>;

#[macro_export]
macro_rules! echo_const( 
    ($port:expr, $msg:expr) => (
        {
            use test_server::start_server;
            start_server($port, |_, res| res.write_str($msg).unwrap())
        }
    )
)

pub fn echo_params(port: u16) -> Listening {
    start_server(port, echo_params_json)    
}

pub fn echo_path(port: u16) -> Listening {
    start_server(port, echo_path_json) 
}

pub fn start_server(port: u16, f: |&mut Request, &mut Writer|: Send) -> Listening {
    let handler: proc(Incoming): Send = proc(mut inc){
        for (mut req, mut res) in inc {
            *res.status_mut() = status::Ok;

            let mut buf = MemWriter::new();
            f(&mut req, &mut buf);
            let content = buf.unwrap();

            res.headers_mut().set(ContentLength(content.len()));
            
            let mut res = res.start().unwrap();
            res.write(content[]).unwrap();
            res.end().unwrap();
        }      
    };

    let handler = ProcHandler { handler: handler };

    Server::http(ADDR.clone(), port)
        .listen(handler).unwrap()
}

struct ProcHandler {
    handler: proc(Incoming): Send,   
}

impl Handler<HttpAcceptor, HttpStream> for ProcHandler {
    fn handle(self, incoming: Incoming) {
        (self.handler)(incoming)    
    }    
}


fn echo_params_json(req: &mut Request, res: &mut Writer) {
    match req.method {
        Get => echo_get(req, res),
        Post => echo_post(req, res),
        _ => unimplemented!(),  
    }    
}

fn echo_get(req: &Request, res: &mut Writer) {
    req_query(req).to_json().to_writer(res).unwrap();
}

fn echo_post(req: &mut Request, res: &mut Writer) {
    let mut params = body_params(req);

    params.extend(req_query(req).into_iter());

    params.to_json().to_writer(res).unwrap();    
}

fn body_params(req: &mut Request) -> HashMap<String, String> {
    let body = req.read_to_string().unwrap();
    form_urlencoded::parse_str(body.as_slice()).into_iter().collect()     
}

fn echo_path_json(req: &mut Request, res: &mut Writer) {
    let path = req_path_vec(req);

    let mut pairs = HashMap::new();
    let mut maybe_key = None;

    for val in path.into_iter() {
        match maybe_key.take() {
            Some(key) => { pairs.insert(key, val); },
            None => maybe_key = Some(val),
        }     
    }        

    pairs.to_json().to_writer(res).unwrap();             
}

fn req_path<'a>(req: &'a Request) -> (Vec<String>, Option<String>, Option<String>) {
    match req.uri {
       AbsolutePath(ref path_str) => url::parse_path(path_str[]).unwrap(),
        _ => fail!("Request was not absolute path!"),   
    }    
}

fn req_path_vec(req: &Request) -> Vec<String> {
        let (path, _, _) = req_path(req);
        path
}

fn req_query(req: &Request) -> HashMap<String, String> {
    let (_, query, _) = req_path(req);
    
    let query = query.unwrap_or_default();
    
    form_urlencoded::parse_str(query[]).into_iter().collect()        
}

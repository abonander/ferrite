#![feature(macro_rules, phase, slicing_syntax)]
#[phase(plugin, link)] extern crate log;
extern crate serialize;
extern crate rest_client;
extern crate hyper;

/// Reexported types
#[allow(unused_imports)]
pub use self::ferrite::{
    JsonError,
    RestClient,
    RestError,
    decode,
    APIError,
    RestErr,
    StatusErr,
    JsonErr
};

mod ferrite {
    pub use serialize::json::DecoderError as JsonError;
    pub use rest_client::{RestClient, RestError};
    pub use serialize::json::decode;
   
    #[deriving(Show)]
    pub enum APIError {
        RestErr(RestError),
        StatusErr(String),
        JsonErr(JsonError), 
    }
}

macro_rules! rest(
    ($method:ident $url:expr: fn $fn_name:ident {$($param:ident: $p_ty: ty),*}($($arg:ident: $a_ty:ty),*) -> $ret:ty) => (
        fn $fn_name($($param: $p_ty),* $($arg: $a_ty),*) -> Result<$ret, APIError> {
            use ferrite::{
                RestClient,
                decode,
                RestErr,
                StatusErr,
                JsonErr
            };

            pair!()

            let url = format!($url, $($param),*);

            let owned: Vec<(&str, String)> = vec![$((stringify!($arg), $arg.to_string())),*];
            let params: Vec<(&str, &str)> = owned.iter().map(pair).collect();
            
            let response = try!(RestClient::$method(url.as_slice(), params.as_slice()).map_err(|e| RestErr(e)));
            
            if response.body.is_empty() {
                Err(StatusErr(response.status.to_string()))    
            } else {
                debug!("response: {}", response.body);
                decode::<$ret>(response.body.as_slice()).map_err(|e| JsonErr(e))    
            }
        }
    );
    ($method:ident $url:expr: fn $fn_name:ident ($($arg:ident: $a_ty:ty),*) -> $ret:ty) => (
        rest!($method $url fn $fn_name{}($($arg: $a_ty),*) -> $ret)
    )
)

macro_rules! pair(
    () => (
        fn pair<'a>(tup: &'a (&'a str, String)) -> (&'a str, &'a str) {
            let (ref a, ref b) = *tup;
            (*a, b.as_slice())
        }    
    )
)

#[macro_export]
macro_rules! get(
    ($url:expr: fn $fn_name:ident {$($param:ident: $p_ty: ty),*}($($arg:ident: $a_ty:ty),*) -> $ret:ty) => (
        rest!(get_with_params $url: 
            fn $fn_name{$($param: $p_ty),*}($($arg: $a_ty),*) -> $ret
        )
    );
    ($url:expr: fn $fn_name:ident ($($arg:ident: $a_ty:ty),*) -> $ret:ty) => (
        rest!(get_with_params $url:
            fn $fn_name{}($($arg: $a_ty),*) -> $ret
        )
    )
)

#[macro_export]
macro_rules! post(
    ($url:expr: fn $fn_name:ident {$($param:ident: $p_ty: ty),*}($($arg:ident: $a_ty:ty),*) -> $ret:ty) => (
        rest!(post_with_params $url: 
            fn $fn_name{$($param: $p_ty),*}($($arg: $a_ty),*) -> $ret
        )
    );
    ($url:expr: fn $fn_name:ident ($($arg:ident: $a_ty:ty),*) -> $ret:ty) => (
        rest!(post_with_params $url:
            fn $fn_name{}($($arg: $a_ty),*) -> $ret
        )
    )
)

#[macro_escape]
#[cfg(test)]
mod test_server;

#[cfg(test)]
mod test{
    use super::APIError;
    use test_server;

    #[deriving(Decodable)]
    struct Test {
        hello: String,    
    }
 
    get!("http://127.0.0.1:15370": fn hello_world() -> Test)

    #[test]
    fn test_hello_world() {
        let mut server = echo_const!(15370,
            r#"{"hello":"world"}"# // Raw string syntax kicks ass 
        );

        let test = hello_world().unwrap();

        assert!(test.hello.as_slice() == "world");
        server.close().unwrap();
    }        
    get!("http://127.0.0.1:15371/hello/{}": fn hello{val: &str}() -> Test)

    #[test]
    fn test_hello() {
        let mut server = test_server::echo_path(15371); 

        let test = hello("world").unwrap();        
        assert!(test.hello.as_slice() == "world");

        server.close().unwrap();  
    }
    
    get!("http://127.0.0.1:15372/": fn hello_vec() -> Vec<Test>)

    #[test]
    fn test_hello_vec() {
        let mut server = echo_const!(15372,
            r#"[
                {"hello":"world"},
                {"hello":"nation"},
                {"hello":"city"},
                {"hello":"person"}
            ]"#
        );

        let test_vec = hello_vec().unwrap();
        let mut test_iter = test_vec.iter();

        assert!(test_iter.next().unwrap().hello.as_slice() == "world");
        assert!(test_iter.next().unwrap().hello.as_slice() == "nation");
        assert!(test_iter.next().unwrap().hello.as_slice() == "city");
        assert!(test_iter.next().unwrap().hello.as_slice() == "person");
        assert!(test_iter.next().is_none());
        
        server.close().unwrap();     
    }

    post!("http://127.0.0.1:15373/": fn post_hello(hello: &str) -> Test)

    #[test]
    fn test_post_please_ignore() {
        let mut server = test_server::echo_params(15373);

        let test = post_hello("world").unwrap();        
        assert!(test.hello.as_slice() == "world");

        server.close().unwrap();
    } 
}

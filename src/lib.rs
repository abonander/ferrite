#![feature(macro_rules)]
extern crate serialize;
extern crate rest_client;

use self::serialize::json::{Json, Decoder, decode};
use self::serialize::Decodable;

pub use self::serialize::json::DecoderError as JsonError;

use self::rest_client::RestError;

#[deriving(Show)]
pub enum APIError {
    RestErr(RestError),
    StatusErr(String),
    JsonErr(JsonError), 
}

#[macro_export]
macro_rules! get(
    ($url:expr : fn $fn_name:ident () -> $ret:ident) => (
        fn $fn_name() -> Result<$ret, APIError> {
            use rest_client::RestClient;
            use serialize::Decodable;
            use serialize::json::decode;
            use super::{RestErr, StatusErr, JsonErr};
            
            const url: &'static str = $url;
            
            let response = try!(RestClient::get(url).map_err(|e| RestErr(e)));
            
            if response.body.is_empty() {
                Err(StatusErr(response.status.to_string()))  
            } else {
                decode::<$ret>(response.body.as_slice()).map_err(|e| JsonErr(e))
            }                
        }
    );
    ($url:expr : fn $fn_name:ident ($($arg:ident: $ty:ty),*) -> $ret:ident) => (
        fn $fn_name($($arg:ident: $ty:ty),+) -> Result<$ret, APIError> {
            use rest_client::RestClient;
            use serialize::Decodable;
            use serialize::json::decode;
            use super::{RestErr, StatusErr, JsonErr}; 

            const url: &'static str = $url;

            let params = [$((stringify!($arg).as_slice(), $arg.to_string().as_slice())),+];

            let response = try!(RestClient::get_with_params(url, params.as_slice()).map_err(|e| RestErr(e)));
            
            if response.body.is_empty() {
                Err(StatusErr(response.status.to_string())),    
            } else {
                decode::<$ret>(response.body.as_slice()).map_err(|e| JsonErr(e))
            }      
        }
    )
)

#[cfg(test)]
mod test{
    use super::APIError;

    #[deriving(Decodable)]
    struct Test {
        hello: String,    
    }
    
    get!("http://echo.jsontest.com/hello/world/": fn hello_world() -> Test)
    
    #[test]
    fn test_hello_world() {
        let test = hello_world().unwrap();

        assert!(test.hello.as_slice() == "world");
    }           
    
}

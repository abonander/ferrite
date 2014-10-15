#![feature(macro_rules)]
extern crate serialize;
extern crate rest_client;

pub use self::serialize::json::DecoderError as JsonError;

use self::rest_client::RestError;

#[deriving(Show)]
pub enum APIError {
    RestErr(RestError),
    StatusErr(String),
    JsonErr(JsonError), 
}

macro_rules! rest(
    ($verb:ident $url:expr: fn $fn_name:ident () -> $ret:ident) => (
        fn $fn_name() -> Result<$ret, APIError> {
            use rest_client::RestClient;
            use serialize::json::decode;
            
            const URL: &'static str = $url;
            
            let response = try!(RestClient::$verb(URL).map_err(|e| RestErr(e)));
            
            if response.body.is_empty() {
                Err(StatusErr(response.status.to_string()))  
            } else {
                decode::<$ret>(response.body.as_slice()).map_err(|e| JsonErr(e))
            }                
        }
    );
    ($verb_ext:ident $url:expr: fn $fn_name:ident ($($arg:ident: $ty:ty),+) -> $ret:ident) => (
        fn $fn_name($($arg: $ty),+) -> Result<$ret, APIError> {
            use rest_client::RestClient;
            use serialize::json::decode;

            pair!()

            const URL: &'static str = $url;

            let owned = [$((stringify!($arg), $arg.to_string())),+];
            let params: Vec<(&str, &str)> =  owned.iter().map(pair).collect();

            let response = try!(RestClient::$verb_ext(URL, params.as_slice()).map_err(|e| RestErr(e)));
            
            if response.body.is_empty() {
                Err(StatusErr(response.status.to_string()))    
            } else {
                decode::<$ret>(response.body.as_slice()).map_err(|e| JsonErr(e))
            }      
        }
    );
    ($verb:ident $url:expr{$($param:tt),+}: fn $fn_name:ident () -> $ret:ident) => (
        fn $fn_name() -> Result<$ret, APIError> {
            use rest_client::RestClient;
            use serialize::json::decode;
            
            let url = format!($url, $($param),+);
            
            let response = try!(RestClient::$verb(url.as_slice()).map_err(|e| RestErr(e)));
            
            if response.body.is_empty() {
                Err(StatusErr(response.status.to_string()))  
            } else {
                decode::<$ret>(response.body.as_slice()).map_err(|e| JsonErr(e))
            }                
        }
    );
    ($verb_ext: ident $url:expr{$($param:tt),+}: fn $fn_name:ident ($($arg:ident: $ty:ty),+) -> $ret:ident) => (
        fn $fn_name($($arg: $ty),+) -> Result<$ret, APIError> {
            use rest_client::RestClient;
            use serialize::json::decode;

            pair!()

            let url = format!($url, $($param),+);

            let owned = [$((stringify!($arg), $arg.to_string())),+];
            let params: Vec<(&str, &str)> = owned.iter().map(pair).collect();
            
            let response = try!(RestClient::$verb_ext(url.as_slice(), params.as_slice()).map_err(|e| RestErr(e)));
            
            if response.body.is_empty() {
                Err(StatusErr(response.status.to_string()))    
            } else {
                decode::<$ret>(response.body.as_slice()).map_err(|e| JsonErr(e))    
            }
        }
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
    ($url:expr: fn $fn_name:ident () -> $ret:ident) => (
        rest!(get $url: fn $fn_name() -> $ret)
    );
    ($url:expr: fn $fn_name:ident ($($arg:ident: $ty:ty),*) -> $ret:ident) => (
        rest!(get_with_params $url: fn $fn_name($($arg: $ty),+) -> $ret)
    );
    ($url:expr{$($param:tt),+}: fn $fn_name:ident () -> $ret:ident) => (
        rest!(get $url{$($param),+}: fn $fn_name() -> $ret)
    );
    ($url:expr{$($param:tt),+}: fn $fn_name:ident ($($arg:ident: $ty:ty),+) -> $ret:ident) => (
        rest!(get_with_params $url{$($param),+}: fn $fn_name($($arg: $ty),+) -> $ret)
    );         
)


#[cfg(test)]
mod test{
    use super::{APIError, RestErr, StatusErr, JsonErr};

    #[deriving(Decodable)]
    struct Test {
        hello: String,    
    }
    
    get!("https://raw.githubusercontent.com/cybergeek94/ferrite/master/json/hello_world.json": fn hello_world() -> Test)
   
    // This probably won't work because the endpoint is down 
    get!("http://echo.jsontest.com/hello/{}"{val}: fn hello(val: &str) -> Test)
    
    #[test]
    fn test_hello_world() {
        let test = hello_world().unwrap();

        assert!(test.hello.as_slice() == "world");
    }        
    
    #[test]
    fn test_hello() {
        let test = hello("world").unwrap();
        
        assert!(test.hello.as_slice() == "world");    
    }
    
}

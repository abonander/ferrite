#![feature(macro_rules)]
#![allow(unused_imports)]
extern crate serialize;
extern crate rest_client;

/// Reexported types
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
    ($verb_ext: ident $url:expr: fn $fn_name:ident ($(?$param:ident: $ty: ty),+ $($arg:ident: $ty:ty),+) -> $ret:ty) => (
        fn $fn_name($($arg: $ty),+) -> Result<$ret, APIError> {
            use ferrite::{
                JsonError,
                RestClient,
                RestError,
                decode,
                APIError,
                RestErr,
                StatusErr,
                JsonErr
            };

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
    );
    ($verb:ident $url:expr: fn $fn_name:ident ($(?$param:ident: $ty:ty),+) -> $ret:ty) => (
        fn $fn_name($($param: $ty),+) -> Result<$ret, APIError> { 
            use ferrite::{
                JsonError,
                RestClient,
                RestError,
                decode,
                APIError,
                RestErr,
                StatusErr,
                JsonErr
            };

            let url = format!($url, $($param),+);
            
            let response = try!(RestClient::$verb(url.as_slice()).map_err(|e| RestErr(e)));
            
            if response.body.is_empty() {
                Err(StatusErr(response.status.to_string()))  
            } else {
                decode::<$ret>(response.body.as_slice()).map_err(|e| JsonErr(e))
            }                
        }
    );
    ($verb:ident $url:expr: fn $fn_name:ident () -> $ret:ty) => (
        fn $fn_name() -> Result<$ret, APIError> {           
            use ferrite::{
                JsonError,
                RestClient,
                RestError,
                decode,
                APIError,
                RestErr,
                StatusErr,
                JsonErr
            };

            const URL: &'static str = $url;
            
            let response = try!(RestClient::$verb(URL).map_err(|e| RestErr(e)));
            
            if response.body.is_empty() {
                Err(StatusErr(response.status.to_string()))  
            } else {
                decode::<$ret>(response.body.as_slice()).map_err(|e| JsonErr(e))
            }                
        }
    );
    ($verb_ext:ident $url:expr: fn $fn_name:ident ($($arg:ident: $ty:ty),+) -> $ret:ty) => (
        fn $fn_name($($arg: $ty),+) -> Result<$ret, APIError> {
            use ferrite::{
                JsonError,
                RestClient,
                RestError,
                decode,
                APIError,
                RestErr,
                StatusErr,
                JsonErr
            };
            
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
    ($url:expr: fn $fn_name:ident () -> $ret:ty) => (
        rest!(get $url: fn $fn_name() -> $ret)
    );
    ($url:expr: fn $fn_name:ident ($(?$param:ident: $ty: ty),+) -> $ret:ty) => (
        rest!(get $url: fn $fn_name($(?$param: $ty),+) -> $ret)
    );
    ($url:expr: fn $fn_name:ident ($(?$param:ident: $ty: ty),+ $($arg:ident: $ty:ty),+) -> $ret:ty) => (
        rest!(get_with_params $url: fn $fn_name($(?$param: $ty),+ $($arg: $ty),+) -> $ret)
    );
    ($url:expr: fn $fn_name:ident ($($arg:ident: $ty:ty),*) -> $ret:ty) => (
        rest!(get_with_params $url: fn $fn_name($($arg: $ty),+) -> $ret)
    );             
)


#[cfg(test)]
mod test{
    use super::APIError; 

    #[deriving(Decodable)]
    struct Test {
        hello: String,    
    }
   
    // TODO: Host these on GitHub-Pages or a local test server 
    get!("https://raw.githubusercontent.com/cybergeek94/ferrite/master/json/hello_world.json": fn hello_world() -> Test)
  
    get!("https://raw.githubusercontent.com/cybergeek94/ferrite/master/json/hello_vec.json": fn hello_vec() -> Vec<Test>)
   
    get!("https://raw.githubusercontent.com/cybergeek94/ferrite/master/json/hello_{}.json": fn hello(?val: &str) -> Test)
    
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

    #[test]
    fn test_hello_vec() {
        let test_vec = hello_vec().unwrap();
        let mut test_iter = test_vec.iter();

        assert!(test_iter.next().unwrap().hello.as_slice() == "world");
        assert!(test_iter.next().unwrap().hello.as_slice() == "nation");
        assert!(test_iter.next().unwrap().hello.as_slice() == "city");
        assert!(test_iter.next().unwrap().hello.as_slice() == "person");
        assert!(test_iter.next().is_none());              
    }
    
}

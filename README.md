ferrite
=======

A typesafe API wrapper for Rust, in the same vein as Retrofit for Java (https://square.github.io/retrofit/).

Compared to Retrofit, ferrite generates the glue-code at compile-time; no runtime reflection necessary. Generated functions return informative errors on failure, and can be called just like any other Rust function.

Currently only a proof-of-concept, ferrite needs quite a bit more work to reach feature-parity with Retrofit,
and even the underlying `rest_client` library that powers it.

* POST Requests
* Miscellaneous REST verbs (PUT, DELETE, HEAD, etc.)
* ~~URL Interpolation (e.g. replacing `{id}` in `/users/{id}/posts/` with a dynamic argument)~~
* Multipart/file/binary uploads (using `Reader`)
* Generation of methods (e.g. functions in an `impl` that take `&self`) that can pass struct fields as parameters
* Support for generics (necessary?)
* Usability/readability tweaks to syntax

Usage
-----

Add to your `Cargo.toml`:
```
[dependencies.ferrite]
git = "https://github.com/cybergeek94/ferrite"


```rust
extern crate ferrite; // Not sure if needs #[phase(plugin)]
// Ferrite's error type needs to be in-scope for the function signatures
use self::ferrite::APIError; 
    
#[deriving(Decodable)]
struct Test {
    hello: String,
}
    
// Function will have the following generated signature:
// fn hello_world() -> Result<Test, APIError> (this is why the error type needs to be in-scope)
get!("https://raw.githubusercontent.com/cybergeek94/ferrite/master/json/hello_world.json":
    fn hello_world() -> Test)
    
get!("https://raw.githubusercontent.com/cybergeek94/ferrite/master/json/hello_world.json": 
    fn hello_world() -> Test)
  
// Vectors are automatically decoded!
get!("https://raw.githubusercontent.com/cybergeek94/ferrite/master/json/hello_vec.json": 
    fn hello_vec() -> Vec<Test>)
   
// Preface a parameter with ? to format it in the URL instead of passing it in the query string
// ?-parameters MUST come before regular ones
get!("https://raw.githubusercontent.com/cybergeek94/ferrite/master/json/hello_{}.json": 
    fn hello(?val: &str) -> Test)

fn main() {
    let test = hello_world().unwrap();
    assert!(test.hello.as_slice() == "world");
    
    let test = hello("world").unwrap();
    assert!(test.hello.as_slice() == "world"); 
    
    let test_vec = hello_vec().unwrap();
    let mut test_iter = test_vec.iter();
    assert!(test_iter.next().unwrap().hello.as_slice() == "world");
    assert!(test_iter.next().unwrap().hello.as_slice() == "nation");
    assert!(test_iter.next().unwrap().hello.as_slice() == "city");
    assert!(test_iter.next().unwrap().hello.as_slice() == "person");
    assert!(test_iter.next().is_none());
}
```

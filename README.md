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



```rust
extern crate ferrite;
use self::ferrite::APIError; // Ferrite's error type needs to be in-scope
    
#[deriving(Decodable)]
struct Test {
    hello: String,
}
    
// Function will have the following generated signature:
// fn hello_world() -> Result<Test, APIError> (this is why the error type needs to be in-scope)
get!("https://raw.githubusercontent.com/cybergeek94/ferrite/master/json/hello_world.json":
    fn hello_world() -> Test)
    
fn main() {
    let test = hello_world().unwrap();
    assert!(test.hello.as_slice() == "world");
}
```

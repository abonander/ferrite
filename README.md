ferrite
=======

A typesafe API wrapper for Rust, in a similar vein as Retrofit (https://square.github.io/retrofit/)

Currently only a proof-of-concept, ferrite needs quite a bit more work to reach feature-parity with Retrofit,
and even the underlying `rest_client` library that powers it.

*[] POST Requests
*[] Miscellaneous REST verbs (PUT, DELETE, HEAD, etc.)
*[] URL Interpolation (e.g. replacing `{id}` in `/users/{id}/posts/` with a dynamic argument)
*[] Multipart/file/binary uploads (using `Reader`)

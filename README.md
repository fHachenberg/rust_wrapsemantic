# rust_wrapsemantic
Implementation of a (SWIG-like) wrap semantic in Rust

Assume that we want to wrap a C++ library. Some of the C++ objects
will be owned (an cleaned up accordingly) by the C++ code but some
object will be owned by our Rust code. SWIG solves this by attaching
an ownership flag to wrapper proxies. This flag is handled using
move semantics.
Here we instead wanna implement this distinction using different types
for "owning wrapper" and "forwarding wrapper".

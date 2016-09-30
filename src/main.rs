extern crate libc;

use libc::{c_void};
use std::ptr;

///Assume that we want to wrap a C++ library. Some of the C++ objects
///will be owned (an cleaned up accordingly) by the C++ code but some
///object will be owned by our Rust code. SWIG solves this by attaching
///an ownership flag to wrapper proxies. This flag is handled using
///move semantics.
///Here we instead wanna implement this distinction using different types
///for "owning wrapper" and "forwarding wrapper".

/// --- general declarations usable for all wrappers follow ---

///wraps a c++ object and calls delete on drop
struct CppBox {
    //c++ object
    cpp_ptr:*mut c_void,
    //c-function which calls delete
    cpp_dtor:fn(*mut c_void)
}

impl Drop for CppBox {
    fn drop(&mut self) {
        self.cpp_dtor(self.cpp_ptr);
    }
}

///represents the methods of the wrapped C++ class
///access to c++ data is done via "data" method,
///which is part of the CppPtrWrapper trait
trait Interface : CppPtrWrapper {
    fn foo(&self) -> i32 {
        return 0;
    }
}

///access c++ void* ptr
trait CppPtrWrapper {
    fn data(&self) -> *mut c_void;
}

/// --- specific declarations usable for one and only one wrapper follow ---

///this struct is named after the wrapped C++ class to
///allow Rust programmers to identify it.
///In the logic of C++ this IS the wrapper class
struct Class<T> {
    ptr:T
}

///attach interface to wrapper type
impl<T> Interface for Class<T> {

}

///constructor for creating Rust-code-owned c++ objects
impl Class<CppBox> {
    fn new() -> Class<CppBox> {
        ///todo: call specific C function wrapping the constructor for wrapped c++ class
        return Class { ptr: CppBox { cpp_ptr:std::ptr::null_mut(), cpp_dtor:std::ptr::null_mut() } };
    }
}

impl CppPtrWrapper for Class<CppBox> {
    fn data(&self) -> *mut c_void {
        return self.ptr.cpp_ptr;
    }
}

impl CppPtrWrapper for Class<*mut c_void> {
    fn data(&self) -> *mut c_void {
        return self.ptr;
    }
}

impl Copy for Class<*mut c_void> {
}

impl Clone for Class<*mut c_void> {
    fn clone(&self) -> Self {
        *self
    }
}

/// --- test code follows ---

fn main() {
    //create owned object
    let obj = Class::new();
    let objref = &obj;
    let val = objref.foo();
    println!("{}", val);
}

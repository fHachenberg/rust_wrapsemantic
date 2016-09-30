extern crate libc;

use libc::{c_void};
use std::ptr;

//calls cpp dtor automatically on drop
struct CppBox {
    cpp_ptr:*mut c_void,
    cpp_dtor:*mut c_void
}

impl Drop for CppBox {
    fn drop(&mut self) {
        //call c++ dtor
    }
}

trait SpecWrapperTrait {
    fn foo(&self) -> i32 {
        return 0;
    }
}

struct SpecWrapper<T> {
    ptr:T
}

impl<T> SpecWrapperTrait for SpecWrapper<T> {
    
}

impl SpecWrapper<CppBox> {
    fn new() -> SpecWrapper<CppBox> {
        return SpecWrapper { ptr: CppBox { cpp_ptr:std::ptr::null_mut(), cpp_dtor:std::ptr::null_mut() } };
    }
}

//Allows to access C++ object
trait CppPtrWrapper {
    fn data(&self) -> *mut c_void;
}

impl CppPtrWrapper for SpecWrapper<CppBox> {
    fn data(&self) -> *mut c_void {
        return self.ptr.cpp_ptr;
    }
}

impl CppPtrWrapper for SpecWrapper<*mut c_void> {
    fn data(&self) -> *mut c_void {
        return self.ptr;
    }
}

impl Copy for SpecWrapper<*mut c_void> {
}

impl Clone for SpecWrapper<*mut c_void> {
    fn clone(&self) -> Self {
        *self
    }
}

fn main() {
    //create owned object
    let obj = SpecWrapper::new();
    let objref = &obj;
    let val = objref.foo();
    println!("{}", val);
}

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLSObject;
);

unsafe impl NSCoding for CLSObject {}

unsafe impl NSObjectProtocol for CLSObject {}

unsafe impl NSSecureCoding for CLSObject {}

extern_methods!(
    unsafe impl CLSObject {
        #[method_id(@__retain_semantics Other dateCreated)]
        pub unsafe fn dateCreated(&self) -> Retained<NSDate>;

        #[method_id(@__retain_semantics Other dateLastModified)]
        pub unsafe fn dateLastModified(&self) -> Retained<NSDate>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

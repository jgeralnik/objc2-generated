//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct EKObject;

    unsafe impl ClassType for EKObject {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for EKObject {}

extern_methods!(
    unsafe impl EKObject {
        #[method(hasChanges)]
        pub unsafe fn hasChanges(&self) -> bool;

        #[method(isNew)]
        pub unsafe fn isNew(&self) -> bool;

        #[method(reset)]
        pub unsafe fn reset(&self);

        #[method(rollback)]
        pub unsafe fn rollback(&self);

        #[method(refresh)]
        pub unsafe fn refresh(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl EKObject {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

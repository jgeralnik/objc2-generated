//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsautoreleasepool?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAutoreleasePool;
);

unsafe impl NSObjectProtocol for NSAutoreleasePool {}

extern_methods!(
    unsafe impl NSAutoreleasePool {
        #[method(addObject:)]
        pub unsafe fn addObject_class(an_object: &AnyObject);

        #[method(addObject:)]
        pub unsafe fn addObject(&self, an_object: &AnyObject);

        #[method(drain)]
        pub unsafe fn drain(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSAutoreleasePool {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

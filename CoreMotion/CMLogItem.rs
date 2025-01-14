//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coremotion/cmlogitem?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CMLogItem;
);

unsafe impl NSCoding for CMLogItem {}

unsafe impl NSCopying for CMLogItem {}

unsafe impl CopyingHelper for CMLogItem {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CMLogItem {}

unsafe impl NSSecureCoding for CMLogItem {}

extern_methods!(
    unsafe impl CMLogItem {
        #[method(timestamp)]
        pub unsafe fn timestamp(&self) -> NSTimeInterval;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CMLogItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

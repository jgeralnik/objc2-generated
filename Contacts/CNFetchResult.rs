//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNFetchResult<ValueType: ?Sized = AnyObject>;
);

unsafe impl<ValueType: ?Sized> NSObjectProtocol for CNFetchResult<ValueType> {}

extern_methods!(
    unsafe impl<ValueType: Message> CNFetchResult<ValueType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Retained<ValueType>;

        #[method_id(@__retain_semantics Other currentHistoryToken)]
        pub unsafe fn currentHistoryToken(&self) -> Retained<NSData>;
    }
);

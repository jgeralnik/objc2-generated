//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZSharedDirectory;
);

unsafe impl NSObjectProtocol for VZSharedDirectory {}

extern_methods!(
    unsafe impl VZSharedDirectory {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithURL:readOnly:)]
        pub unsafe fn initWithURL_readOnly(
            this: Allocated<Self>,
            url: &NSURL,
            read_only: bool,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Retained<NSURL>;

        #[method(isReadOnly)]
        pub unsafe fn isReadOnly(&self) -> bool;
    }
);

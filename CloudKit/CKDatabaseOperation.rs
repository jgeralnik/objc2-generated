//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckdatabaseoperation?language=objc)
    #[unsafe(super(CKOperation, NSOperation, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CKOperation")]
    pub struct CKDatabaseOperation;
);

#[cfg(feature = "CKOperation")]
unsafe impl NSObjectProtocol for CKDatabaseOperation {}

extern_methods!(
    #[cfg(feature = "CKOperation")]
    unsafe impl CKDatabaseOperation {
        #[cfg(feature = "CKDatabase")]
        #[method_id(@__retain_semantics Other database)]
        pub unsafe fn database(&self) -> Option<Retained<CKDatabase>>;

        #[cfg(feature = "CKDatabase")]
        #[method(setDatabase:)]
        pub unsafe fn setDatabase(&self, database: Option<&CKDatabase>);
    }
);

extern_methods!(
    /// Methods declared on superclass `CKOperation`
    #[cfg(feature = "CKOperation")]
    unsafe impl CKDatabaseOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CKOperation")]
    unsafe impl CKDatabaseOperation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

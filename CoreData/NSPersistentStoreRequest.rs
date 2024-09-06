//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPersistentStoreRequestType(pub NSUInteger);
impl NSPersistentStoreRequestType {
    pub const NSFetchRequestType: Self = Self(1);
    pub const NSSaveRequestType: Self = Self(2);
    pub const NSBatchInsertRequestType: Self = Self(5);
    pub const NSBatchUpdateRequestType: Self = Self(6);
    pub const NSBatchDeleteRequestType: Self = Self(7);
}

unsafe impl Encode for NSPersistentStoreRequestType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSPersistentStoreRequestType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPersistentStoreRequest;

    unsafe impl ClassType for NSPersistentStoreRequest {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for NSPersistentStoreRequest {}

unsafe impl CopyingHelper for NSPersistentStoreRequest {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSPersistentStoreRequest {}

extern_methods!(
    unsafe impl NSPersistentStoreRequest {
        #[cfg(feature = "NSPersistentStore")]
        #[method_id(@__retain_semantics Other affectedStores)]
        pub unsafe fn affectedStores(&self) -> Option<Retained<NSArray<NSPersistentStore>>>;

        #[cfg(feature = "NSPersistentStore")]
        #[method(setAffectedStores:)]
        pub unsafe fn setAffectedStores(
            &self,
            affected_stores: Option<&NSArray<NSPersistentStore>>,
        );

        #[method(requestType)]
        pub unsafe fn requestType(&self) -> NSPersistentStoreRequestType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPersistentStoreRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

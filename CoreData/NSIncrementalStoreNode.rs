//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSIncrementalStoreNode;

    unsafe impl ClassType for NSIncrementalStoreNode {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for NSIncrementalStoreNode {}

extern_methods!(
    unsafe impl NSIncrementalStoreNode {
        #[cfg(feature = "NSManagedObjectID")]
        #[method_id(@__retain_semantics Init initWithObjectID:withValues:version:)]
        pub unsafe fn initWithObjectID_withValues_version(
            this: Allocated<Self>,
            object_id: &NSManagedObjectID,
            values: &NSDictionary<NSString, AnyObject>,
            version: u64,
        ) -> Retained<Self>;

        #[method(updateWithValues:version:)]
        pub unsafe fn updateWithValues_version(
            &self,
            values: &NSDictionary<NSString, AnyObject>,
            version: u64,
        );

        #[cfg(feature = "NSManagedObjectID")]
        #[method_id(@__retain_semantics Other objectID)]
        pub unsafe fn objectID(&self) -> Retained<NSManagedObjectID>;

        #[method(version)]
        pub unsafe fn version(&self) -> u64;

        #[cfg(feature = "NSPropertyDescription")]
        #[method_id(@__retain_semantics Other valueForPropertyDescription:)]
        pub unsafe fn valueForPropertyDescription(
            &self,
            prop: &NSPropertyDescription,
        ) -> Option<Retained<AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSIncrementalStoreNode {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

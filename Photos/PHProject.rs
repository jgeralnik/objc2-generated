//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "PHCollection", feature = "PHObject"))]
    pub struct PHProject;

    #[cfg(all(feature = "PHCollection", feature = "PHObject"))]
    unsafe impl ClassType for PHProject {
        #[inherits(PHCollection, PHObject, NSObject)]
        type Super = PHAssetCollection;
    }
);

#[cfg(all(feature = "PHCollection", feature = "PHObject"))]
unsafe impl NSCopying for PHProject {}

#[cfg(all(feature = "PHCollection", feature = "PHObject"))]
unsafe impl CopyingHelper for PHProject {
    type Result = Self;
}

#[cfg(all(feature = "PHCollection", feature = "PHObject"))]
unsafe impl NSObjectProtocol for PHProject {}

extern_methods!(
    #[cfg(all(feature = "PHCollection", feature = "PHObject"))]
    unsafe impl PHProject {
        #[method_id(@__retain_semantics Other projectExtensionData)]
        pub unsafe fn projectExtensionData(&self) -> Retained<NSData>;

        #[method(hasProjectPreview)]
        pub unsafe fn hasProjectPreview(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "PHCollection", feature = "PHObject"))]
    unsafe impl PHProject {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

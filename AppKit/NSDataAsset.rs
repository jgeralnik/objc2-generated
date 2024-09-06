//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

pub type NSDataAssetName = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDataAsset;

    unsafe impl ClassType for NSDataAsset {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for NSDataAsset {}

unsafe impl CopyingHelper for NSDataAsset {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSDataAsset {}

extern_methods!(
    unsafe impl NSDataAsset {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithName:)]
        pub unsafe fn initWithName(
            this: Allocated<Self>,
            name: &NSDataAssetName,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithName:bundle:)]
        pub unsafe fn initWithName_bundle(
            this: Allocated<Self>,
            name: &NSDataAssetName,
            bundle: &NSBundle,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSDataAssetName>;

        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data(&self) -> Retained<NSData>;

        #[method_id(@__retain_semantics Other typeIdentifier)]
        pub unsafe fn typeIdentifier(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSDataAsset {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

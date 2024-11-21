//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHAssetResource;
);

unsafe impl NSObjectProtocol for PHAssetResource {}

extern_methods!(
    unsafe impl PHAssetResource {
        #[cfg(feature = "PhotosTypes")]
        #[method(type)]
        pub unsafe fn r#type(&self) -> PHAssetResourceType;

        #[method_id(@__retain_semantics Other assetLocalIdentifier)]
        pub unsafe fn assetLocalIdentifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other uniformTypeIdentifier)]
        pub unsafe fn uniformTypeIdentifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other originalFilename)]
        pub unsafe fn originalFilename(&self) -> Retained<NSString>;

        #[method(pixelWidth)]
        pub unsafe fn pixelWidth(&self) -> NSInteger;

        #[method(pixelHeight)]
        pub unsafe fn pixelHeight(&self) -> NSInteger;

        #[cfg(all(feature = "PHAsset", feature = "PHObject"))]
        #[method_id(@__retain_semantics Other assetResourcesForAsset:)]
        pub unsafe fn assetResourcesForAsset(asset: &PHAsset)
            -> Retained<NSArray<PHAssetResource>>;

        #[cfg(feature = "PHLivePhoto")]
        #[method_id(@__retain_semantics Other assetResourcesForLivePhoto:)]
        pub unsafe fn assetResourcesForLivePhoto(
            live_photo: &PHLivePhoto,
        ) -> Retained<NSArray<PHAssetResource>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl PHAssetResource {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

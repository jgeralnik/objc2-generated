//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photos/phassetcollectionchangerequest?language=objc)
    #[unsafe(super(PHChangeRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PHChangeRequest")]
    pub struct PHAssetCollectionChangeRequest;
);

#[cfg(feature = "PHChangeRequest")]
unsafe impl NSObjectProtocol for PHAssetCollectionChangeRequest {}

extern_methods!(
    #[cfg(feature = "PHChangeRequest")]
    unsafe impl PHAssetCollectionChangeRequest {
        #[method_id(@__retain_semantics Other creationRequestForAssetCollectionWithTitle:)]
        pub unsafe fn creationRequestForAssetCollectionWithTitle(
            title: &NSString,
        ) -> Retained<Self>;

        #[cfg(feature = "PHObject")]
        #[method_id(@__retain_semantics Other placeholderForCreatedAssetCollection)]
        pub unsafe fn placeholderForCreatedAssetCollection(&self) -> Retained<PHObjectPlaceholder>;

        #[method(deleteAssetCollections:)]
        pub unsafe fn deleteAssetCollections(
            asset_collections: &ProtocolObject<dyn NSFastEnumeration>,
        );

        #[cfg(all(feature = "PHCollection", feature = "PHObject"))]
        #[method_id(@__retain_semantics Other changeRequestForAssetCollection:)]
        pub unsafe fn changeRequestForAssetCollection(
            asset_collection: &PHAssetCollection,
        ) -> Option<Retained<Self>>;

        #[cfg(all(
            feature = "PHAsset",
            feature = "PHCollection",
            feature = "PHFetchResult",
            feature = "PHObject"
        ))]
        #[method_id(@__retain_semantics Other changeRequestForAssetCollection:assets:)]
        pub unsafe fn changeRequestForAssetCollection_assets(
            asset_collection: &PHAssetCollection,
            assets: Option<&PHFetchResult<PHAsset>>,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[method(addAssets:)]
        pub unsafe fn addAssets(&self, assets: &ProtocolObject<dyn NSFastEnumeration>);

        #[method(insertAssets:atIndexes:)]
        pub unsafe fn insertAssets_atIndexes(
            &self,
            assets: &ProtocolObject<dyn NSFastEnumeration>,
            indexes: &NSIndexSet,
        );

        #[method(removeAssets:)]
        pub unsafe fn removeAssets(&self, assets: &ProtocolObject<dyn NSFastEnumeration>);

        #[method(removeAssetsAtIndexes:)]
        pub unsafe fn removeAssetsAtIndexes(&self, indexes: &NSIndexSet);

        #[method(replaceAssetsAtIndexes:withAssets:)]
        pub unsafe fn replaceAssetsAtIndexes_withAssets(
            &self,
            indexes: &NSIndexSet,
            assets: &ProtocolObject<dyn NSFastEnumeration>,
        );

        #[method(moveAssetsAtIndexes:toIndex:)]
        pub unsafe fn moveAssetsAtIndexes_toIndex(
            &self,
            from_indexes: &NSIndexSet,
            to_index: NSUInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "PHChangeRequest")]
    unsafe impl PHAssetCollectionChangeRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

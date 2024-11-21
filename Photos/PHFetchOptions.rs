//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHFetchOptions;
);

unsafe impl NSCopying for PHFetchOptions {}

unsafe impl CopyingHelper for PHFetchOptions {
    type Result = Self;
}

unsafe impl NSObjectProtocol for PHFetchOptions {}

extern_methods!(
    unsafe impl PHFetchOptions {
        #[method_id(@__retain_semantics Other predicate)]
        pub unsafe fn predicate(&self) -> Option<Retained<NSPredicate>>;

        #[method(setPredicate:)]
        pub unsafe fn setPredicate(&self, predicate: Option<&NSPredicate>);

        #[method_id(@__retain_semantics Other sortDescriptors)]
        pub unsafe fn sortDescriptors(&self) -> Option<Retained<NSArray<NSSortDescriptor>>>;

        #[method(setSortDescriptors:)]
        pub unsafe fn setSortDescriptors(
            &self,
            sort_descriptors: Option<&NSArray<NSSortDescriptor>>,
        );

        #[method(includeHiddenAssets)]
        pub unsafe fn includeHiddenAssets(&self) -> bool;

        #[method(setIncludeHiddenAssets:)]
        pub unsafe fn setIncludeHiddenAssets(&self, include_hidden_assets: bool);

        #[method(includeAllBurstAssets)]
        pub unsafe fn includeAllBurstAssets(&self) -> bool;

        #[method(setIncludeAllBurstAssets:)]
        pub unsafe fn setIncludeAllBurstAssets(&self, include_all_burst_assets: bool);

        #[cfg(feature = "PhotosTypes")]
        #[method(includeAssetSourceTypes)]
        pub unsafe fn includeAssetSourceTypes(&self) -> PHAssetSourceType;

        #[cfg(feature = "PhotosTypes")]
        #[method(setIncludeAssetSourceTypes:)]
        pub unsafe fn setIncludeAssetSourceTypes(
            &self,
            include_asset_source_types: PHAssetSourceType,
        );

        #[method(fetchLimit)]
        pub unsafe fn fetchLimit(&self) -> NSUInteger;

        #[method(setFetchLimit:)]
        pub unsafe fn setFetchLimit(&self, fetch_limit: NSUInteger);

        #[method(wantsIncrementalChangeDetails)]
        pub unsafe fn wantsIncrementalChangeDetails(&self) -> bool;

        #[method(setWantsIncrementalChangeDetails:)]
        pub unsafe fn setWantsIncrementalChangeDetails(
            &self,
            wants_incremental_change_details: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl PHFetchOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

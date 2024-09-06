//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPMusicPlayerQueueDescriptor;

    unsafe impl ClassType for MPMusicPlayerQueueDescriptor {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for MPMusicPlayerQueueDescriptor {}

extern_methods!(
    unsafe impl MPMusicPlayerQueueDescriptor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPMusicPlayerMediaItemQueueDescriptor;

    unsafe impl ClassType for MPMusicPlayerMediaItemQueueDescriptor {
        #[inherits(NSObject)]
        type Super = MPMusicPlayerQueueDescriptor;
    }
);

unsafe impl NSObjectProtocol for MPMusicPlayerMediaItemQueueDescriptor {}

extern_methods!(
    unsafe impl MPMusicPlayerMediaItemQueueDescriptor {
        #[cfg(feature = "MPMediaQuery")]
        #[method_id(@__retain_semantics Init initWithQuery:)]
        pub unsafe fn initWithQuery(this: Allocated<Self>, query: &MPMediaQuery) -> Retained<Self>;

        #[cfg(all(feature = "MPMediaEntity", feature = "MPMediaItemCollection"))]
        #[method_id(@__retain_semantics Init initWithItemCollection:)]
        pub unsafe fn initWithItemCollection(
            this: Allocated<Self>,
            item_collection: &MPMediaItemCollection,
        ) -> Retained<Self>;

        #[cfg(feature = "MPMediaQuery")]
        #[method_id(@__retain_semantics Other query)]
        pub unsafe fn query(&self) -> Retained<MPMediaQuery>;

        #[cfg(all(feature = "MPMediaEntity", feature = "MPMediaItemCollection"))]
        #[method_id(@__retain_semantics Other itemCollection)]
        pub unsafe fn itemCollection(&self) -> Retained<MPMediaItemCollection>;

        #[cfg(all(feature = "MPMediaEntity", feature = "MPMediaItem"))]
        #[method_id(@__retain_semantics Other startItem)]
        pub unsafe fn startItem(&self) -> Option<Retained<MPMediaItem>>;

        #[cfg(all(feature = "MPMediaEntity", feature = "MPMediaItem"))]
        #[method(setStartItem:)]
        pub unsafe fn setStartItem(&self, start_item: Option<&MPMediaItem>);

        #[cfg(all(feature = "MPMediaEntity", feature = "MPMediaItem"))]
        #[method(setStartTime:forItem:)]
        pub unsafe fn setStartTime_forItem(
            &self,
            start_time: NSTimeInterval,
            media_item: &MPMediaItem,
        );

        #[cfg(all(feature = "MPMediaEntity", feature = "MPMediaItem"))]
        #[method(setEndTime:forItem:)]
        pub unsafe fn setEndTime_forItem(&self, end_time: NSTimeInterval, media_item: &MPMediaItem);
    }
);

extern_methods!(
    /// Methods declared on superclass `MPMusicPlayerQueueDescriptor`
    unsafe impl MPMusicPlayerMediaItemQueueDescriptor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPMusicPlayerStoreQueueDescriptor;

    unsafe impl ClassType for MPMusicPlayerStoreQueueDescriptor {
        #[inherits(NSObject)]
        type Super = MPMusicPlayerQueueDescriptor;
    }
);

unsafe impl NSObjectProtocol for MPMusicPlayerStoreQueueDescriptor {}

extern_methods!(
    unsafe impl MPMusicPlayerStoreQueueDescriptor {
        #[method_id(@__retain_semantics Init initWithStoreIDs:)]
        pub unsafe fn initWithStoreIDs(
            this: Allocated<Self>,
            store_i_ds: &NSArray<NSString>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other storeIDs)]
        pub unsafe fn storeIDs(&self) -> Option<Retained<NSArray<NSString>>>;

        #[method(setStoreIDs:)]
        pub unsafe fn setStoreIDs(&self, store_i_ds: Option<&NSArray<NSString>>);

        #[method_id(@__retain_semantics Other startItemID)]
        pub unsafe fn startItemID(&self) -> Option<Retained<NSString>>;

        #[method(setStartItemID:)]
        pub unsafe fn setStartItemID(&self, start_item_id: Option<&NSString>);

        #[method(setStartTime:forItemWithStoreID:)]
        pub unsafe fn setStartTime_forItemWithStoreID(
            &self,
            start_time: NSTimeInterval,
            store_id: &NSString,
        );

        #[method(setEndTime:forItemWithStoreID:)]
        pub unsafe fn setEndTime_forItemWithStoreID(
            &self,
            end_time: NSTimeInterval,
            store_id: &NSString,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `MPMusicPlayerQueueDescriptor`
    unsafe impl MPMusicPlayerStoreQueueDescriptor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPMusicPlayerPlayParameters;

    unsafe impl ClassType for MPMusicPlayerPlayParameters {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for MPMusicPlayerPlayParameters {}

extern_methods!(
    unsafe impl MPMusicPlayerPlayParameters {
        #[method_id(@__retain_semantics Init initWithDictionary:)]
        pub unsafe fn initWithDictionary(
            this: Allocated<Self>,
            dictionary: &NSDictionary<NSString, AnyObject>,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other dictionary)]
        pub unsafe fn dictionary(&self) -> Retained<NSDictionary<NSString, AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPMusicPlayerPlayParameters {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPMusicPlayerPlayParametersQueueDescriptor;

    unsafe impl ClassType for MPMusicPlayerPlayParametersQueueDescriptor {
        #[inherits(NSObject)]
        type Super = MPMusicPlayerQueueDescriptor;
    }
);

unsafe impl NSObjectProtocol for MPMusicPlayerPlayParametersQueueDescriptor {}

extern_methods!(
    unsafe impl MPMusicPlayerPlayParametersQueueDescriptor {
        #[method_id(@__retain_semantics Init initWithPlayParametersQueue:)]
        pub unsafe fn initWithPlayParametersQueue(
            this: Allocated<Self>,
            play_parameters_queue: &NSArray<MPMusicPlayerPlayParameters>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other playParametersQueue)]
        pub unsafe fn playParametersQueue(&self) -> Retained<NSArray<MPMusicPlayerPlayParameters>>;

        #[method(setPlayParametersQueue:)]
        pub unsafe fn setPlayParametersQueue(
            &self,
            play_parameters_queue: &NSArray<MPMusicPlayerPlayParameters>,
        );

        #[method_id(@__retain_semantics Other startItemPlayParameters)]
        pub unsafe fn startItemPlayParameters(
            &self,
        ) -> Option<Retained<MPMusicPlayerPlayParameters>>;

        #[method(setStartItemPlayParameters:)]
        pub unsafe fn setStartItemPlayParameters(
            &self,
            start_item_play_parameters: Option<&MPMusicPlayerPlayParameters>,
        );

        #[method(setStartTime:forItemWithPlayParameters:)]
        pub unsafe fn setStartTime_forItemWithPlayParameters(
            &self,
            start_time: NSTimeInterval,
            play_parameters: &MPMusicPlayerPlayParameters,
        );

        #[method(setEndTime:forItemWithPlayParameters:)]
        pub unsafe fn setEndTime_forItemWithPlayParameters(
            &self,
            end_time: NSTimeInterval,
            play_parameters: &MPMusicPlayerPlayParameters,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `MPMusicPlayerQueueDescriptor`
    unsafe impl MPMusicPlayerPlayParametersQueueDescriptor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

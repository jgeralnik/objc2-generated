//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHChange;
);

unsafe impl Send for PHChange {}

unsafe impl Sync for PHChange {}

unsafe impl NSObjectProtocol for PHChange {}

extern_methods!(
    unsafe impl PHChange {
        #[cfg(feature = "PHObject")]
        #[method_id(@__retain_semantics Other changeDetailsForObject:)]
        pub unsafe fn changeDetailsForObject(
            &self,
            object: &PHObject,
        ) -> Option<Retained<PHObjectChangeDetails>>;

        #[cfg(feature = "PHFetchResult")]
        #[method_id(@__retain_semantics Other changeDetailsForFetchResult:)]
        pub unsafe fn changeDetailsForFetchResult(
            &self,
            object: &PHFetchResult,
        ) -> Option<Retained<PHFetchResultChangeDetails>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl PHChange {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHObjectChangeDetails<ObjectType: ?Sized = AnyObject>;
);

unsafe impl<ObjectType: ?Sized> NSObjectProtocol for PHObjectChangeDetails<ObjectType> {}

extern_methods!(
    unsafe impl<ObjectType: Message> PHObjectChangeDetails<ObjectType> {
        #[method_id(@__retain_semantics Other objectBeforeChanges)]
        pub unsafe fn objectBeforeChanges(&self) -> Retained<ObjectType>;

        #[method_id(@__retain_semantics Other objectAfterChanges)]
        pub unsafe fn objectAfterChanges(&self) -> Option<Retained<ObjectType>>;

        #[method(assetContentChanged)]
        pub unsafe fn assetContentChanged(&self) -> bool;

        #[method(objectWasDeleted)]
        pub unsafe fn objectWasDeleted(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<ObjectType: Message> PHObjectChangeDetails<ObjectType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHFetchResultChangeDetails<ObjectType: ?Sized = AnyObject>;
);

unsafe impl<ObjectType: ?Sized> NSObjectProtocol for PHFetchResultChangeDetails<ObjectType> {}

extern_methods!(
    unsafe impl<ObjectType: Message> PHFetchResultChangeDetails<ObjectType> {
        #[cfg(feature = "PHFetchResult")]
        #[method_id(@__retain_semantics Other fetchResultBeforeChanges)]
        pub unsafe fn fetchResultBeforeChanges(&self) -> Retained<PHFetchResult<ObjectType>>;

        #[cfg(feature = "PHFetchResult")]
        #[method_id(@__retain_semantics Other fetchResultAfterChanges)]
        pub unsafe fn fetchResultAfterChanges(&self) -> Retained<PHFetchResult<ObjectType>>;

        #[method(hasIncrementalChanges)]
        pub unsafe fn hasIncrementalChanges(&self) -> bool;

        #[method_id(@__retain_semantics Other removedIndexes)]
        pub unsafe fn removedIndexes(&self) -> Option<Retained<NSIndexSet>>;

        #[method_id(@__retain_semantics Other removedObjects)]
        pub unsafe fn removedObjects(&self) -> Retained<NSArray<ObjectType>>;

        #[method_id(@__retain_semantics Other insertedIndexes)]
        pub unsafe fn insertedIndexes(&self) -> Option<Retained<NSIndexSet>>;

        #[method_id(@__retain_semantics Other insertedObjects)]
        pub unsafe fn insertedObjects(&self) -> Retained<NSArray<ObjectType>>;

        #[method_id(@__retain_semantics Other changedIndexes)]
        pub unsafe fn changedIndexes(&self) -> Option<Retained<NSIndexSet>>;

        #[method_id(@__retain_semantics Other changedObjects)]
        pub unsafe fn changedObjects(&self) -> Retained<NSArray<ObjectType>>;

        #[cfg(feature = "block2")]
        #[method(enumerateMovesWithBlock:)]
        pub unsafe fn enumerateMovesWithBlock(
            &self,
            handler: &block2::Block<dyn Fn(NSUInteger, NSUInteger)>,
        );

        #[method(hasMoves)]
        pub unsafe fn hasMoves(&self) -> bool;

        #[cfg(feature = "PHFetchResult")]
        #[method_id(@__retain_semantics Other changeDetailsFromFetchResult:toFetchResult:changedObjects:)]
        pub unsafe fn changeDetailsFromFetchResult_toFetchResult_changedObjects(
            from_result: &PHFetchResult<ObjectType>,
            to_result: &PHFetchResult<ObjectType>,
            changed_objects: &NSArray<ObjectType>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<ObjectType: Message> PHFetchResultChangeDetails<ObjectType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

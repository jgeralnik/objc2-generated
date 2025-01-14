//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsbatchinsertrequest?language=objc)
    #[unsafe(super(NSPersistentStoreRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSPersistentStoreRequest")]
    pub struct NSBatchInsertRequest;
);

#[cfg(feature = "NSPersistentStoreRequest")]
unsafe impl NSCopying for NSBatchInsertRequest {}

#[cfg(feature = "NSPersistentStoreRequest")]
unsafe impl CopyingHelper for NSBatchInsertRequest {
    type Result = Self;
}

#[cfg(feature = "NSPersistentStoreRequest")]
unsafe impl NSObjectProtocol for NSBatchInsertRequest {}

extern_methods!(
    #[cfg(feature = "NSPersistentStoreRequest")]
    unsafe impl NSBatchInsertRequest {
        #[method_id(@__retain_semantics Other entityName)]
        pub unsafe fn entityName(&self) -> Retained<NSString>;

        #[cfg(feature = "NSEntityDescription")]
        #[method_id(@__retain_semantics Other entity)]
        pub unsafe fn entity(&self) -> Option<Retained<NSEntityDescription>>;

        #[method_id(@__retain_semantics Other objectsToInsert)]
        pub unsafe fn objectsToInsert(
            &self,
        ) -> Option<Retained<NSArray<NSDictionary<NSString, AnyObject>>>>;

        #[method(setObjectsToInsert:)]
        pub unsafe fn setObjectsToInsert(
            &self,
            objects_to_insert: Option<&NSArray<NSDictionary<NSString, AnyObject>>>,
        );

        #[cfg(feature = "block2")]
        #[method(dictionaryHandler)]
        pub unsafe fn dictionaryHandler(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<NSMutableDictionary<NSString, AnyObject>>) -> Bool>;

        #[cfg(feature = "block2")]
        #[method(setDictionaryHandler:)]
        pub unsafe fn setDictionaryHandler(
            &self,
            dictionary_handler: Option<
                &block2::Block<dyn Fn(NonNull<NSMutableDictionary<NSString, AnyObject>>) -> Bool>,
            >,
        );

        #[cfg(all(feature = "NSManagedObject", feature = "block2"))]
        #[method(managedObjectHandler)]
        pub unsafe fn managedObjectHandler(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<NSManagedObject>) -> Bool>;

        #[cfg(all(feature = "NSManagedObject", feature = "block2"))]
        #[method(setManagedObjectHandler:)]
        pub unsafe fn setManagedObjectHandler(
            &self,
            managed_object_handler: Option<
                &block2::Block<dyn Fn(NonNull<NSManagedObject>) -> Bool>,
            >,
        );

        #[cfg(feature = "NSPersistentStoreResult")]
        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSBatchInsertRequestResultType;

        #[cfg(feature = "NSPersistentStoreResult")]
        #[method(setResultType:)]
        pub unsafe fn setResultType(&self, result_type: NSBatchInsertRequestResultType);

        #[method_id(@__retain_semantics Other batchInsertRequestWithEntityName:objects:)]
        pub unsafe fn batchInsertRequestWithEntityName_objects(
            entity_name: &NSString,
            dictionaries: &NSArray<NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other batchInsertRequestWithEntityName:dictionaryHandler:)]
        pub unsafe fn batchInsertRequestWithEntityName_dictionaryHandler(
            entity_name: &NSString,
            handler: &block2::Block<
                dyn Fn(NonNull<NSMutableDictionary<NSString, AnyObject>>) -> Bool,
            >,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSManagedObject", feature = "block2"))]
        #[method_id(@__retain_semantics Other batchInsertRequestWithEntityName:managedObjectHandler:)]
        pub unsafe fn batchInsertRequestWithEntityName_managedObjectHandler(
            entity_name: &NSString,
            handler: &block2::Block<dyn Fn(NonNull<NSManagedObject>) -> Bool>,
        ) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithEntityName:objects:)]
        pub unsafe fn initWithEntityName_objects(
            this: Allocated<Self>,
            entity_name: &NSString,
            dictionaries: &NSArray<NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSEntityDescription")]
        #[method_id(@__retain_semantics Init initWithEntity:objects:)]
        pub unsafe fn initWithEntity_objects(
            this: Allocated<Self>,
            entity: &NSEntityDescription,
            dictionaries: &NSArray<NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSEntityDescription", feature = "block2"))]
        #[method_id(@__retain_semantics Init initWithEntity:dictionaryHandler:)]
        pub unsafe fn initWithEntity_dictionaryHandler(
            this: Allocated<Self>,
            entity: &NSEntityDescription,
            handler: &block2::Block<
                dyn Fn(NonNull<NSMutableDictionary<NSString, AnyObject>>) -> Bool,
            >,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "NSEntityDescription",
            feature = "NSManagedObject",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Init initWithEntity:managedObjectHandler:)]
        pub unsafe fn initWithEntity_managedObjectHandler(
            this: Allocated<Self>,
            entity: &NSEntityDescription,
            handler: &block2::Block<dyn Fn(NonNull<NSManagedObject>) -> Bool>,
        ) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithEntityName:dictionaryHandler:)]
        pub unsafe fn initWithEntityName_dictionaryHandler(
            this: Allocated<Self>,
            entity_name: &NSString,
            handler: &block2::Block<
                dyn Fn(NonNull<NSMutableDictionary<NSString, AnyObject>>) -> Bool,
            >,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSManagedObject", feature = "block2"))]
        #[method_id(@__retain_semantics Init initWithEntityName:managedObjectHandler:)]
        pub unsafe fn initWithEntityName_managedObjectHandler(
            this: Allocated<Self>,
            entity_name: &NSString,
            handler: &block2::Block<dyn Fn(NonNull<NSManagedObject>) -> Bool>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSPersistentStoreRequest")]
    unsafe impl NSBatchInsertRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

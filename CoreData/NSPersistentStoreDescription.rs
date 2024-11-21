//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPersistentStoreDescription;
);

unsafe impl NSCopying for NSPersistentStoreDescription {}

unsafe impl CopyingHelper for NSPersistentStoreDescription {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSPersistentStoreDescription {}

extern_methods!(
    unsafe impl NSPersistentStoreDescription {
        #[method_id(@__retain_semantics Other persistentStoreDescriptionWithURL:)]
        pub unsafe fn persistentStoreDescriptionWithURL(url: &NSURL) -> Retained<Self>;

        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Retained<NSString>;

        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: &NSString);

        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration(&self) -> Option<Retained<NSString>>;

        #[method(setConfiguration:)]
        pub unsafe fn setConfiguration(&self, configuration: Option<&NSString>);

        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Retained<NSURL>>;

        #[method(setURL:)]
        pub unsafe fn setURL(&self, url: Option<&NSURL>);

        #[method_id(@__retain_semantics Other options)]
        pub unsafe fn options(&self) -> Retained<NSDictionary<NSString, NSObject>>;

        #[method(setOption:forKey:)]
        pub unsafe fn setOption_forKey(&self, option: Option<&NSObject>, key: &NSString);

        #[method(isReadOnly)]
        pub unsafe fn isReadOnly(&self) -> bool;

        #[method(setReadOnly:)]
        pub unsafe fn setReadOnly(&self, read_only: bool);

        #[method(timeout)]
        pub unsafe fn timeout(&self) -> NSTimeInterval;

        #[method(setTimeout:)]
        pub unsafe fn setTimeout(&self, timeout: NSTimeInterval);

        #[method_id(@__retain_semantics Other sqlitePragmas)]
        pub unsafe fn sqlitePragmas(&self) -> Retained<NSDictionary<NSString, NSObject>>;

        #[method(setValue:forPragmaNamed:)]
        pub unsafe fn setValue_forPragmaNamed(&self, value: Option<&NSObject>, name: &NSString);

        #[method(shouldAddStoreAsynchronously)]
        pub unsafe fn shouldAddStoreAsynchronously(&self) -> bool;

        #[method(setShouldAddStoreAsynchronously:)]
        pub unsafe fn setShouldAddStoreAsynchronously(&self, should_add_store_asynchronously: bool);

        #[method(shouldMigrateStoreAutomatically)]
        pub unsafe fn shouldMigrateStoreAutomatically(&self) -> bool;

        #[method(setShouldMigrateStoreAutomatically:)]
        pub unsafe fn setShouldMigrateStoreAutomatically(
            &self,
            should_migrate_store_automatically: bool,
        );

        #[method(shouldInferMappingModelAutomatically)]
        pub unsafe fn shouldInferMappingModelAutomatically(&self) -> bool;

        #[method(setShouldInferMappingModelAutomatically:)]
        pub unsafe fn setShouldInferMappingModelAutomatically(
            &self,
            should_infer_mapping_model_automatically: bool,
        );

        #[method_id(@__retain_semantics Init initWithURL:)]
        pub unsafe fn initWithURL(this: Allocated<Self>, url: &NSURL) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPersistentStoreDescription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSPersistentCloudKitContainerAdditions
    unsafe impl NSPersistentStoreDescription {
        #[cfg(feature = "NSPersistentCloudKitContainerOptions")]
        #[method_id(@__retain_semantics Other cloudKitContainerOptions)]
        pub unsafe fn cloudKitContainerOptions(
            &self,
        ) -> Option<Retained<NSPersistentCloudKitContainerOptions>>;

        #[cfg(feature = "NSPersistentCloudKitContainerOptions")]
        #[method(setCloudKitContainerOptions:)]
        pub unsafe fn setCloudKitContainerOptions(
            &self,
            cloud_kit_container_options: Option<&NSPersistentCloudKitContainerOptions>,
        );
    }
);

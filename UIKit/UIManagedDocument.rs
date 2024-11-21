//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-data")]
use objc2_core_data::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(UIDocument, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIDocument")]
    pub struct UIManagedDocument;
);

#[cfg(feature = "UIDocument")]
unsafe impl NSFilePresenter for UIManagedDocument {}

#[cfg(feature = "UIDocument")]
unsafe impl NSObjectProtocol for UIManagedDocument {}

#[cfg(feature = "UIDocument")]
unsafe impl NSProgressReporting for UIManagedDocument {}

extern_methods!(
    #[cfg(feature = "UIDocument")]
    unsafe impl UIManagedDocument {
        #[method_id(@__retain_semantics Other persistentStoreName)]
        pub unsafe fn persistentStoreName(mtm: MainThreadMarker) -> Retained<NSString>;

        #[cfg(feature = "objc2-core-data")]
        #[method_id(@__retain_semantics Other managedObjectContext)]
        pub unsafe fn managedObjectContext(&self) -> Retained<NSManagedObjectContext>;

        #[cfg(feature = "objc2-core-data")]
        #[method_id(@__retain_semantics Other managedObjectModel)]
        pub unsafe fn managedObjectModel(&self) -> Retained<NSManagedObjectModel>;

        #[method_id(@__retain_semantics Other persistentStoreOptions)]
        pub unsafe fn persistentStoreOptions(&self) -> Option<Retained<NSDictionary>>;

        #[method(setPersistentStoreOptions:)]
        pub unsafe fn setPersistentStoreOptions(
            &self,
            persistent_store_options: Option<&NSDictionary>,
        );

        #[method_id(@__retain_semantics Other modelConfiguration)]
        pub unsafe fn modelConfiguration(&self) -> Option<Retained<NSString>>;

        #[method(setModelConfiguration:)]
        pub unsafe fn setModelConfiguration(&self, model_configuration: Option<&NSString>);

        #[method(configurePersistentStoreCoordinatorForURL:ofType:modelConfiguration:storeOptions:error:_)]
        pub unsafe fn configurePersistentStoreCoordinatorForURL_ofType_modelConfiguration_storeOptions_error(
            &self,
            store_url: &NSURL,
            file_type: &NSString,
            configuration: Option<&NSString>,
            store_options: Option<&NSDictionary>,
        ) -> Result<(), Retained<NSError>>;

        #[method_id(@__retain_semantics Other persistentStoreTypeForFileType:)]
        pub unsafe fn persistentStoreTypeForFileType(
            &self,
            file_type: &NSString,
        ) -> Retained<NSString>;

        #[method(readAdditionalContentFromURL:error:_)]
        pub unsafe fn readAdditionalContentFromURL_error(
            &self,
            absolute_url: &NSURL,
        ) -> Result<(), Retained<NSError>>;

        #[method_id(@__retain_semantics Other additionalContentForURL:error:_)]
        pub unsafe fn additionalContentForURL_error(
            &self,
            absolute_url: &NSURL,
        ) -> Result<Retained<AnyObject>, Retained<NSError>>;

        #[method(writeAdditionalContent:toURL:originalContentsURL:error:_)]
        pub unsafe fn writeAdditionalContent_toURL_originalContentsURL_error(
            &self,
            content: &AnyObject,
            absolute_url: &NSURL,
            absolute_original_contents_url: Option<&NSURL>,
        ) -> Result<(), Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIDocument`
    #[cfg(feature = "UIDocument")]
    unsafe impl UIManagedDocument {
        #[method_id(@__retain_semantics Init initWithFileURL:)]
        pub unsafe fn initWithFileURL(this: Allocated<Self>, url: &NSURL) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIDocument")]
    unsafe impl UIManagedDocument {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

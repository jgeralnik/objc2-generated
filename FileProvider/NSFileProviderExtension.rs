//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFileProviderExtension;

    unsafe impl ClassType for NSFileProviderExtension {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for NSFileProviderExtension {}

extern_methods!(
    unsafe impl NSFileProviderExtension {
        #[cfg(feature = "NSFileProviderItem")]
        #[method_id(@__retain_semantics Other itemForIdentifier:error:_)]
        pub unsafe fn itemForIdentifier_error(
            &self,
            identifier: &NSFileProviderItemIdentifier,
        ) -> Result<Retained<NSFileProviderItem>, Retained<NSError>>;

        #[cfg(feature = "NSFileProviderItem")]
        #[method_id(@__retain_semantics Other URLForItemWithPersistentIdentifier:)]
        pub unsafe fn URLForItemWithPersistentIdentifier(
            &self,
            identifier: &NSFileProviderItemIdentifier,
        ) -> Option<Retained<NSURL>>;

        #[cfg(feature = "NSFileProviderItem")]
        #[method_id(@__retain_semantics Other persistentIdentifierForItemAtURL:)]
        pub unsafe fn persistentIdentifierForItemAtURL(
            &self,
            url: &NSURL,
        ) -> Option<Retained<NSFileProviderItemIdentifier>>;

        #[cfg(feature = "block2")]
        #[method(providePlaceholderAtURL:completionHandler:)]
        pub unsafe fn providePlaceholderAtURL_completionHandler(
            &self,
            url: &NSURL,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(startProvidingItemAtURL:completionHandler:)]
        pub unsafe fn startProvidingItemAtURL_completionHandler(
            &self,
            url: &NSURL,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[method(stopProvidingItemAtURL:)]
        pub unsafe fn stopProvidingItemAtURL(&self, url: &NSURL);

        #[method(itemChangedAtURL:)]
        pub unsafe fn itemChangedAtURL(&self, url: &NSURL);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSFileProviderExtension {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// Deprecated
    unsafe impl NSFileProviderExtension {
        #[deprecated = "Use the corresponding method on NSFileProviderManager instead"]
        #[method(writePlaceholderAtURL:withMetadata:error:_)]
        pub unsafe fn writePlaceholderAtURL_withMetadata_error(
            placeholder_url: &NSURL,
            metadata: &NSDictionary<NSURLResourceKey, AnyObject>,
        ) -> Result<(), Retained<NSError>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other placeholderURLForURL:)]
        pub unsafe fn placeholderURLForURL(url: &NSURL) -> Retained<NSURL>;

        #[deprecated]
        #[method_id(@__retain_semantics Other providerIdentifier)]
        pub unsafe fn providerIdentifier(&self) -> Retained<NSString>;

        #[deprecated]
        #[method_id(@__retain_semantics Other documentStorageURL)]
        pub unsafe fn documentStorageURL(&self) -> Retained<NSURL>;
    }
);

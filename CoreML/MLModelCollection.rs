//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use Background Assets or NSURLSession instead."]
    pub struct MLModelCollection;

    unsafe impl ClassType for MLModelCollection {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MLModelCollection {}

extern_methods!(
    unsafe impl MLModelCollection {
        #[deprecated = "Use Background Assets or NSURLSession instead."]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;

        #[cfg(feature = "MLModelCollectionEntry")]
        #[deprecated = "Use Background Assets or NSURLSession instead."]
        #[method_id(@__retain_semantics Other entries)]
        pub unsafe fn entries(&self) -> Id<NSDictionary<NSString, MLModelCollectionEntry>>;

        #[deprecated = "Use Background Assets or NSURLSession instead."]
        #[method_id(@__retain_semantics Other deploymentID)]
        pub unsafe fn deploymentID(&self) -> Id<NSString>;

        #[cfg(feature = "block2")]
        #[deprecated = "Use Background Assets or NSURLSession instead."]
        #[method_id(@__retain_semantics Other beginAccessingModelCollectionWithIdentifier:completionHandler:)]
        pub unsafe fn beginAccessingModelCollectionWithIdentifier_completionHandler(
            identifier: &NSString,
            completion_handler: &block2::Block<dyn Fn(*mut MLModelCollection, *mut NSError)>,
        ) -> Id<NSProgress>;

        #[cfg(feature = "block2")]
        #[deprecated = "Use Background Assets or NSURLSession instead."]
        #[method(endAccessingModelCollectionWithIdentifier:completionHandler:)]
        pub unsafe fn endAccessingModelCollectionWithIdentifier_completionHandler(
            identifier: &NSString,
            completion_handler: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[deprecated = "Use Background Assets or NSURLSession instead."]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[deprecated = "Use Background Assets or NSURLSession instead."]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern "C" {
    pub static MLModelCollectionDidChangeNotification: &'static NSNotificationName;
}
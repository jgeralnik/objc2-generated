//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NLGazetteer;

    unsafe impl ClassType for NLGazetteer {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for NLGazetteer {}

extern_methods!(
    unsafe impl NLGazetteer {
        #[method_id(@__retain_semantics Other gazetteerWithContentsOfURL:error:_)]
        pub unsafe fn gazetteerWithContentsOfURL_error(
            url: &NSURL,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method_id(@__retain_semantics Init initWithContentsOfURL:error:_)]
        pub unsafe fn initWithContentsOfURL_error(
            this: Allocated<Self>,
            url: &NSURL,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method_id(@__retain_semantics Init initWithData:error:_)]
        pub unsafe fn initWithData_error(
            this: Allocated<Self>,
            data: &NSData,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "NLLanguage")]
        #[method_id(@__retain_semantics Init initWithDictionary:language:error:_)]
        pub unsafe fn initWithDictionary_language_error(
            this: Allocated<Self>,
            dictionary: &NSDictionary<NSString, NSArray<NSString>>,
            language: Option<&NLLanguage>,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method_id(@__retain_semantics Other labelForString:)]
        pub unsafe fn labelForString(&self, string: &NSString) -> Option<Retained<NSString>>;

        #[cfg(feature = "NLLanguage")]
        #[method_id(@__retain_semantics Other language)]
        pub unsafe fn language(&self) -> Option<Retained<NLLanguage>>;

        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data(&self) -> Retained<NSData>;

        #[cfg(feature = "NLLanguage")]
        #[method(writeGazetteerForDictionary:language:toURL:error:_)]
        pub unsafe fn writeGazetteerForDictionary_language_toURL_error(
            dictionary: &NSDictionary<NSString, NSArray<NSString>>,
            language: Option<&NLLanguage>,
            url: &NSURL,
        ) -> Result<(), Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NLGazetteer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

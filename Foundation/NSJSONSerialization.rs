//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSJSONReadingOptions {
        NSJSONReadingMutableContainers = 1 << 0,
        NSJSONReadingMutableLeaves = 1 << 1,
        NSJSONReadingFragmentsAllowed = 1 << 2,
        NSJSONReadingJSON5Allowed = 1 << 3,
        NSJSONReadingTopLevelDictionaryAssumed = 1 << 4,
        NSJSONReadingAllowFragments = NSJSONReadingFragmentsAllowed,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSJSONWritingOptions {
        NSJSONWritingPrettyPrinted = 1 << 0,
        NSJSONWritingSortedKeys = 1 << 1,
        NSJSONWritingFragmentsAllowed = 1 << 2,
        NSJSONWritingWithoutEscapingSlashes = 1 << 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSJSONSerialization;

    unsafe impl ClassType for NSJSONSerialization {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSJSONSerialization")]
    unsafe impl NSJSONSerialization {
        #[method(isValidJSONObject:)]
        pub unsafe fn isValidJSONObject(obj: &Object) -> bool;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other dataWithJSONObject:options:error:_)]
        pub unsafe fn dataWithJSONObject_options_error(
            obj: &Object,
            opt: NSJSONWritingOptions,
        ) -> Result<Id<NSData, Shared>, Id<NSError, Shared>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other JSONObjectWithData:options:error:_)]
        pub unsafe fn JSONObjectWithData_options_error(
            data: &NSData,
            opt: NSJSONReadingOptions,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other JSONObjectWithStream:options:error:_)]
        pub unsafe fn JSONObjectWithStream_options_error(
            stream: &NSInputStream,
            opt: NSJSONReadingOptions,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;
    }
);

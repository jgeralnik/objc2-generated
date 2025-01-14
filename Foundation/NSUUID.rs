//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsuuid?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(PartialEq, Eq, Hash)]
    pub struct NSUUID;
);

unsafe impl Send for NSUUID {}

unsafe impl Sync for NSUUID {}

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSUUID {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSUUID {}

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSUUID {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSUUID {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSUUID {}

extern_methods!(
    unsafe impl NSUUID {
        #[method_id(@__retain_semantics Other UUID)]
        pub fn UUID() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Init initWithUUIDString:)]
        pub fn initWithUUIDString(
            this: Allocated<Self>,
            string: &NSString,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSObjCRuntime")]
        #[method(compare:)]
        pub unsafe fn compare(&self, other_uuid: &NSUUID) -> NSComparisonResult;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other UUIDString)]
        pub fn UUIDString(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSUUID {
        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Retained<Self>;
    }
);

impl DefaultRetained for NSUUID {
    #[inline]
    fn default_retained() -> Retained<Self> {
        Self::new()
    }
}

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
    pub struct CXStartCallAction;

    #[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
    unsafe impl ClassType for CXStartCallAction {
        #[inherits(CXAction, NSObject)]
        type Super = CXCallAction;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
unsafe impl NSCoding for CXStartCallAction {}

#[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
unsafe impl NSCopying for CXStartCallAction {}

#[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
unsafe impl CopyingHelper for CXStartCallAction {
    type Result = Self;
}

#[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
unsafe impl NSObjectProtocol for CXStartCallAction {}

#[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
unsafe impl NSSecureCoding for CXStartCallAction {}

extern_methods!(
    #[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
    unsafe impl CXStartCallAction {
        #[cfg(feature = "CXHandle")]
        #[method_id(@__retain_semantics Init initWithCallUUID:handle:)]
        pub unsafe fn initWithCallUUID_handle(
            this: Allocated<Self>,
            call_uuid: &NSUUID,
            handle: &CXHandle,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithCallUUID:)]
        pub unsafe fn initWithCallUUID(this: Allocated<Self>, call_uuid: &NSUUID)
            -> Retained<Self>;

        #[cfg(feature = "CXHandle")]
        #[method_id(@__retain_semantics Other handle)]
        pub unsafe fn handle(&self) -> Retained<CXHandle>;

        #[cfg(feature = "CXHandle")]
        #[method(setHandle:)]
        pub unsafe fn setHandle(&self, handle: &CXHandle);

        #[method_id(@__retain_semantics Other contactIdentifier)]
        pub unsafe fn contactIdentifier(&self) -> Option<Retained<NSString>>;

        #[method(setContactIdentifier:)]
        pub unsafe fn setContactIdentifier(&self, contact_identifier: Option<&NSString>);

        #[method(isVideo)]
        pub unsafe fn isVideo(&self) -> bool;

        #[method(setVideo:)]
        pub unsafe fn setVideo(&self, video: bool);

        #[method(fulfillWithDateStarted:)]
        pub unsafe fn fulfillWithDateStarted(&self, date_started: &NSDate);
    }
);

extern_methods!(
    /// Methods declared on superclass `CXCallAction`
    #[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
    unsafe impl CXStartCallAction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
    unsafe impl CXStartCallAction {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

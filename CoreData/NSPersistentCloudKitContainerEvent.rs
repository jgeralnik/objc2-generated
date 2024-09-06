//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPersistentCloudKitContainerEventType(pub NSInteger);
impl NSPersistentCloudKitContainerEventType {
    #[doc(alias = "NSPersistentCloudKitContainerEventTypeSetup")]
    pub const Setup: Self = Self(0);
    #[doc(alias = "NSPersistentCloudKitContainerEventTypeImport")]
    pub const Import: Self = Self(1);
    #[doc(alias = "NSPersistentCloudKitContainerEventTypeExport")]
    pub const Export: Self = Self(2);
}

unsafe impl Encode for NSPersistentCloudKitContainerEventType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSPersistentCloudKitContainerEventType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    pub static NSPersistentCloudKitContainerEventChangedNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSPersistentCloudKitContainerEventUserInfoKey: &'static NSString;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPersistentCloudKitContainerEvent;

    unsafe impl ClassType for NSPersistentCloudKitContainerEvent {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for NSPersistentCloudKitContainerEvent {}

unsafe impl CopyingHelper for NSPersistentCloudKitContainerEvent {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSPersistentCloudKitContainerEvent {}

extern_methods!(
    unsafe impl NSPersistentCloudKitContainerEvent {
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSUUID>;

        #[method_id(@__retain_semantics Other storeIdentifier)]
        pub unsafe fn storeIdentifier(&self) -> Retained<NSString>;

        #[method(type)]
        pub unsafe fn r#type(&self) -> NSPersistentCloudKitContainerEventType;

        #[method_id(@__retain_semantics Other startDate)]
        pub unsafe fn startDate(&self) -> Retained<NSDate>;

        #[method_id(@__retain_semantics Other endDate)]
        pub unsafe fn endDate(&self) -> Option<Retained<NSDate>>;

        #[method(succeeded)]
        pub unsafe fn succeeded(&self) -> bool;

        #[method_id(@__retain_semantics Other error)]
        pub unsafe fn error(&self) -> Option<Retained<NSError>>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

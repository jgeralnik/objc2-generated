//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static UNNotificationDefaultActionIdentifier: &'static NSString;
}

extern "C" {
    pub static UNNotificationDismissActionIdentifier: &'static NSString;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNNotificationResponse;

    unsafe impl ClassType for UNNotificationResponse {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for UNNotificationResponse {}

unsafe impl NSCopying for UNNotificationResponse {}

unsafe impl CopyingHelper for UNNotificationResponse {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UNNotificationResponse {}

unsafe impl NSSecureCoding for UNNotificationResponse {}

extern_methods!(
    unsafe impl UNNotificationResponse {
        #[cfg(feature = "UNNotification")]
        #[method_id(@__retain_semantics Other notification)]
        pub unsafe fn notification(&self) -> Retained<UNNotification>;

        #[method_id(@__retain_semantics Other actionIdentifier)]
        pub unsafe fn actionIdentifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UNNotificationResponse {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNTextInputNotificationResponse;

    unsafe impl ClassType for UNTextInputNotificationResponse {
        #[inherits(NSObject)]
        type Super = UNNotificationResponse;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for UNTextInputNotificationResponse {}

unsafe impl NSCopying for UNTextInputNotificationResponse {}

unsafe impl CopyingHelper for UNTextInputNotificationResponse {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UNTextInputNotificationResponse {}

unsafe impl NSSecureCoding for UNTextInputNotificationResponse {}

extern_methods!(
    unsafe impl UNTextInputNotificationResponse {
        #[method_id(@__retain_semantics Other userText)]
        pub unsafe fn userText(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UNNotificationResponse`
    unsafe impl UNTextInputNotificationResponse {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UNTextInputNotificationResponse {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

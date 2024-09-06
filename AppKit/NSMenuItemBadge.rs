//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSMenuItemBadgeType(pub NSInteger);
impl NSMenuItemBadgeType {
    #[doc(alias = "NSMenuItemBadgeTypeNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "NSMenuItemBadgeTypeUpdates")]
    pub const Updates: Self = Self(1);
    #[doc(alias = "NSMenuItemBadgeTypeNewItems")]
    pub const NewItems: Self = Self(2);
    #[doc(alias = "NSMenuItemBadgeTypeAlerts")]
    pub const Alerts: Self = Self(3);
}

unsafe impl Encode for NSMenuItemBadgeType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSMenuItemBadgeType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMenuItemBadge;

    unsafe impl ClassType for NSMenuItemBadge {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for NSMenuItemBadge {}

unsafe impl CopyingHelper for NSMenuItemBadge {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSMenuItemBadge {}

extern_methods!(
    unsafe impl NSMenuItemBadge {
        #[method_id(@__retain_semantics Other updatesWithCount:)]
        pub unsafe fn updatesWithCount(item_count: NSInteger) -> Retained<Self>;

        #[method_id(@__retain_semantics New newItemsWithCount:)]
        pub unsafe fn newItemsWithCount(item_count: NSInteger) -> Retained<Self>;

        #[method_id(@__retain_semantics Other alertsWithCount:)]
        pub unsafe fn alertsWithCount(item_count: NSInteger) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCount:type:)]
        pub unsafe fn initWithCount_type(
            this: Allocated<Self>,
            item_count: NSInteger,
            r#type: NSMenuItemBadgeType,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCount:)]
        pub unsafe fn initWithCount(this: Allocated<Self>, item_count: NSInteger)
            -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithString:)]
        pub unsafe fn initWithString(this: Allocated<Self>, string: &NSString) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(itemCount)]
        pub unsafe fn itemCount(&self) -> NSInteger;

        #[method(type)]
        pub unsafe fn r#type(&self) -> NSMenuItemBadgeType;

        #[method_id(@__retain_semantics Other stringValue)]
        pub unsafe fn stringValue(&self) -> Option<Retained<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSMenuItemBadge {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

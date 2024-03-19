//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

#[cfg(feature = "Foundation_NSString")]
typed_extensible_enum!(
    pub type NSTouchBarItemIdentifier = NSString;
);

typed_extensible_enum!(
    pub type NSTouchBarItemPriority = c_float;
);

pub static NSTouchBarItemPriorityHigh: NSTouchBarItemPriority = 1000 as _;

pub static NSTouchBarItemPriorityNormal: NSTouchBarItemPriority = 0 as _;

pub static NSTouchBarItemPriorityLow: NSTouchBarItemPriority = -1000 as _;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTouchBarItem;

    unsafe impl ClassType for NSTouchBarItem {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSTouchBarItem {}

unsafe impl NSObjectProtocol for NSTouchBarItem {}

extern_methods!(
    unsafe impl NSTouchBarItem {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Allocated<Self>,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSTouchBarItemIdentifier>;

        #[method(visibilityPriority)]
        pub unsafe fn visibilityPriority(&self) -> NSTouchBarItemPriority;

        #[method(setVisibilityPriority:)]
        pub unsafe fn setVisibilityPriority(&self, visibility_priority: NSTouchBarItemPriority);

        #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Option<Id<NSView>>;

        #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
        #[method_id(@__retain_semantics Other viewController)]
        pub unsafe fn viewController(&self) -> Option<Id<NSViewController>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString>;

        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTouchBarItem {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSTouchBarItemIdentifierFixedSpaceSmall: &'static NSTouchBarItemIdentifier;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSTouchBarItemIdentifierFixedSpaceLarge: &'static NSTouchBarItemIdentifier;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSTouchBarItemIdentifierFlexibleSpace: &'static NSTouchBarItemIdentifier;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSTouchBarItemIdentifierOtherItemsProxy: &'static NSTouchBarItemIdentifier;
}

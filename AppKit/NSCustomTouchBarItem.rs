//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCustomTouchBarItem;

    unsafe impl ClassType for NSCustomTouchBarItem {
        #[inherits(NSObject)]
        type Super = NSTouchBarItem;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSCustomTouchBarItem")]
    unsafe impl NSCustomTouchBarItem {
        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Id<NSView, Shared>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setView:)]
        pub unsafe fn setView(&self, view: &NSView);

        #[cfg(feature = "AppKit_NSViewController")]
        #[method_id(@__retain_semantics Other viewController)]
        pub unsafe fn viewController(&self) -> Option<Id<NSViewController, Shared>>;

        #[cfg(feature = "AppKit_NSViewController")]
        #[method(setViewController:)]
        pub unsafe fn setViewController(&self, viewController: Option<&NSViewController>);

        #[method_id(@__retain_semantics Other customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString, Shared>;

        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customizationLabel: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTouchBarItem`
    #[cfg(feature = "AppKit_NSCustomTouchBarItem")]
    unsafe impl NSCustomTouchBarItem {
        #[cfg(feature = "AppKit_NSTouchBarItemIdentifier")]
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self, Shared>;
    }
);

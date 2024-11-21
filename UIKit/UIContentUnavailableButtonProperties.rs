//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIContentUnavailableButtonProperties;
);

unsafe impl NSCoding for UIContentUnavailableButtonProperties {}

unsafe impl NSCopying for UIContentUnavailableButtonProperties {}

unsafe impl CopyingHelper for UIContentUnavailableButtonProperties {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIContentUnavailableButtonProperties {}

unsafe impl NSSecureCoding for UIContentUnavailableButtonProperties {}

extern_methods!(
    unsafe impl UIContentUnavailableButtonProperties {
        #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
        #[method_id(@__retain_semantics Other primaryAction)]
        pub unsafe fn primaryAction(&self) -> Option<Retained<UIAction>>;

        #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
        #[method(setPrimaryAction:)]
        pub unsafe fn setPrimaryAction(&self, primary_action: Option<&UIAction>);

        #[cfg(all(feature = "UIMenu", feature = "UIMenuElement"))]
        #[method_id(@__retain_semantics Other menu)]
        pub unsafe fn menu(&self) -> Option<Retained<UIMenu>>;

        #[cfg(all(feature = "UIMenu", feature = "UIMenuElement"))]
        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: Option<&UIMenu>);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[cfg(feature = "UIButton")]
        #[method(role)]
        pub unsafe fn role(&self) -> UIButtonRole;

        #[cfg(feature = "UIButton")]
        #[method(setRole:)]
        pub unsafe fn setRole(&self, role: UIButtonRole);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIContentUnavailableButtonProperties {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

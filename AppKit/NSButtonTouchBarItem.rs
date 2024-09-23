//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSTouchBarItem")]
    pub struct NSButtonTouchBarItem;

    #[cfg(feature = "NSTouchBarItem")]
    unsafe impl ClassType for NSButtonTouchBarItem {
        #[inherits(NSObject)]
        type Super = NSTouchBarItem;
    }
);

#[cfg(feature = "NSTouchBarItem")]
unsafe impl NSCoding for NSButtonTouchBarItem {}

#[cfg(feature = "NSTouchBarItem")]
unsafe impl NSObjectProtocol for NSButtonTouchBarItem {}

extern_methods!(
    #[cfg(feature = "NSTouchBarItem")]
    unsafe impl NSButtonTouchBarItem {
        #[method_id(@__retain_semantics Other buttonTouchBarItemWithIdentifier:title:target:action:)]
        pub unsafe fn buttonTouchBarItemWithIdentifier_title_target_action(
            identifier: &NSTouchBarItemIdentifier,
            title: &NSString,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other buttonTouchBarItemWithIdentifier:image:target:action:)]
        pub unsafe fn buttonTouchBarItemWithIdentifier_image_target_action(
            identifier: &NSTouchBarItemIdentifier,
            image: &NSImage,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other buttonTouchBarItemWithIdentifier:title:image:target:action:)]
        pub unsafe fn buttonTouchBarItemWithIdentifier_title_image_target_action(
            identifier: &NSTouchBarItemIdentifier,
            title: &NSString,
            image: &NSImage,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Retained<NSImage>>;

        #[cfg(feature = "NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other bezelColor)]
        pub unsafe fn bezelColor(&self) -> Option<Retained<NSColor>>;

        #[cfg(feature = "NSColor")]
        #[method(setBezelColor:)]
        pub unsafe fn setBezelColor(&self, bezel_color: Option<&NSColor>);

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Retained<AnyObject>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&AnyObject>);

        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;

        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method_id(@__retain_semantics Other customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Retained<NSString>;

        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customization_label: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTouchBarItem`
    #[cfg(feature = "NSTouchBarItem")]
    unsafe impl NSButtonTouchBarItem {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Allocated<Self>,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSTouchBarItem")]
    unsafe impl NSButtonTouchBarItem {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

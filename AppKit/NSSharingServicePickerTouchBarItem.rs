//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssharingservicepickertouchbaritem?language=objc)
    #[unsafe(super(NSTouchBarItem, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSTouchBarItem")]
    pub struct NSSharingServicePickerTouchBarItem;
);

#[cfg(feature = "NSTouchBarItem")]
unsafe impl NSCoding for NSSharingServicePickerTouchBarItem {}

#[cfg(feature = "NSTouchBarItem")]
unsafe impl NSObjectProtocol for NSSharingServicePickerTouchBarItem {}

extern_methods!(
    #[cfg(feature = "NSTouchBarItem")]
    unsafe impl NSSharingServicePickerTouchBarItem {
        #[cfg(feature = "NSSharingService")]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSSharingServicePickerTouchBarItemDelegate>>>;

        #[cfg(feature = "NSSharingService")]
        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSSharingServicePickerTouchBarItemDelegate>>,
        );

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method_id(@__retain_semantics Other buttonTitle)]
        pub unsafe fn buttonTitle(&self) -> Retained<NSString>;

        #[method(setButtonTitle:)]
        pub unsafe fn setButtonTitle(&self, button_title: &NSString);

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other buttonImage)]
        pub unsafe fn buttonImage(&self) -> Option<Retained<NSImage>>;

        #[cfg(feature = "NSImage")]
        #[method(setButtonImage:)]
        pub unsafe fn setButtonImage(&self, button_image: Option<&NSImage>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTouchBarItem`
    #[cfg(feature = "NSTouchBarItem")]
    unsafe impl NSSharingServicePickerTouchBarItem {
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
    unsafe impl NSSharingServicePickerTouchBarItem {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssharingservicepickertouchbaritemdelegate?language=objc)
    #[cfg(feature = "NSSharingService")]
    pub unsafe trait NSSharingServicePickerTouchBarItemDelegate:
        NSSharingServicePickerDelegate + MainThreadOnly
    {
        #[cfg(feature = "NSTouchBarItem")]
        #[method_id(@__retain_semantics Other itemsForSharingServicePickerTouchBarItem:)]
        unsafe fn itemsForSharingServicePickerTouchBarItem(
            &self,
            picker_touch_bar_item: &NSSharingServicePickerTouchBarItem,
        ) -> Retained<NSArray>;
    }

    #[cfg(feature = "NSSharingService")]
    unsafe impl ProtocolType for dyn NSSharingServicePickerTouchBarItemDelegate {}
);

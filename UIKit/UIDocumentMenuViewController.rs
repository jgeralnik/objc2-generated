//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidocumentmenuorder?language=objc)
// NS_ENUM
#[deprecated]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIDocumentMenuOrder(pub NSUInteger);
impl UIDocumentMenuOrder {
    #[deprecated]
    #[doc(alias = "UIDocumentMenuOrderFirst")]
    pub const First: Self = Self(0);
    #[deprecated]
    #[doc(alias = "UIDocumentMenuOrderLast")]
    pub const Last: Self = Self(1);
}

unsafe impl Encode for UIDocumentMenuOrder {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIDocumentMenuOrder {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidocumentmenudelegate?language=objc)
    #[deprecated = "UIDocumentMenuDelegate is deprecated. Use UIDocumentPickerViewController directly."]
    pub unsafe trait UIDocumentMenuDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(
            feature = "UIDocumentPickerViewController",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[deprecated = "UIDocumentMenuDelegate is deprecated. Use UIDocumentPickerViewController directly."]
        #[method(documentMenu:didPickDocumentPicker:)]
        unsafe fn documentMenu_didPickDocumentPicker(
            &self,
            document_menu: &UIDocumentMenuViewController,
            document_picker: &UIDocumentPickerViewController,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[deprecated = "UIDocumentMenuDelegate is deprecated. Use UIDocumentPickerViewController directly."]
        #[optional]
        #[method(documentMenuWasCancelled:)]
        unsafe fn documentMenuWasCancelled(&self, document_menu: &UIDocumentMenuViewController);
    }

    unsafe impl ProtocolType for dyn UIDocumentMenuDelegate {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidocumentmenuviewcontroller?language=objc)
    #[unsafe(super(UIViewController, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    #[deprecated = "UIDocumentMenuViewController is deprecated. Use UIDocumentPickerViewController directly."]
    pub struct UIDocumentMenuViewController;
);

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSCoding for UIDocumentMenuViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSObjectProtocol for UIDocumentMenuViewController {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIAppearanceContainer for UIDocumentMenuViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIContentContainer for UIDocumentMenuViewController {}

#[cfg(all(
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIFocusEnvironment for UIDocumentMenuViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIResponderStandardEditActions for UIDocumentMenuViewController {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIViewController"
))]
unsafe impl UITraitEnvironment for UIDocumentMenuViewController {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIDocumentMenuViewController {
        #[cfg(feature = "UIDocumentPickerViewController")]
        #[deprecated = "UIDocumentMenuViewController is deprecated. Use UIDocumentPickerViewController directly."]
        #[method_id(@__retain_semantics Init initWithDocumentTypes:inMode:)]
        pub unsafe fn initWithDocumentTypes_inMode(
            this: Allocated<Self>,
            allowed_ut_is: &NSArray<NSString>,
            mode: UIDocumentPickerMode,
        ) -> Retained<Self>;

        #[cfg(feature = "UIDocumentPickerViewController")]
        #[deprecated = "UIDocumentMenuViewController is deprecated. Use UIDocumentPickerViewController directly."]
        #[method_id(@__retain_semantics Init initWithURL:inMode:)]
        pub unsafe fn initWithURL_inMode(
            this: Allocated<Self>,
            url: &NSURL,
            mode: UIDocumentPickerMode,
        ) -> Retained<Self>;

        #[deprecated = "UIDocumentMenuViewController is deprecated. Use UIDocumentPickerViewController directly."]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "UIImage", feature = "block2"))]
        #[deprecated = "UIDocumentMenuViewController is deprecated. Use UIDocumentPickerViewController directly."]
        #[method(addOptionWithTitle:image:order:handler:)]
        pub unsafe fn addOptionWithTitle_image_order_handler(
            &self,
            title: &NSString,
            image: Option<&UIImage>,
            order: UIDocumentMenuOrder,
            handler: &block2::Block<dyn Fn()>,
        );

        #[deprecated = "UIDocumentMenuViewController is deprecated. Use UIDocumentPickerViewController directly."]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIDocumentMenuDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[deprecated = "UIDocumentMenuViewController is deprecated. Use UIDocumentPickerViewController directly."]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UIDocumentMenuDelegate>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `UIViewController`
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIDocumentMenuViewController {
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSString>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIDocumentMenuViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

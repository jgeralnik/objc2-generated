//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_EXTENSIBLE_ENUM
pub type UIActionIdentifier = NSString;

extern "C" {
    pub static UIActionPaste: &'static UIActionIdentifier;
}

extern "C" {
    pub static UIActionPasteAndMatchStyle: &'static UIActionIdentifier;
}

extern "C" {
    pub static UIActionPasteAndGo: &'static UIActionIdentifier;
}

extern "C" {
    pub static UIActionPasteAndSearch: &'static UIActionIdentifier;
}

#[cfg(all(feature = "UIMenuElement", feature = "block2"))]
pub type UIActionHandler = *mut block2::Block<dyn Fn(NonNull<UIAction>)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIMenuElement")]
    pub struct UIAction;

    #[cfg(feature = "UIMenuElement")]
    unsafe impl ClassType for UIAction {
        #[inherits(NSObject)]
        type Super = UIMenuElement;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "UIMenuElement")]
unsafe impl NSCoding for UIAction {}

#[cfg(feature = "UIMenuElement")]
unsafe impl NSCopying for UIAction {}

#[cfg(feature = "UIMenuElement")]
unsafe impl CopyingHelper for UIAction {
    type Result = Self;
}

#[cfg(feature = "UIMenuElement")]
unsafe impl NSObjectProtocol for UIAction {}

#[cfg(feature = "UIMenuElement")]
unsafe impl NSSecureCoding for UIAction {}

#[cfg(all(feature = "UIMenuElement", feature = "UIMenuLeaf"))]
unsafe impl UIMenuLeaf for UIAction {}

extern_methods!(
    #[cfg(feature = "UIMenuElement")]
    unsafe impl UIAction {
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&UIImage>);

        #[method_id(@__retain_semantics Other discoverabilityTitle)]
        pub unsafe fn discoverabilityTitle(&self) -> Option<Retained<NSString>>;

        #[method(setDiscoverabilityTitle:)]
        pub unsafe fn setDiscoverabilityTitle(&self, discoverability_title: Option<&NSString>);

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<UIActionIdentifier>;

        #[method(attributes)]
        pub unsafe fn attributes(&self) -> UIMenuElementAttributes;

        #[method(setAttributes:)]
        pub unsafe fn setAttributes(&self, attributes: UIMenuElementAttributes);

        #[method(state)]
        pub unsafe fn state(&self) -> UIMenuElementState;

        #[method(setState:)]
        pub unsafe fn setState(&self, state: UIMenuElementState);

        #[method_id(@__retain_semantics Other sender)]
        pub unsafe fn sender(&self) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other actionWithHandler:)]
        pub unsafe fn actionWithHandler(
            handler: UIActionHandler,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(all(feature = "UIImage", feature = "block2"))]
        #[method_id(@__retain_semantics Other actionWithTitle:image:identifier:handler:)]
        pub unsafe fn actionWithTitle_image_identifier_handler(
            title: &NSString,
            image: Option<&UIImage>,
            identifier: Option<&UIActionIdentifier>,
            handler: UIActionHandler,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIMenuElement`
    #[cfg(feature = "UIMenuElement")]
    unsafe impl UIAction {
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// UICaptureTextFromCameraSupporting
    #[cfg(feature = "UIMenuElement")]
    unsafe impl UIAction {
        #[cfg(all(
            feature = "UIResponder",
            feature = "UITextInput",
            feature = "UITextInputTraits"
        ))]
        #[method_id(@__retain_semantics Other captureTextFromCameraActionForResponder:identifier:)]
        pub unsafe fn captureTextFromCameraActionForResponder_identifier(
            responder: &UIResponder,
            identifier: Option<&UIActionIdentifier>,
        ) -> Retained<Self>;
    }
);

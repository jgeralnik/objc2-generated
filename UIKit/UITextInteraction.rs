//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITextInteractionMode(pub NSInteger);
impl UITextInteractionMode {
    #[doc(alias = "UITextInteractionModeEditable")]
    pub const Editable: Self = Self(0);
    #[doc(alias = "UITextInteractionModeNonEditable")]
    pub const NonEditable: Self = Self(1);
}

unsafe impl Encode for UITextInteractionMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITextInteractionMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait UITextInteractionDelegate: NSObjectProtocol + MainThreadOnly {
        #[optional]
        #[method(interactionShouldBegin:atPoint:)]
        unsafe fn interactionShouldBegin_atPoint(
            &self,
            interaction: &UITextInteraction,
            point: CGPoint,
        ) -> bool;

        #[optional]
        #[method(interactionWillBegin:)]
        unsafe fn interactionWillBegin(&self, interaction: &UITextInteraction);

        #[optional]
        #[method(interactionDidEnd:)]
        unsafe fn interactionDidEnd(&self, interaction: &UITextInteraction);
    }

    unsafe impl ProtocolType for dyn UITextInteractionDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITextInteraction;

    unsafe impl ClassType for UITextInteraction {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UITextInteraction {}

#[cfg(feature = "UIInteraction")]
unsafe impl UIInteraction for UITextInteraction {}

extern_methods!(
    unsafe impl UITextInteraction {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UITextInteractionDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UITextInteractionDelegate>>,
        );

        #[cfg(all(
            feature = "UIResponder",
            feature = "UITextInput",
            feature = "UITextInputTraits"
        ))]
        #[method_id(@__retain_semantics Other textInput)]
        pub unsafe fn textInput(&self) -> Option<Retained<UIResponder>>;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UITextInput",
            feature = "UITextInputTraits"
        ))]
        #[method(setTextInput:)]
        pub unsafe fn setTextInput(&self, text_input: Option<&UIResponder>);

        #[method(textInteractionMode)]
        pub unsafe fn textInteractionMode(&self) -> UITextInteractionMode;

        #[cfg(feature = "UIGestureRecognizer")]
        #[method_id(@__retain_semantics Other gesturesForFailureRequirements)]
        pub unsafe fn gesturesForFailureRequirements(
            &self,
        ) -> Retained<NSArray<UIGestureRecognizer>>;

        #[method_id(@__retain_semantics Other textInteractionForMode:)]
        pub unsafe fn textInteractionForMode(
            mode: UITextInteractionMode,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITextInteraction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

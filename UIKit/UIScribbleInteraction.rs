//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiscribbleinteraction?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIScribbleInteraction;
);

unsafe impl NSObjectProtocol for UIScribbleInteraction {}

#[cfg(feature = "UIInteraction")]
unsafe impl UIInteraction for UIScribbleInteraction {}

extern_methods!(
    unsafe impl UIScribbleInteraction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDelegate:)]
        pub unsafe fn initWithDelegate(
            this: Allocated<Self>,
            delegate: &ProtocolObject<dyn UIScribbleInteractionDelegate>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIScribbleInteractionDelegate>>>;

        #[method(isHandlingWriting)]
        pub unsafe fn isHandlingWriting(&self) -> bool;

        #[method(isPencilInputExpected)]
        pub unsafe fn isPencilInputExpected(mtm: MainThreadMarker) -> bool;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiscribbleinteractiondelegate?language=objc)
    pub unsafe trait UIScribbleInteractionDelegate:
        NSObjectProtocol + MainThreadOnly
    {
        #[optional]
        #[method(scribbleInteraction:shouldBeginAtLocation:)]
        unsafe fn scribbleInteraction_shouldBeginAtLocation(
            &self,
            interaction: &UIScribbleInteraction,
            location: CGPoint,
        ) -> bool;

        #[optional]
        #[method(scribbleInteractionShouldDelayFocus:)]
        unsafe fn scribbleInteractionShouldDelayFocus(
            &self,
            interaction: &UIScribbleInteraction,
        ) -> bool;

        #[optional]
        #[method(scribbleInteractionWillBeginWriting:)]
        unsafe fn scribbleInteractionWillBeginWriting(&self, interaction: &UIScribbleInteraction);

        #[optional]
        #[method(scribbleInteractionDidFinishWriting:)]
        unsafe fn scribbleInteractionDidFinishWriting(&self, interaction: &UIScribbleInteraction);
    }

    unsafe impl ProtocolType for dyn UIScribbleInteractionDelegate {}
);

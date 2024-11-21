//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIFeedbackGenerator;
);

unsafe impl NSObjectProtocol for UIFeedbackGenerator {}

extern_methods!(
    unsafe impl UIFeedbackGenerator {
        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other feedbackGeneratorForView:)]
        pub unsafe fn feedbackGeneratorForView(view: &UIView) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(prepare)]
        pub unsafe fn prepare(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIFeedbackGenerator {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    unsafe impl UIFeedbackGenerator {}
);

#[cfg(feature = "UIInteraction")]
unsafe impl UIInteraction for UIFeedbackGenerator {}

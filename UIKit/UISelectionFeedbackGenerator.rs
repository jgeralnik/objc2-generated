//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIFeedbackGenerator")]
    pub struct UISelectionFeedbackGenerator;

    #[cfg(feature = "UIFeedbackGenerator")]
    unsafe impl ClassType for UISelectionFeedbackGenerator {
        #[inherits(NSObject)]
        type Super = UIFeedbackGenerator;
        type ThreadKind = dyn MainThreadOnly;
    }
);

#[cfg(feature = "UIFeedbackGenerator")]
unsafe impl NSObjectProtocol for UISelectionFeedbackGenerator {}

extern_methods!(
    #[cfg(feature = "UIFeedbackGenerator")]
    unsafe impl UISelectionFeedbackGenerator {
        #[method(selectionChanged)]
        pub unsafe fn selectionChanged(&self);

        #[method(selectionChangedAtLocation:)]
        pub unsafe fn selectionChangedAtLocation(&self, location: CGPoint);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIFeedbackGenerator`
    #[cfg(feature = "UIFeedbackGenerator")]
    unsafe impl UISelectionFeedbackGenerator {
        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other feedbackGeneratorForView:)]
        pub unsafe fn feedbackGeneratorForView(view: &UIView) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIFeedbackGenerator")]
    unsafe impl UISelectionFeedbackGenerator {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

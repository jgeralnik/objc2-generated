//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsalignmentfeedbacktoken?language=objc)
    pub unsafe trait NSAlignmentFeedbackToken: NSObjectProtocol {}

    unsafe impl ProtocolType for dyn NSAlignmentFeedbackToken {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsalignmentfeedbackfilter?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAlignmentFeedbackFilter;
);

unsafe impl NSObjectProtocol for NSAlignmentFeedbackFilter {}

extern_methods!(
    unsafe impl NSAlignmentFeedbackFilter {
        #[cfg(feature = "NSEvent")]
        #[method(inputEventMask)]
        pub unsafe fn inputEventMask() -> NSEventMask;

        #[cfg(feature = "NSEvent")]
        #[method(updateWithEvent:)]
        pub unsafe fn updateWithEvent(&self, event: &NSEvent);

        #[cfg(all(feature = "NSGestureRecognizer", feature = "NSPanGestureRecognizer"))]
        #[method(updateWithPanRecognizer:)]
        pub unsafe fn updateWithPanRecognizer(&self, pan_recognizer: &NSPanGestureRecognizer);

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other alignmentFeedbackTokenForMovementInView:previousPoint:alignedPoint:defaultPoint:)]
        pub unsafe fn alignmentFeedbackTokenForMovementInView_previousPoint_alignedPoint_defaultPoint(
            &self,
            view: Option<&NSView>,
            previous_point: NSPoint,
            aligned_point: NSPoint,
            default_point: NSPoint,
        ) -> Option<Retained<ProtocolObject<dyn NSAlignmentFeedbackToken>>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other alignmentFeedbackTokenForHorizontalMovementInView:previousX:alignedX:defaultX:)]
        pub unsafe fn alignmentFeedbackTokenForHorizontalMovementInView_previousX_alignedX_defaultX(
            &self,
            view: Option<&NSView>,
            previous_x: CGFloat,
            aligned_x: CGFloat,
            default_x: CGFloat,
        ) -> Option<Retained<ProtocolObject<dyn NSAlignmentFeedbackToken>>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other alignmentFeedbackTokenForVerticalMovementInView:previousY:alignedY:defaultY:)]
        pub unsafe fn alignmentFeedbackTokenForVerticalMovementInView_previousY_alignedY_defaultY(
            &self,
            view: Option<&NSView>,
            previous_y: CGFloat,
            aligned_y: CGFloat,
            default_y: CGFloat,
        ) -> Option<Retained<ProtocolObject<dyn NSAlignmentFeedbackToken>>>;

        #[cfg(feature = "NSHapticFeedback")]
        #[method(performFeedback:performanceTime:)]
        pub unsafe fn performFeedback_performanceTime(
            &self,
            alignment_feedback_tokens: &NSArray<ProtocolObject<dyn NSAlignmentFeedbackToken>>,
            performance_time: NSHapticFeedbackPerformanceTime,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSAlignmentFeedbackFilter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uifocusmovementhint?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIFocusMovementHint;
);

unsafe impl NSCopying for UIFocusMovementHint {}

unsafe impl CopyingHelper for UIFocusMovementHint {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIFocusMovementHint {}

extern_methods!(
    unsafe impl UIFocusMovementHint {
        #[method(movementDirection)]
        pub unsafe fn movementDirection(&self) -> CGVector;

        #[cfg(feature = "objc2-quartz-core")]
        #[cfg(not(target_os = "watchos"))]
        #[method(perspectiveTransform)]
        pub unsafe fn perspectiveTransform(&self) -> CATransform3D;

        #[method(rotation)]
        pub unsafe fn rotation(&self) -> CGVector;

        #[method(translation)]
        pub unsafe fn translation(&self) -> CGVector;

        #[cfg(feature = "objc2-quartz-core")]
        #[cfg(not(target_os = "watchos"))]
        #[method(interactionTransform)]
        pub unsafe fn interactionTransform(&self) -> CATransform3D;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

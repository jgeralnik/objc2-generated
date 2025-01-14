//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;
use objc2_local_authentication::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/localauthenticationembeddedui/laauthenticationview?language=objc)
    #[unsafe(super(NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    pub struct LAAuthenticationView;
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSAccessibility for LAAuthenticationView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSAccessibilityElementProtocol for LAAuthenticationView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSAnimatablePropertyContainer for LAAuthenticationView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSAppearanceCustomization for LAAuthenticationView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSCoding for LAAuthenticationView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSDraggingDestination for LAAuthenticationView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSObjectProtocol for LAAuthenticationView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSUserInterfaceItemIdentification for LAAuthenticationView {}

extern_methods!(
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl LAAuthenticationView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithContext:)]
        pub unsafe fn initWithContext(this: Allocated<Self>, context: &LAContext)
            -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithContext:controlSize:)]
        pub unsafe fn initWithContext_controlSize(
            this: Allocated<Self>,
            context: &LAContext,
            control_size: NSControlSize,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other context)]
        pub unsafe fn context(&self) -> Retained<LAContext>;

        #[method(controlSize)]
        pub unsafe fn controlSize(&self) -> NSControlSize;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl LAAuthenticationView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl LAAuthenticationView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

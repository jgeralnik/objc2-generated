//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(UIWindowScenePlacement, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIWindowScenePlacement")]
    #[deprecated = "UIWindowSceneReplacePlacement has been replaced with UIWindowScenePushPlacement"]
    pub struct UIWindowSceneReplacePlacement;
);

#[cfg(feature = "UIWindowScenePlacement")]
unsafe impl NSCopying for UIWindowSceneReplacePlacement {}

#[cfg(feature = "UIWindowScenePlacement")]
unsafe impl CopyingHelper for UIWindowSceneReplacePlacement {
    type Result = Self;
}

#[cfg(feature = "UIWindowScenePlacement")]
unsafe impl NSObjectProtocol for UIWindowSceneReplacePlacement {}

extern_methods!(
    #[cfg(feature = "UIWindowScenePlacement")]
    unsafe impl UIWindowSceneReplacePlacement {
        #[cfg(feature = "UISceneSession")]
        #[deprecated = "UIWindowSceneReplacePlacement has been replaced with UIWindowScenePushPlacement"]
        #[method_id(@__retain_semantics Other placementToReplaceSceneSession:)]
        pub unsafe fn placementToReplaceSceneSession(
            scene_session: &UISceneSession,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIWindowScenePlacement`
    #[cfg(feature = "UIWindowScenePlacement")]
    unsafe impl UIWindowSceneReplacePlacement {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

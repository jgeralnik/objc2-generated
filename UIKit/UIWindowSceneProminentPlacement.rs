//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIWindowScenePlacement")]
    pub struct UIWindowSceneProminentPlacement;

    #[cfg(feature = "UIWindowScenePlacement")]
    unsafe impl ClassType for UIWindowSceneProminentPlacement {
        #[inherits(NSObject)]
        type Super = UIWindowScenePlacement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "UIWindowScenePlacement")]
unsafe impl NSCopying for UIWindowSceneProminentPlacement {}

#[cfg(feature = "UIWindowScenePlacement")]
unsafe impl CopyingHelper for UIWindowSceneProminentPlacement {
    type Result = Self;
}

#[cfg(feature = "UIWindowScenePlacement")]
unsafe impl NSObjectProtocol for UIWindowSceneProminentPlacement {}

extern_methods!(
    #[cfg(feature = "UIWindowScenePlacement")]
    unsafe impl UIWindowSceneProminentPlacement {
        #[method_id(@__retain_semantics Other prominentPlacement)]
        pub unsafe fn prominentPlacement() -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIWindowScenePlacement`
    #[cfg(feature = "UIWindowScenePlacement")]
    unsafe impl UIWindowSceneProminentPlacement {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

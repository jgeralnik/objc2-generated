//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UISceneWindowingBehaviors;

    unsafe impl ClassType for UISceneWindowingBehaviors {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UISceneWindowingBehaviors {}

extern_methods!(
    unsafe impl UISceneWindowingBehaviors {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method(isClosable)]
        pub unsafe fn isClosable(&self) -> bool;

        #[method(setClosable:)]
        pub unsafe fn setClosable(&self, closable: bool);

        #[method(isMiniaturizable)]
        pub unsafe fn isMiniaturizable(&self) -> bool;

        #[method(setMiniaturizable:)]
        pub unsafe fn setMiniaturizable(&self, miniaturizable: bool);
    }
);

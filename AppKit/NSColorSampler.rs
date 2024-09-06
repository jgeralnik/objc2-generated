//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSColorSampler;

    unsafe impl ClassType for NSColorSampler {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for NSColorSampler {}

extern_methods!(
    unsafe impl NSColorSampler {
        #[cfg(all(feature = "NSColor", feature = "block2"))]
        #[method(showSamplerWithSelectionHandler:)]
        pub unsafe fn showSamplerWithSelectionHandler(
            &self,
            selection_handler: &block2::Block<dyn Fn(*mut NSColor)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSColorSampler {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIImageAccumulator;
);

unsafe impl NSObjectProtocol for CIImageAccumulator {}

extern_methods!(
    unsafe impl CIImageAccumulator {
        #[cfg(feature = "CIImage")]
        #[method_id(@__retain_semantics Other imageAccumulatorWithExtent:format:)]
        pub unsafe fn imageAccumulatorWithExtent_format(
            extent: CGRect,
            format: CIFormat,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "CIImage")]
        #[method_id(@__retain_semantics Init initWithExtent:format:)]
        pub unsafe fn initWithExtent_format(
            this: Allocated<Self>,
            extent: CGRect,
            format: CIFormat,
        ) -> Option<Retained<Self>>;

        #[method(extent)]
        pub unsafe fn extent(&self) -> CGRect;

        #[cfg(feature = "CIImage")]
        #[method(format)]
        pub unsafe fn format(&self) -> CIFormat;

        #[cfg(feature = "CIImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Retained<CIImage>;

        #[cfg(feature = "CIImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: &CIImage);

        #[cfg(feature = "CIImage")]
        #[method(setImage:dirtyRect:)]
        pub unsafe fn setImage_dirtyRect(&self, image: &CIImage, dirty_rect: CGRect);

        #[method(clear)]
        pub unsafe fn clear(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIImageAccumulator {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

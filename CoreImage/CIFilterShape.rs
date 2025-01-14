//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/cifiltershape?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIFilterShape;
);

unsafe impl NSCopying for CIFilterShape {}

unsafe impl CopyingHelper for CIFilterShape {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CIFilterShape {}

extern_methods!(
    unsafe impl CIFilterShape {
        #[method_id(@__retain_semantics Other shapeWithRect:)]
        pub unsafe fn shapeWithRect(r: CGRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithRect:)]
        pub unsafe fn initWithRect(this: Allocated<Self>, r: CGRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Other insetByX:Y:)]
        pub unsafe fn insetByX_Y(&self, dx: c_int, dy: c_int) -> Retained<CIFilterShape>;

        #[method_id(@__retain_semantics Other unionWith:)]
        pub unsafe fn unionWith(&self, s2: &CIFilterShape) -> Retained<CIFilterShape>;

        #[method_id(@__retain_semantics Other unionWithRect:)]
        pub unsafe fn unionWithRect(&self, r: CGRect) -> Retained<CIFilterShape>;

        #[method_id(@__retain_semantics Other intersectWith:)]
        pub unsafe fn intersectWith(&self, s2: &CIFilterShape) -> Retained<CIFilterShape>;

        #[method_id(@__retain_semantics Other intersectWithRect:)]
        pub unsafe fn intersectWithRect(&self, r: CGRect) -> Retained<CIFilterShape>;

        #[method(extent)]
        pub unsafe fn extent(&self) -> CGRect;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIFilterShape {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

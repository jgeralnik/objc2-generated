//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UICalendarViewDecorationSize(pub NSInteger);
impl UICalendarViewDecorationSize {
    #[doc(alias = "UICalendarViewDecorationSizeSmall")]
    pub const Small: Self = Self(0);
    #[doc(alias = "UICalendarViewDecorationSizeMedium")]
    pub const Medium: Self = Self(1);
    #[doc(alias = "UICalendarViewDecorationSizeLarge")]
    pub const Large: Self = Self(2);
}

unsafe impl Encode for UICalendarViewDecorationSize {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UICalendarViewDecorationSize {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UICalendarViewDecoration;

    unsafe impl ClassType for UICalendarViewDecoration {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UICalendarViewDecoration {}

extern_methods!(
    unsafe impl UICalendarViewDecoration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(feature = "UIColor", feature = "UIImage"))]
        #[method_id(@__retain_semantics Init initWithImage:color:size:)]
        pub unsafe fn initWithImage_color_size(
            this: Allocated<Self>,
            image: Option<&UIImage>,
            color: Option<&UIColor>,
            size: UICalendarViewDecorationSize,
        ) -> Retained<Self>;

        #[cfg(all(feature = "UIResponder", feature = "UIView", feature = "block2"))]
        #[method_id(@__retain_semantics Init initWithCustomViewProvider:)]
        pub unsafe fn initWithCustomViewProvider(
            this: Allocated<Self>,
            custom_view_provider: &block2::Block<dyn Fn() -> NonNull<UIView>>,
        ) -> Retained<Self>;

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other decorationWithColor:size:)]
        pub unsafe fn decorationWithColor_size(
            color: Option<&UIColor>,
            size: UICalendarViewDecorationSize,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other decorationWithImage:)]
        pub unsafe fn decorationWithImage(
            image: Option<&UIImage>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(all(feature = "UIColor", feature = "UIImage"))]
        #[method_id(@__retain_semantics Other decorationWithImage:color:size:)]
        pub unsafe fn decorationWithImage_color_size(
            image: Option<&UIImage>,
            color: Option<&UIColor>,
            size: UICalendarViewDecorationSize,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(all(feature = "UIResponder", feature = "UIView", feature = "block2"))]
        #[method_id(@__retain_semantics Other decorationWithCustomViewProvider:)]
        pub unsafe fn decorationWithCustomViewProvider(
            custom_view_provider: &block2::Block<dyn Fn() -> NonNull<UIView>>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UICalendarViewDecoration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

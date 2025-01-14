//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uibaritem?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIBarItem;
);

unsafe impl NSCoding for UIBarItem {}

unsafe impl NSObjectProtocol for UIBarItem {}

#[cfg(feature = "UIAppearance")]
unsafe impl UIAppearance for UIBarItem {}

extern_methods!(
    unsafe impl UIBarItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Retained<NSString>>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&UIImage>);

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other landscapeImagePhone)]
        pub unsafe fn landscapeImagePhone(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method(setLandscapeImagePhone:)]
        pub unsafe fn setLandscapeImagePhone(&self, landscape_image_phone: Option<&UIImage>);

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other largeContentSizeImage)]
        pub unsafe fn largeContentSizeImage(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method(setLargeContentSizeImage:)]
        pub unsafe fn setLargeContentSizeImage(&self, large_content_size_image: Option<&UIImage>);

        #[cfg(feature = "UIGeometry")]
        #[method(imageInsets)]
        pub unsafe fn imageInsets(&self) -> UIEdgeInsets;

        #[cfg(feature = "UIGeometry")]
        #[method(setImageInsets:)]
        pub unsafe fn setImageInsets(&self, image_insets: UIEdgeInsets);

        #[cfg(feature = "UIGeometry")]
        #[method(landscapeImagePhoneInsets)]
        pub unsafe fn landscapeImagePhoneInsets(&self) -> UIEdgeInsets;

        #[cfg(feature = "UIGeometry")]
        #[method(setLandscapeImagePhoneInsets:)]
        pub unsafe fn setLandscapeImagePhoneInsets(
            &self,
            landscape_image_phone_insets: UIEdgeInsets,
        );

        #[cfg(feature = "UIGeometry")]
        #[method(largeContentSizeImageInsets)]
        pub unsafe fn largeContentSizeImageInsets(&self) -> UIEdgeInsets;

        #[cfg(feature = "UIGeometry")]
        #[method(setLargeContentSizeImageInsets:)]
        pub unsafe fn setLargeContentSizeImageInsets(
            &self,
            large_content_size_image_insets: UIEdgeInsets,
        );

        #[method(tag)]
        pub unsafe fn tag(&self) -> NSInteger;

        #[method(setTag:)]
        pub unsafe fn setTag(&self, tag: NSInteger);

        #[cfg(feature = "UIControl")]
        #[method(setTitleTextAttributes:forState:)]
        pub unsafe fn setTitleTextAttributes_forState(
            &self,
            attributes: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
            state: UIControlState,
        );

        #[cfg(feature = "UIControl")]
        #[method_id(@__retain_semantics Other titleTextAttributesForState:)]
        pub unsafe fn titleTextAttributesForState(
            &self,
            state: UIControlState,
        ) -> Option<Retained<NSDictionary<NSAttributedStringKey, AnyObject>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIBarItem {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

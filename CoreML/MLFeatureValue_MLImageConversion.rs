//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coreml/mlfeaturevalueimageoption?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type MLFeatureValueImageOption = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreml/mlfeaturevalueimageoptioncroprect?language=objc)
    pub static MLFeatureValueImageOptionCropRect: &'static MLFeatureValueImageOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreml/mlfeaturevalueimageoptioncropandscale?language=objc)
    pub static MLFeatureValueImageOptionCropAndScale: &'static MLFeatureValueImageOption;
}

extern_methods!(
    /// MLImageConversion
    #[cfg(feature = "MLFeatureValue")]
    unsafe impl MLFeatureValue {
        #[method_id(@__retain_semantics Other featureValueWithImageAtURL:pixelsWide:pixelsHigh:pixelFormatType:options:error:_)]
        pub unsafe fn featureValueWithImageAtURL_pixelsWide_pixelsHigh_pixelFormatType_options_error(
            url: &NSURL,
            pixels_wide: NSInteger,
            pixels_high: NSInteger,
            pixel_format_type: OSType,
            options: Option<&NSDictionary<MLFeatureValueImageOption, AnyObject>>,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "MLImageConstraint")]
        #[method_id(@__retain_semantics Other featureValueWithImageAtURL:constraint:options:error:_)]
        pub unsafe fn featureValueWithImageAtURL_constraint_options_error(
            url: &NSURL,
            constraint: &MLImageConstraint,
            options: Option<&NSDictionary<MLFeatureValueImageOption, AnyObject>>,
        ) -> Result<Retained<Self>, Retained<NSError>>;
    }
);

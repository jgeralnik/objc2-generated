//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_ENUM
pub type CIRAWDecoderVersion = NSString;

extern "C" {
    pub static CIRAWDecoderVersionNone: &'static CIRAWDecoderVersion;
}

extern "C" {
    pub static CIRAWDecoderVersion8: &'static CIRAWDecoderVersion;
}

extern "C" {
    pub static CIRAWDecoderVersion8DNG: &'static CIRAWDecoderVersion;
}

extern "C" {
    pub static CIRAWDecoderVersion7: &'static CIRAWDecoderVersion;
}

extern "C" {
    pub static CIRAWDecoderVersion7DNG: &'static CIRAWDecoderVersion;
}

extern "C" {
    pub static CIRAWDecoderVersion6: &'static CIRAWDecoderVersion;
}

extern "C" {
    pub static CIRAWDecoderVersion6DNG: &'static CIRAWDecoderVersion;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CIFilter")]
    pub struct CIRAWFilter;

    #[cfg(feature = "CIFilter")]
    unsafe impl ClassType for CIRAWFilter {
        #[inherits(NSObject)]
        type Super = CIFilter;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CIFilter")]
unsafe impl NSCoding for CIRAWFilter {}

#[cfg(feature = "CIFilter")]
unsafe impl NSCopying for CIRAWFilter {}

#[cfg(feature = "CIFilter")]
unsafe impl CopyingHelper for CIRAWFilter {
    type Result = Self;
}

#[cfg(feature = "CIFilter")]
unsafe impl NSObjectProtocol for CIRAWFilter {}

#[cfg(feature = "CIFilter")]
unsafe impl NSSecureCoding for CIRAWFilter {}

extern_methods!(
    #[cfg(feature = "CIFilter")]
    unsafe impl CIRAWFilter {
        #[method_id(@__retain_semantics Other supportedCameraModels)]
        pub unsafe fn supportedCameraModels() -> Retained<NSArray<NSString>>;

        #[method_id(@__retain_semantics Other supportedDecoderVersions)]
        pub unsafe fn supportedDecoderVersions(&self) -> Retained<NSArray<CIRAWDecoderVersion>>;

        #[method(nativeSize)]
        pub unsafe fn nativeSize(&self) -> CGSize;

        #[method_id(@__retain_semantics Other properties)]
        pub unsafe fn properties(&self) -> Retained<NSDictionary>;

        #[method(isDraftModeEnabled)]
        pub unsafe fn isDraftModeEnabled(&self) -> bool;

        #[method(setDraftModeEnabled:)]
        pub unsafe fn setDraftModeEnabled(&self, draft_mode_enabled: bool);

        #[method_id(@__retain_semantics Other decoderVersion)]
        pub unsafe fn decoderVersion(&self) -> Retained<CIRAWDecoderVersion>;

        #[method(setDecoderVersion:)]
        pub unsafe fn setDecoderVersion(&self, decoder_version: &CIRAWDecoderVersion);

        #[method(scaleFactor)]
        pub unsafe fn scaleFactor(&self) -> c_float;

        #[method(setScaleFactor:)]
        pub unsafe fn setScaleFactor(&self, scale_factor: c_float);

        #[method(exposure)]
        pub unsafe fn exposure(&self) -> c_float;

        #[method(setExposure:)]
        pub unsafe fn setExposure(&self, exposure: c_float);

        #[method(baselineExposure)]
        pub unsafe fn baselineExposure(&self) -> c_float;

        #[method(setBaselineExposure:)]
        pub unsafe fn setBaselineExposure(&self, baseline_exposure: c_float);

        #[method(shadowBias)]
        pub unsafe fn shadowBias(&self) -> c_float;

        #[method(setShadowBias:)]
        pub unsafe fn setShadowBias(&self, shadow_bias: c_float);

        #[method(boostAmount)]
        pub unsafe fn boostAmount(&self) -> c_float;

        #[method(setBoostAmount:)]
        pub unsafe fn setBoostAmount(&self, boost_amount: c_float);

        #[method(boostShadowAmount)]
        pub unsafe fn boostShadowAmount(&self) -> c_float;

        #[method(setBoostShadowAmount:)]
        pub unsafe fn setBoostShadowAmount(&self, boost_shadow_amount: c_float);

        #[method(isGamutMappingEnabled)]
        pub unsafe fn isGamutMappingEnabled(&self) -> bool;

        #[method(setGamutMappingEnabled:)]
        pub unsafe fn setGamutMappingEnabled(&self, gamut_mapping_enabled: bool);

        #[method(isLensCorrectionSupported)]
        pub unsafe fn isLensCorrectionSupported(&self) -> bool;

        #[method(isLensCorrectionEnabled)]
        pub unsafe fn isLensCorrectionEnabled(&self) -> bool;

        #[method(setLensCorrectionEnabled:)]
        pub unsafe fn setLensCorrectionEnabled(&self, lens_correction_enabled: bool);

        #[method(isLuminanceNoiseReductionSupported)]
        pub unsafe fn isLuminanceNoiseReductionSupported(&self) -> bool;

        #[method(luminanceNoiseReductionAmount)]
        pub unsafe fn luminanceNoiseReductionAmount(&self) -> c_float;

        #[method(setLuminanceNoiseReductionAmount:)]
        pub unsafe fn setLuminanceNoiseReductionAmount(
            &self,
            luminance_noise_reduction_amount: c_float,
        );

        #[method(isColorNoiseReductionSupported)]
        pub unsafe fn isColorNoiseReductionSupported(&self) -> bool;

        #[method(colorNoiseReductionAmount)]
        pub unsafe fn colorNoiseReductionAmount(&self) -> c_float;

        #[method(setColorNoiseReductionAmount:)]
        pub unsafe fn setColorNoiseReductionAmount(&self, color_noise_reduction_amount: c_float);

        #[method(isSharpnessSupported)]
        pub unsafe fn isSharpnessSupported(&self) -> bool;

        #[method(sharpnessAmount)]
        pub unsafe fn sharpnessAmount(&self) -> c_float;

        #[method(setSharpnessAmount:)]
        pub unsafe fn setSharpnessAmount(&self, sharpness_amount: c_float);

        #[method(isContrastSupported)]
        pub unsafe fn isContrastSupported(&self) -> bool;

        #[method(contrastAmount)]
        pub unsafe fn contrastAmount(&self) -> c_float;

        #[method(setContrastAmount:)]
        pub unsafe fn setContrastAmount(&self, contrast_amount: c_float);

        #[method(isDetailSupported)]
        pub unsafe fn isDetailSupported(&self) -> bool;

        #[method(detailAmount)]
        pub unsafe fn detailAmount(&self) -> c_float;

        #[method(setDetailAmount:)]
        pub unsafe fn setDetailAmount(&self, detail_amount: c_float);

        #[method(isMoireReductionSupported)]
        pub unsafe fn isMoireReductionSupported(&self) -> bool;

        #[method(moireReductionAmount)]
        pub unsafe fn moireReductionAmount(&self) -> c_float;

        #[method(setMoireReductionAmount:)]
        pub unsafe fn setMoireReductionAmount(&self, moire_reduction_amount: c_float);

        #[method(isLocalToneMapSupported)]
        pub unsafe fn isLocalToneMapSupported(&self) -> bool;

        #[method(localToneMapAmount)]
        pub unsafe fn localToneMapAmount(&self) -> c_float;

        #[method(setLocalToneMapAmount:)]
        pub unsafe fn setLocalToneMapAmount(&self, local_tone_map_amount: c_float);

        #[method(extendedDynamicRangeAmount)]
        pub unsafe fn extendedDynamicRangeAmount(&self) -> c_float;

        #[method(setExtendedDynamicRangeAmount:)]
        pub unsafe fn setExtendedDynamicRangeAmount(&self, extended_dynamic_range_amount: c_float);

        #[method(neutralChromaticity)]
        pub unsafe fn neutralChromaticity(&self) -> CGPoint;

        #[method(setNeutralChromaticity:)]
        pub unsafe fn setNeutralChromaticity(&self, neutral_chromaticity: CGPoint);

        #[method(neutralLocation)]
        pub unsafe fn neutralLocation(&self) -> CGPoint;

        #[method(setNeutralLocation:)]
        pub unsafe fn setNeutralLocation(&self, neutral_location: CGPoint);

        #[method(neutralTemperature)]
        pub unsafe fn neutralTemperature(&self) -> c_float;

        #[method(setNeutralTemperature:)]
        pub unsafe fn setNeutralTemperature(&self, neutral_temperature: c_float);

        #[method(neutralTint)]
        pub unsafe fn neutralTint(&self) -> c_float;

        #[method(setNeutralTint:)]
        pub unsafe fn setNeutralTint(&self, neutral_tint: c_float);

        #[method_id(@__retain_semantics Other linearSpaceFilter)]
        pub unsafe fn linearSpaceFilter(&self) -> Option<Retained<CIFilter>>;

        #[method(setLinearSpaceFilter:)]
        pub unsafe fn setLinearSpaceFilter(&self, linear_space_filter: Option<&CIFilter>);

        #[cfg(feature = "CIImage")]
        #[method_id(@__retain_semantics Other previewImage)]
        pub unsafe fn previewImage(&self) -> Option<Retained<CIImage>>;

        #[cfg(feature = "CIImage")]
        #[method_id(@__retain_semantics Other portraitEffectsMatte)]
        pub unsafe fn portraitEffectsMatte(&self) -> Option<Retained<CIImage>>;

        #[cfg(feature = "CIImage")]
        #[method_id(@__retain_semantics Other semanticSegmentationSkinMatte)]
        pub unsafe fn semanticSegmentationSkinMatte(&self) -> Option<Retained<CIImage>>;

        #[cfg(feature = "CIImage")]
        #[method_id(@__retain_semantics Other semanticSegmentationHairMatte)]
        pub unsafe fn semanticSegmentationHairMatte(&self) -> Option<Retained<CIImage>>;

        #[cfg(feature = "CIImage")]
        #[method_id(@__retain_semantics Other semanticSegmentationGlassesMatte)]
        pub unsafe fn semanticSegmentationGlassesMatte(&self) -> Option<Retained<CIImage>>;

        #[cfg(feature = "CIImage")]
        #[method_id(@__retain_semantics Other semanticSegmentationSkyMatte)]
        pub unsafe fn semanticSegmentationSkyMatte(&self) -> Option<Retained<CIImage>>;

        #[cfg(feature = "CIImage")]
        #[method_id(@__retain_semantics Other semanticSegmentationTeethMatte)]
        pub unsafe fn semanticSegmentationTeethMatte(&self) -> Option<Retained<CIImage>>;

        #[method_id(@__retain_semantics Other filterWithImageURL:)]
        pub unsafe fn filterWithImageURL(url: &NSURL) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other filterWithImageData:identifierHint:)]
        pub unsafe fn filterWithImageData_identifierHint(
            data: &NSData,
            identifier_hint: Option<&NSString>,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CIFilter")]
    unsafe impl CIRAWFilter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

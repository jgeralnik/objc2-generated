// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]

#[link(name = "CoreImage", kind = "framework")]
extern "C" {}

#[cfg(feature = "CIBarcodeDescriptor")]
#[path = "CIBarcodeDescriptor.rs"]
mod __CIBarcodeDescriptor;
#[cfg(feature = "CIColor")]
#[path = "CIColor.rs"]
mod __CIColor;
#[cfg(feature = "CIContext")]
#[path = "CIContext.rs"]
mod __CIContext;
#[cfg(feature = "CIDetector")]
#[path = "CIDetector.rs"]
mod __CIDetector;
#[cfg(feature = "CIFeature")]
#[path = "CIFeature.rs"]
mod __CIFeature;
#[cfg(feature = "CIFilter")]
#[path = "CIFilter.rs"]
mod __CIFilter;
#[cfg(feature = "CIFilterConstructor")]
#[path = "CIFilterConstructor.rs"]
mod __CIFilterConstructor;
#[cfg(feature = "CIFilterGenerator")]
#[path = "CIFilterGenerator.rs"]
mod __CIFilterGenerator;
#[cfg(feature = "CIFilterShape")]
#[path = "CIFilterShape.rs"]
mod __CIFilterShape;
#[cfg(feature = "CIImage")]
#[path = "CIImage.rs"]
mod __CIImage;
#[cfg(feature = "CIImageAccumulator")]
#[path = "CIImageAccumulator.rs"]
mod __CIImageAccumulator;
#[cfg(feature = "CIImageProcessor")]
#[path = "CIImageProcessor.rs"]
mod __CIImageProcessor;
#[cfg(feature = "CIImageProvider")]
#[path = "CIImageProvider.rs"]
mod __CIImageProvider;
#[cfg(feature = "CIKernel")]
#[path = "CIKernel.rs"]
mod __CIKernel;
#[cfg(feature = "CIKernelMetalLib")]
#[path = "CIKernelMetalLib.rs"]
mod __CIKernelMetalLib;
#[cfg(feature = "CIPlugIn")]
#[path = "CIPlugIn.rs"]
mod __CIPlugIn;
#[cfg(feature = "CIPlugInInterface")]
#[path = "CIPlugInInterface.rs"]
mod __CIPlugInInterface;
#[cfg(feature = "CIRAWFilter")]
#[path = "CIRAWFilter.rs"]
mod __CIRAWFilter;
#[cfg(feature = "CIRAWFilter_Deprecated")]
#[path = "CIRAWFilter_Deprecated.rs"]
mod __CIRAWFilter_Deprecated;
#[cfg(feature = "CIRenderDestination")]
#[path = "CIRenderDestination.rs"]
mod __CIRenderDestination;
#[cfg(feature = "CISampler")]
#[path = "CISampler.rs"]
mod __CISampler;
#[cfg(feature = "CIVector")]
#[path = "CIVector.rs"]
mod __CIVector;
#[cfg(feature = "CoreImageDefines")]
#[path = "CoreImageDefines.rs"]
mod __CoreImageDefines;

#[cfg(feature = "CIBarcodeDescriptor")]
pub use self::__CIBarcodeDescriptor::CIAztecCodeDescriptor;
#[cfg(feature = "CIBarcodeDescriptor")]
pub use self::__CIBarcodeDescriptor::CIBarcodeDescriptor;
#[cfg(feature = "CIBarcodeDescriptor")]
pub use self::__CIBarcodeDescriptor::CIDataMatrixCodeDescriptor;
#[cfg(feature = "CIBarcodeDescriptor")]
pub use self::__CIBarcodeDescriptor::CIDataMatrixCodeECCVersion;
#[cfg(feature = "CIBarcodeDescriptor")]
pub use self::__CIBarcodeDescriptor::CIPDF417CodeDescriptor;
#[cfg(feature = "CIBarcodeDescriptor")]
pub use self::__CIBarcodeDescriptor::CIQRCodeDescriptor;
#[cfg(feature = "CIBarcodeDescriptor")]
pub use self::__CIBarcodeDescriptor::CIQRCodeErrorCorrectionLevel;
#[cfg(feature = "CIBarcodeDescriptor")]
pub use self::__CIBarcodeDescriptor::NSUserActivityCIBarcodeDescriptor;
#[cfg(feature = "CIColor")]
pub use self::__CIColor::CIColor;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::kCIContextAllowLowPower;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::kCIContextCacheIntermediates;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::kCIContextHighQualityDownsample;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::kCIContextMemoryLimit;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::kCIContextName;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::kCIContextOutputColorSpace;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::kCIContextOutputPremultiplied;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::kCIContextPriorityRequestLow;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::kCIContextUseSoftwareRenderer;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::kCIContextWorkingColorSpace;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::kCIContextWorkingFormat;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::kCIImageRepresentationAVDepthData;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::kCIImageRepresentationAVPortraitEffectsMatte;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::kCIImageRepresentationAVSemanticSegmentationMattes;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::kCIImageRepresentationDepthImage;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::kCIImageRepresentationDisparityImage;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::kCIImageRepresentationHDRGainMapImage;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::kCIImageRepresentationHDRImage;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::kCIImageRepresentationPortraitEffectsMatteImage;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::kCIImageRepresentationSemanticSegmentationGlassesMatteImage;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::kCIImageRepresentationSemanticSegmentationHairMatteImage;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::kCIImageRepresentationSemanticSegmentationSkinMatteImage;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::kCIImageRepresentationSemanticSegmentationSkyMatteImage;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::kCIImageRepresentationSemanticSegmentationTeethMatteImage;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::CIContext;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::CIContextOption;
#[cfg(feature = "CIContext")]
pub use self::__CIContext::CIImageRepresentationOption;
#[cfg(feature = "CIDetector")]
pub use self::__CIDetector::CIDetector;
#[cfg(feature = "CIDetector")]
pub use self::__CIDetector::CIDetectorAccuracy;
#[cfg(feature = "CIDetector")]
pub use self::__CIDetector::CIDetectorAccuracyHigh;
#[cfg(feature = "CIDetector")]
pub use self::__CIDetector::CIDetectorAccuracyLow;
#[cfg(feature = "CIDetector")]
pub use self::__CIDetector::CIDetectorAspectRatio;
#[cfg(feature = "CIDetector")]
pub use self::__CIDetector::CIDetectorEyeBlink;
#[cfg(feature = "CIDetector")]
pub use self::__CIDetector::CIDetectorFocalLength;
#[cfg(feature = "CIDetector")]
pub use self::__CIDetector::CIDetectorImageOrientation;
#[cfg(feature = "CIDetector")]
pub use self::__CIDetector::CIDetectorMaxFeatureCount;
#[cfg(feature = "CIDetector")]
pub use self::__CIDetector::CIDetectorMinFeatureSize;
#[cfg(feature = "CIDetector")]
pub use self::__CIDetector::CIDetectorNumberOfAngles;
#[cfg(feature = "CIDetector")]
pub use self::__CIDetector::CIDetectorReturnSubFeatures;
#[cfg(feature = "CIDetector")]
pub use self::__CIDetector::CIDetectorSmile;
#[cfg(feature = "CIDetector")]
pub use self::__CIDetector::CIDetectorTracking;
#[cfg(feature = "CIDetector")]
pub use self::__CIDetector::CIDetectorTypeFace;
#[cfg(feature = "CIDetector")]
pub use self::__CIDetector::CIDetectorTypeQRCode;
#[cfg(feature = "CIDetector")]
pub use self::__CIDetector::CIDetectorTypeRectangle;
#[cfg(feature = "CIDetector")]
pub use self::__CIDetector::CIDetectorTypeText;
#[cfg(feature = "CIFeature")]
pub use self::__CIFeature::CIFaceFeature;
#[cfg(feature = "CIFeature")]
pub use self::__CIFeature::CIFeature;
#[cfg(feature = "CIFeature")]
pub use self::__CIFeature::CIFeatureTypeFace;
#[cfg(feature = "CIFeature")]
pub use self::__CIFeature::CIFeatureTypeQRCode;
#[cfg(feature = "CIFeature")]
pub use self::__CIFeature::CIFeatureTypeRectangle;
#[cfg(feature = "CIFeature")]
pub use self::__CIFeature::CIFeatureTypeText;
#[cfg(feature = "CIFeature")]
pub use self::__CIFeature::CIQRCodeFeature;
#[cfg(feature = "CIFeature")]
pub use self::__CIFeature::CIRectangleFeature;
#[cfg(feature = "CIFeature")]
pub use self::__CIFeature::CITextFeature;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIApplyOptionColorSpace;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIApplyOptionDefinition;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIApplyOptionExtent;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIApplyOptionUserInfo;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeClass;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeDefault;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeDescription;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeDisplayName;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeFilterAvailable_Mac;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeFilterAvailable_iOS;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeFilterCategories;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeFilterDisplayName;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeFilterName;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeIdentity;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeMax;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeMin;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeName;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeReferenceDocumentation;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeSliderMax;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeSliderMin;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeType;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeTypeAngle;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeTypeBoolean;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeTypeColor;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeTypeCount;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeTypeDistance;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeTypeGradient;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeTypeImage;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeTypeInteger;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeTypeOffset;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeTypeOpaqueColor;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeTypePosition;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeTypePosition3;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeTypeRectangle;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeTypeScalar;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeTypeTime;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIAttributeTypeTransform;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCICategoryBlur;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCICategoryBuiltIn;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCICategoryColorAdjustment;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCICategoryColorEffect;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCICategoryCompositeOperation;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCICategoryDistortionEffect;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCICategoryFilterGenerator;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCICategoryGenerator;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCICategoryGeometryAdjustment;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCICategoryGradient;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCICategoryHalftoneEffect;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCICategoryHighDynamicRange;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCICategoryInterlaced;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCICategoryNonSquarePixels;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCICategoryReduction;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCICategorySharpen;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCICategoryStillImage;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCICategoryStylize;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCICategoryTileEffect;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCICategoryTransition;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCICategoryVideo;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputAmountKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputAngleKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputAspectRatioKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputBackgroundImageKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputBiasKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputBrightnessKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputCenterKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputColorKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputContrastKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputDepthImageKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputDisparityImageKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputEVKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputExtentKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputGradientImageKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputImageKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputIntensityKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputMaskImageKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputMatteImageKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputRadiusKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputRefractionKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputSaturationKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputScaleKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputShadingImageKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputSharpnessKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputTargetImageKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputTimeKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputTransformKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputVersionKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputWeightsKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIInputWidthKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIOutputImageKey;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIUIParameterSet;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIUISetAdvanced;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIUISetBasic;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIUISetDevelopment;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::kCIUISetIntermediate;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::CIFilter;
#[cfg(feature = "CIFilter")]
pub use self::__CIFilter::CIFilterProtocol;
#[cfg(feature = "CIFilterConstructor")]
pub use self::__CIFilterConstructor::CIFilterConstructor;
#[cfg(feature = "CIFilterGenerator")]
pub use self::__CIFilterGenerator::kCIFilterGeneratorExportedKey;
#[cfg(feature = "CIFilterGenerator")]
pub use self::__CIFilterGenerator::kCIFilterGeneratorExportedKeyName;
#[cfg(feature = "CIFilterGenerator")]
pub use self::__CIFilterGenerator::kCIFilterGeneratorExportedKeyTargetObject;
#[cfg(feature = "CIFilterGenerator")]
pub use self::__CIFilterGenerator::CIFilterGenerator;
#[cfg(feature = "CIFilterShape")]
pub use self::__CIFilterShape::CIFilterShape;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatA16;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatA8;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatABGR8;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatARGB8;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatAf;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatAh;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatBGRA8;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatL16;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatL8;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatLA16;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatLA8;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatLAf;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatLAh;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatLf;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatLh;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatR16;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatR8;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatRG16;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatRG8;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatRGB10;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatRGBA16;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatRGBA8;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatRGBAf;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatRGBAh;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatRGBX16;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatRGBXf;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatRGBXh;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatRGf;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatRGh;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatRf;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIFormatRh;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIImageApplyOrientationProperty;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIImageAutoAdjustCrop;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIImageAutoAdjustEnhance;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIImageAutoAdjustFeatures;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIImageAutoAdjustLevel;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIImageAutoAdjustRedEye;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIImageAuxiliaryDepth;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIImageAuxiliaryDisparity;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIImageAuxiliaryHDRGainMap;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIImageAuxiliaryPortraitEffectsMatte;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIImageAuxiliarySemanticSegmentationGlassesMatte;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIImageAuxiliarySemanticSegmentationHairMatte;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIImageAuxiliarySemanticSegmentationSkinMatte;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIImageAuxiliarySemanticSegmentationSkyMatte;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIImageAuxiliarySemanticSegmentationTeethMatte;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIImageCacheImmediately;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIImageColorSpace;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIImageContentHeadroom;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIImageExpandToHDR;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIImageNearestSampling;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIImageProperties;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIImageTextureFormat;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIImageTextureTarget;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::kCIImageToneMapHDRtoSDR;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::CIFormat;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::CIImage;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::CIImageAutoAdjustmentOption;
#[cfg(feature = "CIImage")]
pub use self::__CIImage::CIImageOption;
#[cfg(feature = "CIImageAccumulator")]
pub use self::__CIImageAccumulator::CIImageAccumulator;
#[cfg(feature = "CIImageProcessor")]
pub use self::__CIImageProcessor::CIImageProcessorInput;
#[cfg(feature = "CIImageProcessor")]
pub use self::__CIImageProcessor::CIImageProcessorKernel;
#[cfg(feature = "CIImageProcessor")]
pub use self::__CIImageProcessor::CIImageProcessorOutput;
#[cfg(all(feature = "CIImage", feature = "CIImageProvider"))]
pub use self::__CIImageProvider::kCIImageProviderTileSize;
#[cfg(all(feature = "CIImage", feature = "CIImageProvider"))]
pub use self::__CIImageProvider::kCIImageProviderUserInfo;
#[cfg(feature = "CIImageProvider")]
pub use self::__CIImageProvider::NSObjectCIImageProvider;
#[cfg(feature = "CIKernel")]
pub use self::__CIKernel::CIBlendKernel;
#[cfg(feature = "CIKernel")]
pub use self::__CIKernel::CIColorKernel;
#[cfg(feature = "CIKernel")]
pub use self::__CIKernel::CIKernel;
#[cfg(all(feature = "CIKernel", feature = "block2"))]
pub use self::__CIKernel::CIKernelROICallback;
#[cfg(feature = "CIKernel")]
pub use self::__CIKernel::CIWarpKernel;
#[cfg(feature = "CIPlugIn")]
pub use self::__CIPlugIn::CIPlugIn;
#[cfg(feature = "CIPlugInInterface")]
pub use self::__CIPlugInInterface::CIPlugInRegistration;
#[cfg(feature = "CIRAWFilter")]
pub use self::__CIRAWFilter::CIRAWDecoderVersion;
#[cfg(feature = "CIRAWFilter")]
pub use self::__CIRAWFilter::CIRAWDecoderVersion6;
#[cfg(feature = "CIRAWFilter")]
pub use self::__CIRAWFilter::CIRAWDecoderVersion6DNG;
#[cfg(feature = "CIRAWFilter")]
pub use self::__CIRAWFilter::CIRAWDecoderVersion7;
#[cfg(feature = "CIRAWFilter")]
pub use self::__CIRAWFilter::CIRAWDecoderVersion7DNG;
#[cfg(feature = "CIRAWFilter")]
pub use self::__CIRAWFilter::CIRAWDecoderVersion8;
#[cfg(feature = "CIRAWFilter")]
pub use self::__CIRAWFilter::CIRAWDecoderVersion8DNG;
#[cfg(feature = "CIRAWFilter")]
pub use self::__CIRAWFilter::CIRAWDecoderVersionNone;
#[cfg(all(feature = "CIFilter", feature = "CIRAWFilter"))]
pub use self::__CIRAWFilter::CIRAWFilter;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIActiveKeys;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputAllowDraftModeKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputBaselineExposureKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputBoostKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputBoostShadowAmountKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputColorNoiseReductionAmountKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputDecoderVersionKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputDisableGamutMapKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputEnableChromaticNoiseTrackingKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputEnableEDRModeKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputEnableSharpeningKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputEnableVendorLensCorrectionKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputIgnoreImageOrientationKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputImageOrientationKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputLinearSpaceFilter;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputLocalToneMapAmountKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputLuminanceNoiseReductionAmountKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputMoireAmountKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputNeutralChromaticityXKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputNeutralChromaticityYKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputNeutralLocationKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputNeutralTemperatureKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputNeutralTintKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputNoiseReductionAmountKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputNoiseReductionContrastAmountKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputNoiseReductionDetailAmountKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputNoiseReductionSharpnessAmountKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIInputScaleFactorKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIOutputNativeSizeKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCIPropertiesKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::kCISupportedDecoderVersionsKey;
#[cfg(feature = "CIRAWFilter_Deprecated")]
pub use self::__CIRAWFilter_Deprecated::CIRAWFilterOption;
#[cfg(feature = "CIRenderDestination")]
pub use self::__CIRenderDestination::CIRenderDestination;
#[cfg(feature = "CIRenderDestination")]
pub use self::__CIRenderDestination::CIRenderDestinationAlphaMode;
#[cfg(feature = "CIRenderDestination")]
pub use self::__CIRenderDestination::CIRenderInfo;
#[cfg(feature = "CIRenderDestination")]
pub use self::__CIRenderDestination::CIRenderTask;
#[cfg(feature = "CISampler")]
pub use self::__CISampler::kCISamplerAffineMatrix;
#[cfg(feature = "CISampler")]
pub use self::__CISampler::kCISamplerColorSpace;
#[cfg(feature = "CISampler")]
pub use self::__CISampler::kCISamplerFilterLinear;
#[cfg(feature = "CISampler")]
pub use self::__CISampler::kCISamplerFilterMode;
#[cfg(feature = "CISampler")]
pub use self::__CISampler::kCISamplerFilterNearest;
#[cfg(feature = "CISampler")]
pub use self::__CISampler::kCISamplerWrapBlack;
#[cfg(feature = "CISampler")]
pub use self::__CISampler::kCISamplerWrapClamp;
#[cfg(feature = "CISampler")]
pub use self::__CISampler::kCISamplerWrapMode;
#[cfg(feature = "CISampler")]
pub use self::__CISampler::CISampler;
#[cfg(feature = "CIVector")]
pub use self::__CIVector::CIVector;

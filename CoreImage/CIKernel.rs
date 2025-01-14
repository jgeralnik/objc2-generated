//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coreimage/cikernelroicallback?language=objc)
#[cfg(feature = "block2")]
pub type CIKernelROICallback = *mut block2::Block<dyn Fn(c_int, CGRect) -> CGRect>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/cikernel?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIKernel;
);

unsafe impl NSObjectProtocol for CIKernel {}

extern_methods!(
    unsafe impl CIKernel {
        #[deprecated = "Core Image Kernel Language API deprecated. (Define CI_SILENCE_GL_DEPRECATION to silence these warnings)"]
        #[method_id(@__retain_semantics Other kernelsWithString:)]
        pub unsafe fn kernelsWithString(string: &NSString) -> Option<Retained<NSArray<CIKernel>>>;

        #[method_id(@__retain_semantics Other kernelsWithMetalString:error:_)]
        pub unsafe fn kernelsWithMetalString_error(
            source: &NSString,
        ) -> Result<Retained<NSArray<CIKernel>>, Retained<NSError>>;

        #[deprecated = "Core Image Kernel Language API deprecated. (Define CI_SILENCE_GL_DEPRECATION to silence these warnings)"]
        #[method_id(@__retain_semantics Other kernelWithString:)]
        pub unsafe fn kernelWithString(string: &NSString) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other kernelWithFunctionName:fromMetalLibraryData:error:_)]
        pub unsafe fn kernelWithFunctionName_fromMetalLibraryData_error(
            name: &NSString,
            data: &NSData,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "CIImage")]
        #[method_id(@__retain_semantics Other kernelWithFunctionName:fromMetalLibraryData:outputPixelFormat:error:_)]
        pub unsafe fn kernelWithFunctionName_fromMetalLibraryData_outputPixelFormat_error(
            name: &NSString,
            data: &NSData,
            format: CIFormat,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method_id(@__retain_semantics Other kernelNamesFromMetalLibraryData:)]
        pub unsafe fn kernelNamesFromMetalLibraryData(data: &NSData)
            -> Retained<NSArray<NSString>>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[method(setROISelector:)]
        pub unsafe fn setROISelector(&self, method: Sel);

        #[cfg(all(feature = "CIImage", feature = "block2"))]
        #[method_id(@__retain_semantics Other applyWithExtent:roiCallback:arguments:)]
        pub unsafe fn applyWithExtent_roiCallback_arguments(
            &self,
            extent: CGRect,
            callback: CIKernelROICallback,
            args: Option<&NSArray<AnyObject>>,
        ) -> Option<Retained<CIImage>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIKernel {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/cicolorkernel?language=objc)
    #[unsafe(super(CIKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIColorKernel;
);

unsafe impl NSObjectProtocol for CIColorKernel {}

extern_methods!(
    unsafe impl CIColorKernel {
        #[deprecated = "Core Image Kernel Language API deprecated. (Define CI_SILENCE_GL_DEPRECATION to silence these warnings)"]
        #[method_id(@__retain_semantics Other kernelWithString:)]
        pub unsafe fn kernelWithString(string: &NSString) -> Option<Retained<Self>>;

        #[cfg(feature = "CIImage")]
        #[method_id(@__retain_semantics Other applyWithExtent:arguments:)]
        pub unsafe fn applyWithExtent_arguments(
            &self,
            extent: CGRect,
            args: Option<&NSArray<AnyObject>>,
        ) -> Option<Retained<CIImage>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CIKernel`
    unsafe impl CIColorKernel {
        #[method_id(@__retain_semantics Other kernelWithFunctionName:fromMetalLibraryData:error:_)]
        pub unsafe fn kernelWithFunctionName_fromMetalLibraryData_error(
            name: &NSString,
            data: &NSData,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "CIImage")]
        #[method_id(@__retain_semantics Other kernelWithFunctionName:fromMetalLibraryData:outputPixelFormat:error:_)]
        pub unsafe fn kernelWithFunctionName_fromMetalLibraryData_outputPixelFormat_error(
            name: &NSString,
            data: &NSData,
            format: CIFormat,
        ) -> Result<Retained<Self>, Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIColorKernel {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/ciwarpkernel?language=objc)
    #[unsafe(super(CIKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIWarpKernel;
);

unsafe impl NSObjectProtocol for CIWarpKernel {}

extern_methods!(
    unsafe impl CIWarpKernel {
        #[deprecated = "Core Image Kernel Language API deprecated. (Define CI_SILENCE_GL_DEPRECATION to silence these warnings)"]
        #[method_id(@__retain_semantics Other kernelWithString:)]
        pub unsafe fn kernelWithString(string: &NSString) -> Option<Retained<Self>>;

        #[cfg(all(feature = "CIImage", feature = "block2"))]
        #[method_id(@__retain_semantics Other applyWithExtent:roiCallback:inputImage:arguments:)]
        pub unsafe fn applyWithExtent_roiCallback_inputImage_arguments(
            &self,
            extent: CGRect,
            callback: CIKernelROICallback,
            image: &CIImage,
            args: Option<&NSArray<AnyObject>>,
        ) -> Option<Retained<CIImage>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CIKernel`
    unsafe impl CIWarpKernel {
        #[method_id(@__retain_semantics Other kernelWithFunctionName:fromMetalLibraryData:error:_)]
        pub unsafe fn kernelWithFunctionName_fromMetalLibraryData_error(
            name: &NSString,
            data: &NSData,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "CIImage")]
        #[method_id(@__retain_semantics Other kernelWithFunctionName:fromMetalLibraryData:outputPixelFormat:error:_)]
        pub unsafe fn kernelWithFunctionName_fromMetalLibraryData_outputPixelFormat_error(
            name: &NSString,
            data: &NSData,
            format: CIFormat,
        ) -> Result<Retained<Self>, Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIWarpKernel {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/ciblendkernel?language=objc)
    #[unsafe(super(CIColorKernel, CIKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIBlendKernel;
);

unsafe impl NSObjectProtocol for CIBlendKernel {}

extern_methods!(
    unsafe impl CIBlendKernel {
        #[deprecated = "Core Image Kernel Language API deprecated. (Define CI_SILENCE_GL_DEPRECATION to silence these warnings)"]
        #[method_id(@__retain_semantics Other kernelWithString:)]
        pub unsafe fn kernelWithString(string: &NSString) -> Option<Retained<Self>>;

        #[cfg(feature = "CIImage")]
        #[method_id(@__retain_semantics Other applyWithForeground:background:)]
        pub unsafe fn applyWithForeground_background(
            &self,
            foreground: &CIImage,
            background: &CIImage,
        ) -> Option<Retained<CIImage>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CIKernel`
    unsafe impl CIBlendKernel {
        #[method_id(@__retain_semantics Other kernelWithFunctionName:fromMetalLibraryData:error:_)]
        pub unsafe fn kernelWithFunctionName_fromMetalLibraryData_error(
            name: &NSString,
            data: &NSData,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "CIImage")]
        #[method_id(@__retain_semantics Other kernelWithFunctionName:fromMetalLibraryData:outputPixelFormat:error:_)]
        pub unsafe fn kernelWithFunctionName_fromMetalLibraryData_outputPixelFormat_error(
            name: &NSString,
            data: &NSData,
            format: CIFormat,
        ) -> Result<Retained<Self>, Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIBlendKernel {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// BuiltIn
    unsafe impl CIBlendKernel {
        #[method_id(@__retain_semantics Other componentAdd)]
        pub unsafe fn componentAdd() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other componentMultiply)]
        pub unsafe fn componentMultiply() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other componentMin)]
        pub unsafe fn componentMin() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other componentMax)]
        pub unsafe fn componentMax() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other clear)]
        pub unsafe fn clear() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other source)]
        pub unsafe fn source() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other destination)]
        pub unsafe fn destination() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other sourceOver)]
        pub unsafe fn sourceOver() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other destinationOver)]
        pub unsafe fn destinationOver() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other sourceIn)]
        pub unsafe fn sourceIn() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other destinationIn)]
        pub unsafe fn destinationIn() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other sourceOut)]
        pub unsafe fn sourceOut() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other destinationOut)]
        pub unsafe fn destinationOut() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other sourceAtop)]
        pub unsafe fn sourceAtop() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other destinationAtop)]
        pub unsafe fn destinationAtop() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other exclusiveOr)]
        pub unsafe fn exclusiveOr() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other multiply)]
        pub unsafe fn multiply() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other screen)]
        pub unsafe fn screen() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other overlay)]
        pub unsafe fn overlay() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other darken)]
        pub unsafe fn darken() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other lighten)]
        pub unsafe fn lighten() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other colorDodge)]
        pub unsafe fn colorDodge() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other colorBurn)]
        pub unsafe fn colorBurn() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other hardLight)]
        pub unsafe fn hardLight() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other softLight)]
        pub unsafe fn softLight() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other difference)]
        pub unsafe fn difference() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other exclusion)]
        pub unsafe fn exclusion() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other hue)]
        pub unsafe fn hue() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other saturation)]
        pub unsafe fn saturation() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other color)]
        pub unsafe fn color() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other luminosity)]
        pub unsafe fn luminosity() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other subtract)]
        pub unsafe fn subtract() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other divide)]
        pub unsafe fn divide() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other linearBurn)]
        pub unsafe fn linearBurn() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other linearDodge)]
        pub unsafe fn linearDodge() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other vividLight)]
        pub unsafe fn vividLight() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other linearLight)]
        pub unsafe fn linearLight() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other pinLight)]
        pub unsafe fn pinLight() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other hardMix)]
        pub unsafe fn hardMix() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other darkerColor)]
        pub unsafe fn darkerColor() -> Retained<CIBlendKernel>;

        #[method_id(@__retain_semantics Other lighterColor)]
        pub unsafe fn lighterColor() -> Retained<CIBlendKernel>;
    }
);

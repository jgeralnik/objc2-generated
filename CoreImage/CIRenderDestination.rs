//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-metal")]
use objc2_metal::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coreimage/cirenderdestinationalphamode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CIRenderDestinationAlphaMode(pub NSUInteger);
impl CIRenderDestinationAlphaMode {
    pub const CIRenderDestinationAlphaNone: Self = Self(0);
    pub const CIRenderDestinationAlphaPremultiplied: Self = Self(1);
    pub const CIRenderDestinationAlphaUnpremultiplied: Self = Self(2);
}

unsafe impl Encode for CIRenderDestinationAlphaMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for CIRenderDestinationAlphaMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/cirenderdestination?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIRenderDestination;
);

unsafe impl NSObjectProtocol for CIRenderDestination {}

extern_methods!(
    unsafe impl CIRenderDestination {
        #[cfg(feature = "objc2-metal")]
        #[method_id(@__retain_semantics Init initWithMTLTexture:commandBuffer:)]
        pub unsafe fn initWithMTLTexture_commandBuffer(
            this: Allocated<Self>,
            texture: &ProtocolObject<dyn MTLTexture>,
            command_buffer: Option<&ProtocolObject<dyn MTLCommandBuffer>>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "block2", feature = "objc2-metal"))]
        #[method_id(@__retain_semantics Init initWithWidth:height:pixelFormat:commandBuffer:mtlTextureProvider:)]
        pub unsafe fn initWithWidth_height_pixelFormat_commandBuffer_mtlTextureProvider(
            this: Allocated<Self>,
            width: NSUInteger,
            height: NSUInteger,
            pixel_format: MTLPixelFormat,
            command_buffer: Option<&ProtocolObject<dyn MTLCommandBuffer>>,
            block: Option<&block2::Block<dyn Fn() -> NonNull<ProtocolObject<dyn MTLTexture>>>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithGLTexture:target:width:height:)]
        pub unsafe fn initWithGLTexture_target_width_height(
            this: Allocated<Self>,
            texture: c_uint,
            target: c_uint,
            width: NSUInteger,
            height: NSUInteger,
        ) -> Retained<Self>;

        #[cfg(feature = "CIImage")]
        #[method_id(@__retain_semantics Init initWithBitmapData:width:height:bytesPerRow:format:)]
        pub unsafe fn initWithBitmapData_width_height_bytesPerRow_format(
            this: Allocated<Self>,
            data: NonNull<c_void>,
            width: NSUInteger,
            height: NSUInteger,
            bytes_per_row: NSUInteger,
            format: CIFormat,
        ) -> Retained<Self>;

        #[method(width)]
        pub unsafe fn width(&self) -> NSUInteger;

        #[method(height)]
        pub unsafe fn height(&self) -> NSUInteger;

        #[method(alphaMode)]
        pub unsafe fn alphaMode(&self) -> CIRenderDestinationAlphaMode;

        #[method(setAlphaMode:)]
        pub unsafe fn setAlphaMode(&self, alpha_mode: CIRenderDestinationAlphaMode);

        #[method(isFlipped)]
        pub unsafe fn isFlipped(&self) -> bool;

        #[method(setFlipped:)]
        pub unsafe fn setFlipped(&self, flipped: bool);

        #[method(isDithered)]
        pub unsafe fn isDithered(&self) -> bool;

        #[method(setDithered:)]
        pub unsafe fn setDithered(&self, dithered: bool);

        #[method(isClamped)]
        pub unsafe fn isClamped(&self) -> bool;

        #[method(setClamped:)]
        pub unsafe fn setClamped(&self, clamped: bool);

        #[cfg(feature = "CIKernel")]
        #[method_id(@__retain_semantics Other blendKernel)]
        pub unsafe fn blendKernel(&self) -> Option<Retained<CIBlendKernel>>;

        #[cfg(feature = "CIKernel")]
        #[method(setBlendKernel:)]
        pub unsafe fn setBlendKernel(&self, blend_kernel: Option<&CIBlendKernel>);

        #[method(blendsInDestinationColorSpace)]
        pub unsafe fn blendsInDestinationColorSpace(&self) -> bool;

        #[method(setBlendsInDestinationColorSpace:)]
        pub unsafe fn setBlendsInDestinationColorSpace(
            &self,
            blends_in_destination_color_space: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIRenderDestination {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/cirenderinfo?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIRenderInfo;
);

unsafe impl NSObjectProtocol for CIRenderInfo {}

extern_methods!(
    unsafe impl CIRenderInfo {
        #[method(kernelExecutionTime)]
        pub unsafe fn kernelExecutionTime(&self) -> NSTimeInterval;

        #[method(kernelCompileTime)]
        pub unsafe fn kernelCompileTime(&self) -> NSTimeInterval;

        #[method(passCount)]
        pub unsafe fn passCount(&self) -> NSInteger;

        #[method(pixelsProcessed)]
        pub unsafe fn pixelsProcessed(&self) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIRenderInfo {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/cirendertask?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIRenderTask;
);

unsafe impl NSObjectProtocol for CIRenderTask {}

extern_methods!(
    unsafe impl CIRenderTask {
        #[method_id(@__retain_semantics Other waitUntilCompletedAndReturnError:_)]
        pub unsafe fn waitUntilCompletedAndReturnError(
            &self,
        ) -> Result<Retained<CIRenderInfo>, Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIRenderTask {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// CIRenderDestination
    #[cfg(feature = "CIContext")]
    unsafe impl CIContext {
        #[cfg(feature = "CIImage")]
        #[method_id(@__retain_semantics Other startTaskToRender:fromRect:toDestination:atPoint:error:_)]
        pub unsafe fn startTaskToRender_fromRect_toDestination_atPoint_error(
            &self,
            image: &CIImage,
            from_rect: CGRect,
            destination: &CIRenderDestination,
            at_point: CGPoint,
        ) -> Result<Retained<CIRenderTask>, Retained<NSError>>;

        #[cfg(feature = "CIImage")]
        #[method_id(@__retain_semantics Other startTaskToRender:toDestination:error:_)]
        pub unsafe fn startTaskToRender_toDestination_error(
            &self,
            image: &CIImage,
            destination: &CIRenderDestination,
        ) -> Result<Retained<CIRenderTask>, Retained<NSError>>;

        #[cfg(feature = "CIImage")]
        #[method(prepareRender:fromRect:toDestination:atPoint:error:_)]
        pub unsafe fn prepareRender_fromRect_toDestination_atPoint_error(
            &self,
            image: &CIImage,
            from_rect: CGRect,
            destination: &CIRenderDestination,
            at_point: CGPoint,
        ) -> Result<(), Retained<NSError>>;

        #[method_id(@__retain_semantics Other startTaskToClear:error:_)]
        pub unsafe fn startTaskToClear_error(
            &self,
            destination: &CIRenderDestination,
        ) -> Result<Retained<CIRenderTask>, Retained<NSError>>;
    }
);

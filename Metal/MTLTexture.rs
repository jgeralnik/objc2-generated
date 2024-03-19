//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLTextureType {
        MTLTextureType1D = 0,
        MTLTextureType1DArray = 1,
        MTLTextureType2D = 2,
        MTLTextureType2DArray = 3,
        MTLTextureType2DMultisample = 4,
        #[doc(alias = "MTLTextureTypeCube")]
        Cube = 5,
        #[doc(alias = "MTLTextureTypeCubeArray")]
        CubeArray = 6,
        MTLTextureType3D = 7,
        MTLTextureType2DMultisampleArray = 8,
        #[doc(alias = "MTLTextureTypeTextureBuffer")]
        TextureBuffer = 9,
    }
);

ns_enum!(
    #[underlying(u8)]
    pub enum MTLTextureSwizzle {
        #[doc(alias = "MTLTextureSwizzleZero")]
        Zero = 0,
        #[doc(alias = "MTLTextureSwizzleOne")]
        One = 1,
        #[doc(alias = "MTLTextureSwizzleRed")]
        Red = 2,
        #[doc(alias = "MTLTextureSwizzleGreen")]
        Green = 3,
        #[doc(alias = "MTLTextureSwizzleBlue")]
        Blue = 4,
        #[doc(alias = "MTLTextureSwizzleAlpha")]
        Alpha = 5,
    }
);

extern_struct!(
    #[encoding_name("?")]
    pub struct MTLTextureSwizzleChannels {
        pub red: MTLTextureSwizzle,
        pub green: MTLTextureSwizzle,
        pub blue: MTLTextureSwizzle,
        pub alpha: MTLTextureSwizzle,
    }
);

// TODO: pub fn MTLTextureSwizzleChannelsMake(r: MTLTextureSwizzle,g: MTLTextureSwizzle,b: MTLTextureSwizzle,a: MTLTextureSwizzle,) -> MTLTextureSwizzleChannels;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLSharedTextureHandle;

    unsafe impl ClassType for MTLSharedTextureHandle {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for MTLSharedTextureHandle {}

unsafe impl NSObjectProtocol for MTLSharedTextureHandle {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for MTLSharedTextureHandle {}

extern_methods!(
    unsafe impl MTLSharedTextureHandle {
        #[cfg(feature = "Metal_MTLDevice")]
        #[method_id(@__retain_semantics Other device)]
        pub fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub fn label(&self) -> Option<Id<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLSharedTextureHandle {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum MTLTextureUsage {
        #[doc(alias = "MTLTextureUsageUnknown")]
        Unknown = 0x0000,
        #[doc(alias = "MTLTextureUsageShaderRead")]
        ShaderRead = 0x0001,
        #[doc(alias = "MTLTextureUsageShaderWrite")]
        ShaderWrite = 0x0002,
        #[doc(alias = "MTLTextureUsageRenderTarget")]
        RenderTarget = 0x0004,
        #[doc(alias = "MTLTextureUsagePixelFormatView")]
        PixelFormatView = 0x0010,
        #[doc(alias = "MTLTextureUsageShaderAtomic")]
        ShaderAtomic = 0x0020,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MTLTextureCompressionType {
        #[doc(alias = "MTLTextureCompressionTypeLossless")]
        Lossless = 0,
        #[doc(alias = "MTLTextureCompressionTypeLossy")]
        Lossy = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLTextureDescriptor;

    unsafe impl ClassType for MTLTextureDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for MTLTextureDescriptor {}

unsafe impl NSObjectProtocol for MTLTextureDescriptor {}

extern_methods!(
    unsafe impl MTLTextureDescriptor {
        #[cfg(feature = "Metal_MTLPixelFormat")]
        #[method_id(@__retain_semantics Other texture2DDescriptorWithPixelFormat:width:height:mipmapped:)]
        pub unsafe fn texture2DDescriptorWithPixelFormat_width_height_mipmapped(
            pixel_format: MTLPixelFormat,
            width: NSUInteger,
            height: NSUInteger,
            mipmapped: bool,
        ) -> Id<MTLTextureDescriptor>;

        #[cfg(feature = "Metal_MTLPixelFormat")]
        #[method_id(@__retain_semantics Other textureCubeDescriptorWithPixelFormat:size:mipmapped:)]
        pub unsafe fn textureCubeDescriptorWithPixelFormat_size_mipmapped(
            pixel_format: MTLPixelFormat,
            size: NSUInteger,
            mipmapped: bool,
        ) -> Id<MTLTextureDescriptor>;

        #[cfg(all(feature = "Metal_MTLPixelFormat", feature = "Metal_MTLResource"))]
        #[method_id(@__retain_semantics Other textureBufferDescriptorWithPixelFormat:width:resourceOptions:usage:)]
        pub unsafe fn textureBufferDescriptorWithPixelFormat_width_resourceOptions_usage(
            pixel_format: MTLPixelFormat,
            width: NSUInteger,
            resource_options: MTLResourceOptions,
            usage: MTLTextureUsage,
        ) -> Id<MTLTextureDescriptor>;

        #[method(textureType)]
        pub fn textureType(&self) -> MTLTextureType;

        #[method(setTextureType:)]
        pub fn setTextureType(&self, texture_type: MTLTextureType);

        #[cfg(feature = "Metal_MTLPixelFormat")]
        #[method(pixelFormat)]
        pub fn pixelFormat(&self) -> MTLPixelFormat;

        #[cfg(feature = "Metal_MTLPixelFormat")]
        #[method(setPixelFormat:)]
        pub fn setPixelFormat(&self, pixel_format: MTLPixelFormat);

        #[method(width)]
        pub fn width(&self) -> NSUInteger;

        #[method(setWidth:)]
        pub unsafe fn setWidth(&self, width: NSUInteger);

        #[method(height)]
        pub fn height(&self) -> NSUInteger;

        #[method(setHeight:)]
        pub unsafe fn setHeight(&self, height: NSUInteger);

        #[method(depth)]
        pub fn depth(&self) -> NSUInteger;

        #[method(setDepth:)]
        pub unsafe fn setDepth(&self, depth: NSUInteger);

        #[method(mipmapLevelCount)]
        pub fn mipmapLevelCount(&self) -> NSUInteger;

        #[method(setMipmapLevelCount:)]
        pub unsafe fn setMipmapLevelCount(&self, mipmap_level_count: NSUInteger);

        #[method(sampleCount)]
        pub fn sampleCount(&self) -> NSUInteger;

        #[method(setSampleCount:)]
        pub unsafe fn setSampleCount(&self, sample_count: NSUInteger);

        #[method(arrayLength)]
        pub fn arrayLength(&self) -> NSUInteger;

        #[method(setArrayLength:)]
        pub unsafe fn setArrayLength(&self, array_length: NSUInteger);

        #[cfg(feature = "Metal_MTLResource")]
        #[method(resourceOptions)]
        pub fn resourceOptions(&self) -> MTLResourceOptions;

        #[cfg(feature = "Metal_MTLResource")]
        #[method(setResourceOptions:)]
        pub fn setResourceOptions(&self, resource_options: MTLResourceOptions);

        #[cfg(feature = "Metal_MTLResource")]
        #[method(cpuCacheMode)]
        pub fn cpuCacheMode(&self) -> MTLCPUCacheMode;

        #[cfg(feature = "Metal_MTLResource")]
        #[method(setCpuCacheMode:)]
        pub fn setCpuCacheMode(&self, cpu_cache_mode: MTLCPUCacheMode);

        #[cfg(feature = "Metal_MTLResource")]
        #[method(storageMode)]
        pub fn storageMode(&self) -> MTLStorageMode;

        #[cfg(feature = "Metal_MTLResource")]
        #[method(setStorageMode:)]
        pub fn setStorageMode(&self, storage_mode: MTLStorageMode);

        #[cfg(feature = "Metal_MTLResource")]
        #[method(hazardTrackingMode)]
        pub fn hazardTrackingMode(&self) -> MTLHazardTrackingMode;

        #[cfg(feature = "Metal_MTLResource")]
        #[method(setHazardTrackingMode:)]
        pub fn setHazardTrackingMode(&self, hazard_tracking_mode: MTLHazardTrackingMode);

        #[method(usage)]
        pub fn usage(&self) -> MTLTextureUsage;

        #[method(setUsage:)]
        pub fn setUsage(&self, usage: MTLTextureUsage);

        #[method(allowGPUOptimizedContents)]
        pub fn allowGPUOptimizedContents(&self) -> bool;

        #[method(setAllowGPUOptimizedContents:)]
        pub fn setAllowGPUOptimizedContents(&self, allow_gpu_optimized_contents: bool);

        #[method(compressionType)]
        pub unsafe fn compressionType(&self) -> MTLTextureCompressionType;

        #[method(setCompressionType:)]
        pub unsafe fn setCompressionType(&self, compression_type: MTLTextureCompressionType);

        #[method(swizzle)]
        pub fn swizzle(&self) -> MTLTextureSwizzleChannels;

        #[method(setSwizzle:)]
        pub fn setSwizzle(&self, swizzle: MTLTextureSwizzleChannels);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLTextureDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    #[cfg(feature = "Metal_MTLResource")]
    pub unsafe trait MTLTexture: MTLResource {
        #[deprecated = "Use parentTexture or buffer instead"]
        #[method_id(@__retain_semantics Other rootResource)]
        fn rootResource(&self) -> Option<Id<ProtocolObject<dyn MTLResource>>>;

        #[method_id(@__retain_semantics Other parentTexture)]
        fn parentTexture(&self) -> Option<Id<ProtocolObject<dyn MTLTexture>>>;

        #[method(parentRelativeLevel)]
        fn parentRelativeLevel(&self) -> NSUInteger;

        #[method(parentRelativeSlice)]
        fn parentRelativeSlice(&self) -> NSUInteger;

        #[cfg(feature = "Metal_MTLBuffer")]
        #[method_id(@__retain_semantics Other buffer)]
        fn buffer(&self) -> Option<Id<ProtocolObject<dyn MTLBuffer>>>;

        #[method(bufferOffset)]
        fn bufferOffset(&self) -> NSUInteger;

        #[method(bufferBytesPerRow)]
        fn bufferBytesPerRow(&self) -> NSUInteger;

        #[method(iosurfacePlane)]
        fn iosurfacePlane(&self) -> NSUInteger;

        #[method(textureType)]
        fn textureType(&self) -> MTLTextureType;

        #[cfg(feature = "Metal_MTLPixelFormat")]
        #[method(pixelFormat)]
        fn pixelFormat(&self) -> MTLPixelFormat;

        #[method(width)]
        fn width(&self) -> NSUInteger;

        #[method(height)]
        fn height(&self) -> NSUInteger;

        #[method(depth)]
        fn depth(&self) -> NSUInteger;

        #[method(mipmapLevelCount)]
        fn mipmapLevelCount(&self) -> NSUInteger;

        #[method(sampleCount)]
        fn sampleCount(&self) -> NSUInteger;

        #[method(arrayLength)]
        fn arrayLength(&self) -> NSUInteger;

        #[method(usage)]
        fn usage(&self) -> MTLTextureUsage;

        #[method(isShareable)]
        fn isShareable(&self) -> bool;

        #[method(isFramebufferOnly)]
        fn isFramebufferOnly(&self) -> bool;

        #[optional]
        #[method(firstMipmapInTail)]
        fn firstMipmapInTail(&self) -> NSUInteger;

        #[optional]
        #[method(tailSizeInBytes)]
        fn tailSizeInBytes(&self) -> NSUInteger;

        #[optional]
        #[method(isSparse)]
        fn isSparse(&self) -> bool;

        #[method(allowGPUOptimizedContents)]
        fn allowGPUOptimizedContents(&self) -> bool;

        #[method(compressionType)]
        unsafe fn compressionType(&self) -> MTLTextureCompressionType;

        #[cfg(feature = "Metal_MTLTypes")]
        #[method(gpuResourceID)]
        unsafe fn gpuResourceID(&self) -> MTLResourceID;

        #[cfg(feature = "Metal_MTLTypes")]
        #[method(getBytes:bytesPerRow:bytesPerImage:fromRegion:mipmapLevel:slice:)]
        unsafe fn getBytes_bytesPerRow_bytesPerImage_fromRegion_mipmapLevel_slice(
            &self,
            pixel_bytes: NonNull<c_void>,
            bytes_per_row: NSUInteger,
            bytes_per_image: NSUInteger,
            region: MTLRegion,
            level: NSUInteger,
            slice: NSUInteger,
        );

        #[cfg(feature = "Metal_MTLTypes")]
        #[method(replaceRegion:mipmapLevel:slice:withBytes:bytesPerRow:bytesPerImage:)]
        unsafe fn replaceRegion_mipmapLevel_slice_withBytes_bytesPerRow_bytesPerImage(
            &self,
            region: MTLRegion,
            level: NSUInteger,
            slice: NSUInteger,
            pixel_bytes: NonNull<c_void>,
            bytes_per_row: NSUInteger,
            bytes_per_image: NSUInteger,
        );

        #[cfg(feature = "Metal_MTLTypes")]
        #[method(getBytes:bytesPerRow:fromRegion:mipmapLevel:)]
        unsafe fn getBytes_bytesPerRow_fromRegion_mipmapLevel(
            &self,
            pixel_bytes: NonNull<c_void>,
            bytes_per_row: NSUInteger,
            region: MTLRegion,
            level: NSUInteger,
        );

        #[cfg(feature = "Metal_MTLTypes")]
        #[method(replaceRegion:mipmapLevel:withBytes:bytesPerRow:)]
        unsafe fn replaceRegion_mipmapLevel_withBytes_bytesPerRow(
            &self,
            region: MTLRegion,
            level: NSUInteger,
            pixel_bytes: NonNull<c_void>,
            bytes_per_row: NSUInteger,
        );

        #[cfg(feature = "Metal_MTLPixelFormat")]
        #[method_id(@__retain_semantics New newTextureViewWithPixelFormat:)]
        fn newTextureViewWithPixelFormat(
            &self,
            pixel_format: MTLPixelFormat,
        ) -> Option<Id<ProtocolObject<dyn MTLTexture>>>;

        #[cfg(all(feature = "Foundation_NSRange", feature = "Metal_MTLPixelFormat"))]
        #[method_id(@__retain_semantics New newTextureViewWithPixelFormat:textureType:levels:slices:)]
        unsafe fn newTextureViewWithPixelFormat_textureType_levels_slices(
            &self,
            pixel_format: MTLPixelFormat,
            texture_type: MTLTextureType,
            level_range: NSRange,
            slice_range: NSRange,
        ) -> Option<Id<ProtocolObject<dyn MTLTexture>>>;

        #[method_id(@__retain_semantics New newSharedTextureHandle)]
        fn newSharedTextureHandle(&self) -> Option<Id<MTLSharedTextureHandle>>;

        #[method_id(@__retain_semantics Other remoteStorageTexture)]
        fn remoteStorageTexture(&self) -> Option<Id<ProtocolObject<dyn MTLTexture>>>;

        #[cfg(feature = "Metal_MTLDevice")]
        #[method_id(@__retain_semantics New newRemoteTextureViewForDevice:)]
        unsafe fn newRemoteTextureViewForDevice(
            &self,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Id<ProtocolObject<dyn MTLTexture>>>;

        #[method(swizzle)]
        fn swizzle(&self) -> MTLTextureSwizzleChannels;

        #[cfg(all(feature = "Foundation_NSRange", feature = "Metal_MTLPixelFormat"))]
        #[method_id(@__retain_semantics New newTextureViewWithPixelFormat:textureType:levels:slices:swizzle:)]
        unsafe fn newTextureViewWithPixelFormat_textureType_levels_slices_swizzle(
            &self,
            pixel_format: MTLPixelFormat,
            texture_type: MTLTextureType,
            level_range: NSRange,
            slice_range: NSRange,
            swizzle: MTLTextureSwizzleChannels,
        ) -> Option<Id<ProtocolObject<dyn MTLTexture>>>;
    }

    #[cfg(feature = "Metal_MTLResource")]
    unsafe impl ProtocolType for dyn MTLTexture {}
);

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLClearColor {
    pub red: c_double,
    pub green: c_double,
    pub blue: c_double,
    pub alpha: c_double,
}

unsafe impl Encode for MTLClearColor {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <c_double>::ENCODING,
            <c_double>::ENCODING,
            <c_double>::ENCODING,
            <c_double>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTLClearColor {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// TODO: pub fn MTLClearColorMake(red: c_double,green: c_double,blue: c_double,alpha: c_double,) -> MTLClearColor;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLLoadAction(pub NSUInteger);
impl MTLLoadAction {
    #[doc(alias = "MTLLoadActionDontCare")]
    pub const DontCare: Self = Self(0);
    #[doc(alias = "MTLLoadActionLoad")]
    pub const Load: Self = Self(1);
    #[doc(alias = "MTLLoadActionClear")]
    pub const Clear: Self = Self(2);
}

unsafe impl Encode for MTLLoadAction {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLLoadAction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLStoreAction(pub NSUInteger);
impl MTLStoreAction {
    #[doc(alias = "MTLStoreActionDontCare")]
    pub const DontCare: Self = Self(0);
    #[doc(alias = "MTLStoreActionStore")]
    pub const Store: Self = Self(1);
    #[doc(alias = "MTLStoreActionMultisampleResolve")]
    pub const MultisampleResolve: Self = Self(2);
    #[doc(alias = "MTLStoreActionStoreAndMultisampleResolve")]
    pub const StoreAndMultisampleResolve: Self = Self(3);
    #[doc(alias = "MTLStoreActionUnknown")]
    pub const Unknown: Self = Self(4);
    #[doc(alias = "MTLStoreActionCustomSampleDepthStore")]
    pub const CustomSampleDepthStore: Self = Self(5);
}

unsafe impl Encode for MTLStoreAction {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLStoreAction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLStoreActionOptions(pub NSUInteger);
bitflags::bitflags! {
    impl MTLStoreActionOptions: NSUInteger {
        const MTLStoreActionOptionNone = 0;
        const MTLStoreActionOptionCustomSamplePositions = 1<<0;
    }
}

unsafe impl Encode for MTLStoreActionOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLStoreActionOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRenderPassAttachmentDescriptor;
);

unsafe impl NSCopying for MTLRenderPassAttachmentDescriptor {}

unsafe impl CopyingHelper for MTLRenderPassAttachmentDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLRenderPassAttachmentDescriptor {}

extern_methods!(
    unsafe impl MTLRenderPassAttachmentDescriptor {
        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLResource",
            feature = "MTLTexture"
        ))]
        #[method_id(@__retain_semantics Other texture)]
        pub fn texture(&self) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLResource",
            feature = "MTLTexture"
        ))]
        #[method(setTexture:)]
        pub fn setTexture(&self, texture: Option<&ProtocolObject<dyn MTLTexture>>);

        #[method(level)]
        pub fn level(&self) -> NSUInteger;

        #[method(setLevel:)]
        pub fn setLevel(&self, level: NSUInteger);

        #[method(slice)]
        pub fn slice(&self) -> NSUInteger;

        #[method(setSlice:)]
        pub fn setSlice(&self, slice: NSUInteger);

        #[method(depthPlane)]
        pub fn depthPlane(&self) -> NSUInteger;

        #[method(setDepthPlane:)]
        pub fn setDepthPlane(&self, depth_plane: NSUInteger);

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLResource",
            feature = "MTLTexture"
        ))]
        #[method_id(@__retain_semantics Other resolveTexture)]
        pub fn resolveTexture(&self) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLResource",
            feature = "MTLTexture"
        ))]
        #[method(setResolveTexture:)]
        pub fn setResolveTexture(&self, resolve_texture: Option<&ProtocolObject<dyn MTLTexture>>);

        #[method(resolveLevel)]
        pub fn resolveLevel(&self) -> NSUInteger;

        #[method(setResolveLevel:)]
        pub fn setResolveLevel(&self, resolve_level: NSUInteger);

        #[method(resolveSlice)]
        pub fn resolveSlice(&self) -> NSUInteger;

        #[method(setResolveSlice:)]
        pub fn setResolveSlice(&self, resolve_slice: NSUInteger);

        #[method(resolveDepthPlane)]
        pub fn resolveDepthPlane(&self) -> NSUInteger;

        #[method(setResolveDepthPlane:)]
        pub fn setResolveDepthPlane(&self, resolve_depth_plane: NSUInteger);

        #[method(loadAction)]
        pub fn loadAction(&self) -> MTLLoadAction;

        #[method(setLoadAction:)]
        pub fn setLoadAction(&self, load_action: MTLLoadAction);

        #[method(storeAction)]
        pub fn storeAction(&self) -> MTLStoreAction;

        #[method(setStoreAction:)]
        pub fn setStoreAction(&self, store_action: MTLStoreAction);

        #[method(storeActionOptions)]
        pub fn storeActionOptions(&self) -> MTLStoreActionOptions;

        #[method(setStoreActionOptions:)]
        pub fn setStoreActionOptions(&self, store_action_options: MTLStoreActionOptions);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLRenderPassAttachmentDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(MTLRenderPassAttachmentDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRenderPassColorAttachmentDescriptor;
);

unsafe impl NSCopying for MTLRenderPassColorAttachmentDescriptor {}

unsafe impl CopyingHelper for MTLRenderPassColorAttachmentDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLRenderPassColorAttachmentDescriptor {}

extern_methods!(
    unsafe impl MTLRenderPassColorAttachmentDescriptor {
        #[method(clearColor)]
        pub fn clearColor(&self) -> MTLClearColor;

        #[method(setClearColor:)]
        pub fn setClearColor(&self, clear_color: MTLClearColor);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLRenderPassColorAttachmentDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Retained<Self>;
    }
);

impl DefaultRetained for MTLRenderPassColorAttachmentDescriptor {
    #[inline]
    fn default_retained() -> Retained<Self> {
        Self::new()
    }
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLMultisampleDepthResolveFilter(pub NSUInteger);
impl MTLMultisampleDepthResolveFilter {
    #[doc(alias = "MTLMultisampleDepthResolveFilterSample0")]
    pub const Sample0: Self = Self(0);
    #[doc(alias = "MTLMultisampleDepthResolveFilterMin")]
    pub const Min: Self = Self(1);
    #[doc(alias = "MTLMultisampleDepthResolveFilterMax")]
    pub const Max: Self = Self(2);
}

unsafe impl Encode for MTLMultisampleDepthResolveFilter {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLMultisampleDepthResolveFilter {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(MTLRenderPassAttachmentDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRenderPassDepthAttachmentDescriptor;
);

unsafe impl NSCopying for MTLRenderPassDepthAttachmentDescriptor {}

unsafe impl CopyingHelper for MTLRenderPassDepthAttachmentDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLRenderPassDepthAttachmentDescriptor {}

extern_methods!(
    unsafe impl MTLRenderPassDepthAttachmentDescriptor {
        #[method(clearDepth)]
        pub fn clearDepth(&self) -> c_double;

        #[method(setClearDepth:)]
        pub fn setClearDepth(&self, clear_depth: c_double);

        #[method(depthResolveFilter)]
        pub fn depthResolveFilter(&self) -> MTLMultisampleDepthResolveFilter;

        #[method(setDepthResolveFilter:)]
        pub fn setDepthResolveFilter(&self, depth_resolve_filter: MTLMultisampleDepthResolveFilter);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLRenderPassDepthAttachmentDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLMultisampleStencilResolveFilter(pub NSUInteger);
impl MTLMultisampleStencilResolveFilter {
    #[doc(alias = "MTLMultisampleStencilResolveFilterSample0")]
    pub const Sample0: Self = Self(0);
    #[doc(alias = "MTLMultisampleStencilResolveFilterDepthResolvedSample")]
    pub const DepthResolvedSample: Self = Self(1);
}

unsafe impl Encode for MTLMultisampleStencilResolveFilter {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLMultisampleStencilResolveFilter {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(MTLRenderPassAttachmentDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRenderPassStencilAttachmentDescriptor;
);

unsafe impl NSCopying for MTLRenderPassStencilAttachmentDescriptor {}

unsafe impl CopyingHelper for MTLRenderPassStencilAttachmentDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLRenderPassStencilAttachmentDescriptor {}

extern_methods!(
    unsafe impl MTLRenderPassStencilAttachmentDescriptor {
        #[method(clearStencil)]
        pub fn clearStencil(&self) -> u32;

        #[method(setClearStencil:)]
        pub fn setClearStencil(&self, clear_stencil: u32);

        #[method(stencilResolveFilter)]
        pub fn stencilResolveFilter(&self) -> MTLMultisampleStencilResolveFilter;

        #[method(setStencilResolveFilter:)]
        pub fn setStencilResolveFilter(
            &self,
            stencil_resolve_filter: MTLMultisampleStencilResolveFilter,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLRenderPassStencilAttachmentDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRenderPassColorAttachmentDescriptorArray;
);

unsafe impl NSObjectProtocol for MTLRenderPassColorAttachmentDescriptorArray {}

extern_methods!(
    unsafe impl MTLRenderPassColorAttachmentDescriptorArray {
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            attachment_index: NSUInteger,
        ) -> Retained<MTLRenderPassColorAttachmentDescriptor>;

        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            attachment: Option<&MTLRenderPassColorAttachmentDescriptor>,
            attachment_index: NSUInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLRenderPassColorAttachmentDescriptorArray {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRenderPassSampleBufferAttachmentDescriptor;
);

unsafe impl NSCopying for MTLRenderPassSampleBufferAttachmentDescriptor {}

unsafe impl CopyingHelper for MTLRenderPassSampleBufferAttachmentDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLRenderPassSampleBufferAttachmentDescriptor {}

extern_methods!(
    unsafe impl MTLRenderPassSampleBufferAttachmentDescriptor {
        #[cfg(feature = "MTLCounters")]
        #[method_id(@__retain_semantics Other sampleBuffer)]
        pub fn sampleBuffer(&self) -> Option<Retained<ProtocolObject<dyn MTLCounterSampleBuffer>>>;

        #[cfg(feature = "MTLCounters")]
        #[method(setSampleBuffer:)]
        pub fn setSampleBuffer(
            &self,
            sample_buffer: Option<&ProtocolObject<dyn MTLCounterSampleBuffer>>,
        );

        #[method(startOfVertexSampleIndex)]
        pub fn startOfVertexSampleIndex(&self) -> NSUInteger;

        #[method(setStartOfVertexSampleIndex:)]
        pub unsafe fn setStartOfVertexSampleIndex(&self, start_of_vertex_sample_index: NSUInteger);

        #[method(endOfVertexSampleIndex)]
        pub fn endOfVertexSampleIndex(&self) -> NSUInteger;

        #[method(setEndOfVertexSampleIndex:)]
        pub unsafe fn setEndOfVertexSampleIndex(&self, end_of_vertex_sample_index: NSUInteger);

        #[method(startOfFragmentSampleIndex)]
        pub fn startOfFragmentSampleIndex(&self) -> NSUInteger;

        #[method(setStartOfFragmentSampleIndex:)]
        pub unsafe fn setStartOfFragmentSampleIndex(
            &self,
            start_of_fragment_sample_index: NSUInteger,
        );

        #[method(endOfFragmentSampleIndex)]
        pub fn endOfFragmentSampleIndex(&self) -> NSUInteger;

        #[method(setEndOfFragmentSampleIndex:)]
        pub unsafe fn setEndOfFragmentSampleIndex(&self, end_of_fragment_sample_index: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLRenderPassSampleBufferAttachmentDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRenderPassSampleBufferAttachmentDescriptorArray;
);

unsafe impl NSObjectProtocol for MTLRenderPassSampleBufferAttachmentDescriptorArray {}

extern_methods!(
    unsafe impl MTLRenderPassSampleBufferAttachmentDescriptorArray {
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            attachment_index: NSUInteger,
        ) -> Retained<MTLRenderPassSampleBufferAttachmentDescriptor>;

        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            attachment: Option<&MTLRenderPassSampleBufferAttachmentDescriptor>,
            attachment_index: NSUInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLRenderPassSampleBufferAttachmentDescriptorArray {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRenderPassDescriptor;
);

unsafe impl NSCopying for MTLRenderPassDescriptor {}

unsafe impl CopyingHelper for MTLRenderPassDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLRenderPassDescriptor {}

extern_methods!(
    unsafe impl MTLRenderPassDescriptor {
        #[method_id(@__retain_semantics Other renderPassDescriptor)]
        pub fn renderPassDescriptor() -> Retained<MTLRenderPassDescriptor>;

        #[method_id(@__retain_semantics Other colorAttachments)]
        pub fn colorAttachments(&self) -> Retained<MTLRenderPassColorAttachmentDescriptorArray>;

        #[method_id(@__retain_semantics Other depthAttachment)]
        pub fn depthAttachment(&self) -> Retained<MTLRenderPassDepthAttachmentDescriptor>;

        #[method(setDepthAttachment:)]
        pub fn setDepthAttachment(
            &self,
            depth_attachment: Option<&MTLRenderPassDepthAttachmentDescriptor>,
        );

        #[method_id(@__retain_semantics Other stencilAttachment)]
        pub fn stencilAttachment(&self) -> Retained<MTLRenderPassStencilAttachmentDescriptor>;

        #[method(setStencilAttachment:)]
        pub fn setStencilAttachment(
            &self,
            stencil_attachment: Option<&MTLRenderPassStencilAttachmentDescriptor>,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLBuffer",
            feature = "MTLResource"
        ))]
        #[method_id(@__retain_semantics Other visibilityResultBuffer)]
        pub fn visibilityResultBuffer(&self) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLBuffer",
            feature = "MTLResource"
        ))]
        #[method(setVisibilityResultBuffer:)]
        pub fn setVisibilityResultBuffer(
            &self,
            visibility_result_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
        );

        #[method(renderTargetArrayLength)]
        pub fn renderTargetArrayLength(&self) -> NSUInteger;

        #[method(setRenderTargetArrayLength:)]
        pub unsafe fn setRenderTargetArrayLength(&self, render_target_array_length: NSUInteger);

        #[method(imageblockSampleLength)]
        pub fn imageblockSampleLength(&self) -> NSUInteger;

        #[method(setImageblockSampleLength:)]
        pub unsafe fn setImageblockSampleLength(&self, imageblock_sample_length: NSUInteger);

        #[method(threadgroupMemoryLength)]
        pub fn threadgroupMemoryLength(&self) -> NSUInteger;

        #[method(setThreadgroupMemoryLength:)]
        pub unsafe fn setThreadgroupMemoryLength(&self, threadgroup_memory_length: NSUInteger);

        #[method(tileWidth)]
        pub fn tileWidth(&self) -> NSUInteger;

        #[method(setTileWidth:)]
        pub fn setTileWidth(&self, tile_width: NSUInteger);

        #[method(tileHeight)]
        pub fn tileHeight(&self) -> NSUInteger;

        #[method(setTileHeight:)]
        pub fn setTileHeight(&self, tile_height: NSUInteger);

        #[method(defaultRasterSampleCount)]
        pub fn defaultRasterSampleCount(&self) -> NSUInteger;

        #[method(setDefaultRasterSampleCount:)]
        pub fn setDefaultRasterSampleCount(&self, default_raster_sample_count: NSUInteger);

        #[method(renderTargetWidth)]
        pub fn renderTargetWidth(&self) -> NSUInteger;

        #[method(setRenderTargetWidth:)]
        pub fn setRenderTargetWidth(&self, render_target_width: NSUInteger);

        #[method(renderTargetHeight)]
        pub fn renderTargetHeight(&self) -> NSUInteger;

        #[method(setRenderTargetHeight:)]
        pub fn setRenderTargetHeight(&self, render_target_height: NSUInteger);

        #[cfg(feature = "MTLTypes")]
        #[method(setSamplePositions:count:)]
        pub unsafe fn setSamplePositions_count(
            &self,
            positions: *mut MTLSamplePosition,
            count: NSUInteger,
        );

        #[cfg(feature = "MTLTypes")]
        #[method(getSamplePositions:count:)]
        pub unsafe fn getSamplePositions_count(
            &self,
            positions: *mut MTLSamplePosition,
            count: NSUInteger,
        ) -> NSUInteger;

        #[cfg(feature = "MTLRasterizationRate")]
        #[method_id(@__retain_semantics Other rasterizationRateMap)]
        pub fn rasterizationRateMap(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLRasterizationRateMap>>>;

        #[cfg(feature = "MTLRasterizationRate")]
        #[method(setRasterizationRateMap:)]
        pub fn setRasterizationRateMap(
            &self,
            rasterization_rate_map: Option<&ProtocolObject<dyn MTLRasterizationRateMap>>,
        );

        #[method_id(@__retain_semantics Other sampleBufferAttachments)]
        pub fn sampleBufferAttachments(
            &self,
        ) -> Retained<MTLRenderPassSampleBufferAttachmentDescriptorArray>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLRenderPassDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

// TODO: pub fn MTLClearColorMake(red: c_double,green: c_double,blue: c_double,alpha: c_double,) -> MTLClearColor;

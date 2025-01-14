//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtldispatchthreadgroupsindirectarguments?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLDispatchThreadgroupsIndirectArguments {
    pub threadgroupsPerGrid: [u32; 3],
}

unsafe impl Encode for MTLDispatchThreadgroupsIndirectArguments {
    const ENCODING: Encoding = Encoding::Struct("?", &[<[u32; 3]>::ENCODING]);
}

unsafe impl RefEncode for MTLDispatchThreadgroupsIndirectArguments {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlstageinregionindirectarguments?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLStageInRegionIndirectArguments {
    pub stageInOrigin: [u32; 3],
    pub stageInSize: [u32; 3],
}

unsafe impl Encode for MTLStageInRegionIndirectArguments {
    const ENCODING: Encoding = Encoding::Struct("?", &[<[u32; 3]>::ENCODING, <[u32; 3]>::ENCODING]);
}

unsafe impl RefEncode for MTLStageInRegionIndirectArguments {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder?language=objc)
    #[cfg(feature = "MTLCommandEncoder")]
    pub unsafe trait MTLComputeCommandEncoder: MTLCommandEncoder {
        #[cfg(feature = "MTLCommandBuffer")]
        #[method(dispatchType)]
        unsafe fn dispatchType(&self) -> MTLDispatchType;

        #[cfg(feature = "MTLComputePipeline")]
        #[method(setComputePipelineState:)]
        fn setComputePipelineState(&self, state: &ProtocolObject<dyn MTLComputePipelineState>);

        #[method(setBytes:length:atIndex:)]
        unsafe fn setBytes_length_atIndex(
            &self,
            bytes: NonNull<c_void>,
            length: NSUInteger,
            index: NSUInteger,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLBuffer",
            feature = "MTLResource"
        ))]
        #[method(setBuffer:offset:atIndex:)]
        unsafe fn setBuffer_offset_atIndex(
            &self,
            buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            offset: NSUInteger,
            index: NSUInteger,
        );

        #[method(setBufferOffset:atIndex:)]
        unsafe fn setBufferOffset_atIndex(&self, offset: NSUInteger, index: NSUInteger);

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLBuffer",
            feature = "MTLResource"
        ))]
        #[method(setBuffers:offsets:withRange:)]
        unsafe fn setBuffers_offsets_withRange(
            &self,
            buffers: NonNull<*const ProtocolObject<dyn MTLBuffer>>,
            offsets: NonNull<NSUInteger>,
            range: NSRange,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLBuffer",
            feature = "MTLResource"
        ))]
        #[method(setBuffer:offset:attributeStride:atIndex:)]
        unsafe fn setBuffer_offset_attributeStride_atIndex(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: NSUInteger,
            stride: NSUInteger,
            index: NSUInteger,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLBuffer",
            feature = "MTLResource"
        ))]
        #[method(setBuffers:offsets:attributeStrides:withRange:)]
        unsafe fn setBuffers_offsets_attributeStrides_withRange(
            &self,
            buffers: NonNull<*const ProtocolObject<dyn MTLBuffer>>,
            offsets: NonNull<NSUInteger>,
            strides: NonNull<NSUInteger>,
            range: NSRange,
        );

        #[method(setBufferOffset:attributeStride:atIndex:)]
        unsafe fn setBufferOffset_attributeStride_atIndex(
            &self,
            offset: NSUInteger,
            stride: NSUInteger,
            index: NSUInteger,
        );

        #[method(setBytes:length:attributeStride:atIndex:)]
        unsafe fn setBytes_length_attributeStride_atIndex(
            &self,
            bytes: NonNull<c_void>,
            length: NSUInteger,
            stride: NSUInteger,
            index: NSUInteger,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLResource",
            feature = "MTLVisibleFunctionTable"
        ))]
        #[method(setVisibleFunctionTable:atBufferIndex:)]
        unsafe fn setVisibleFunctionTable_atBufferIndex(
            &self,
            visible_function_table: Option<&ProtocolObject<dyn MTLVisibleFunctionTable>>,
            buffer_index: NSUInteger,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLResource",
            feature = "MTLVisibleFunctionTable"
        ))]
        #[method(setVisibleFunctionTables:withBufferRange:)]
        unsafe fn setVisibleFunctionTables_withBufferRange(
            &self,
            visible_function_tables: NonNull<*const ProtocolObject<dyn MTLVisibleFunctionTable>>,
            range: NSRange,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLIntersectionFunctionTable",
            feature = "MTLResource"
        ))]
        #[method(setIntersectionFunctionTable:atBufferIndex:)]
        unsafe fn setIntersectionFunctionTable_atBufferIndex(
            &self,
            intersection_function_table: Option<&ProtocolObject<dyn MTLIntersectionFunctionTable>>,
            buffer_index: NSUInteger,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLIntersectionFunctionTable",
            feature = "MTLResource"
        ))]
        #[method(setIntersectionFunctionTables:withBufferRange:)]
        unsafe fn setIntersectionFunctionTables_withBufferRange(
            &self,
            intersection_function_tables: NonNull<
                *const ProtocolObject<dyn MTLIntersectionFunctionTable>,
            >,
            range: NSRange,
        );

        #[cfg(all(
            feature = "MTLAccelerationStructure",
            feature = "MTLAllocation",
            feature = "MTLResource"
        ))]
        #[method(setAccelerationStructure:atBufferIndex:)]
        unsafe fn setAccelerationStructure_atBufferIndex(
            &self,
            acceleration_structure: Option<&ProtocolObject<dyn MTLAccelerationStructure>>,
            buffer_index: NSUInteger,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLResource",
            feature = "MTLTexture"
        ))]
        #[method(setTexture:atIndex:)]
        unsafe fn setTexture_atIndex(
            &self,
            texture: Option<&ProtocolObject<dyn MTLTexture>>,
            index: NSUInteger,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLResource",
            feature = "MTLTexture"
        ))]
        #[method(setTextures:withRange:)]
        unsafe fn setTextures_withRange(
            &self,
            textures: NonNull<*const ProtocolObject<dyn MTLTexture>>,
            range: NSRange,
        );

        #[cfg(feature = "MTLSampler")]
        #[method(setSamplerState:atIndex:)]
        unsafe fn setSamplerState_atIndex(
            &self,
            sampler: Option<&ProtocolObject<dyn MTLSamplerState>>,
            index: NSUInteger,
        );

        #[cfg(feature = "MTLSampler")]
        #[method(setSamplerStates:withRange:)]
        unsafe fn setSamplerStates_withRange(
            &self,
            samplers: NonNull<*const ProtocolObject<dyn MTLSamplerState>>,
            range: NSRange,
        );

        #[cfg(feature = "MTLSampler")]
        #[method(setSamplerState:lodMinClamp:lodMaxClamp:atIndex:)]
        unsafe fn setSamplerState_lodMinClamp_lodMaxClamp_atIndex(
            &self,
            sampler: Option<&ProtocolObject<dyn MTLSamplerState>>,
            lod_min_clamp: c_float,
            lod_max_clamp: c_float,
            index: NSUInteger,
        );

        #[cfg(feature = "MTLSampler")]
        #[method(setSamplerStates:lodMinClamps:lodMaxClamps:withRange:)]
        unsafe fn setSamplerStates_lodMinClamps_lodMaxClamps_withRange(
            &self,
            samplers: NonNull<*const ProtocolObject<dyn MTLSamplerState>>,
            lod_min_clamps: NonNull<c_float>,
            lod_max_clamps: NonNull<c_float>,
            range: NSRange,
        );

        #[method(setThreadgroupMemoryLength:atIndex:)]
        unsafe fn setThreadgroupMemoryLength_atIndex(&self, length: NSUInteger, index: NSUInteger);

        #[method(setImageblockWidth:height:)]
        unsafe fn setImageblockWidth_height(&self, width: NSUInteger, height: NSUInteger);

        #[cfg(feature = "MTLTypes")]
        #[method(setStageInRegion:)]
        unsafe fn setStageInRegion(&self, region: MTLRegion);

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLBuffer",
            feature = "MTLResource"
        ))]
        #[method(setStageInRegionWithIndirectBuffer:indirectBufferOffset:)]
        unsafe fn setStageInRegionWithIndirectBuffer_indirectBufferOffset(
            &self,
            indirect_buffer: &ProtocolObject<dyn MTLBuffer>,
            indirect_buffer_offset: NSUInteger,
        );

        #[cfg(feature = "MTLTypes")]
        #[method(dispatchThreadgroups:threadsPerThreadgroup:)]
        fn dispatchThreadgroups_threadsPerThreadgroup(
            &self,
            threadgroups_per_grid: MTLSize,
            threads_per_threadgroup: MTLSize,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLBuffer",
            feature = "MTLResource",
            feature = "MTLTypes"
        ))]
        #[method(dispatchThreadgroupsWithIndirectBuffer:indirectBufferOffset:threadsPerThreadgroup:)]
        unsafe fn dispatchThreadgroupsWithIndirectBuffer_indirectBufferOffset_threadsPerThreadgroup(
            &self,
            indirect_buffer: &ProtocolObject<dyn MTLBuffer>,
            indirect_buffer_offset: NSUInteger,
            threads_per_threadgroup: MTLSize,
        );

        #[cfg(feature = "MTLTypes")]
        #[method(dispatchThreads:threadsPerThreadgroup:)]
        fn dispatchThreads_threadsPerThreadgroup(
            &self,
            threads_per_grid: MTLSize,
            threads_per_threadgroup: MTLSize,
        );

        #[cfg(feature = "MTLFence")]
        #[method(updateFence:)]
        fn updateFence(&self, fence: &ProtocolObject<dyn MTLFence>);

        #[cfg(feature = "MTLFence")]
        #[method(waitForFence:)]
        fn waitForFence(&self, fence: &ProtocolObject<dyn MTLFence>);

        #[cfg(all(feature = "MTLAllocation", feature = "MTLResource"))]
        #[method(useResource:usage:)]
        fn useResource_usage(
            &self,
            resource: &ProtocolObject<dyn MTLResource>,
            usage: MTLResourceUsage,
        );

        #[cfg(all(feature = "MTLAllocation", feature = "MTLResource"))]
        #[method(useResources:count:usage:)]
        unsafe fn useResources_count_usage(
            &self,
            resources: NonNull<NonNull<ProtocolObject<dyn MTLResource>>>,
            count: NSUInteger,
            usage: MTLResourceUsage,
        );

        #[cfg(all(feature = "MTLAllocation", feature = "MTLHeap"))]
        #[method(useHeap:)]
        fn useHeap(&self, heap: &ProtocolObject<dyn MTLHeap>);

        #[cfg(all(feature = "MTLAllocation", feature = "MTLHeap"))]
        #[method(useHeaps:count:)]
        unsafe fn useHeaps_count(
            &self,
            heaps: NonNull<NonNull<ProtocolObject<dyn MTLHeap>>>,
            count: NSUInteger,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLIndirectCommandBuffer",
            feature = "MTLResource"
        ))]
        #[method(executeCommandsInBuffer:withRange:)]
        unsafe fn executeCommandsInBuffer_withRange(
            &self,
            indirect_command_buffer: &ProtocolObject<dyn MTLIndirectCommandBuffer>,
            execution_range: NSRange,
        );

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLBuffer",
            feature = "MTLIndirectCommandBuffer",
            feature = "MTLResource"
        ))]
        #[method(executeCommandsInBuffer:indirectBuffer:indirectBufferOffset:)]
        unsafe fn executeCommandsInBuffer_indirectBuffer_indirectBufferOffset(
            &self,
            indirect_commandbuffer: &ProtocolObject<dyn MTLIndirectCommandBuffer>,
            indirect_range_buffer: &ProtocolObject<dyn MTLBuffer>,
            indirect_buffer_offset: NSUInteger,
        );

        #[method(memoryBarrierWithScope:)]
        unsafe fn memoryBarrierWithScope(&self, scope: MTLBarrierScope);

        #[cfg(all(feature = "MTLAllocation", feature = "MTLResource"))]
        #[method(memoryBarrierWithResources:count:)]
        unsafe fn memoryBarrierWithResources_count(
            &self,
            resources: NonNull<NonNull<ProtocolObject<dyn MTLResource>>>,
            count: NSUInteger,
        );

        #[cfg(feature = "MTLCounters")]
        #[method(sampleCountersInBuffer:atSampleIndex:withBarrier:)]
        unsafe fn sampleCountersInBuffer_atSampleIndex_withBarrier(
            &self,
            sample_buffer: &ProtocolObject<dyn MTLCounterSampleBuffer>,
            sample_index: NSUInteger,
            barrier: bool,
        );
    }

    #[cfg(feature = "MTLCommandEncoder")]
    unsafe impl ProtocolType for dyn MTLComputeCommandEncoder {}
);

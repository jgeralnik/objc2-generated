//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_protocol!(
    pub unsafe trait MTLIndirectRenderCommand: NSObjectProtocol + IsRetainable {
        #[cfg(feature = "MTLRenderPipeline")]
        #[method(setRenderPipelineState:)]
        unsafe fn setRenderPipelineState(
            &self,
            pipeline_state: &ProtocolObject<dyn MTLRenderPipelineState>,
        );

        #[cfg(all(feature = "MTLBuffer", feature = "MTLResource"))]
        #[method(setVertexBuffer:offset:atIndex:)]
        unsafe fn setVertexBuffer_offset_atIndex(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: NSUInteger,
            index: NSUInteger,
        );

        #[cfg(all(feature = "MTLBuffer", feature = "MTLResource"))]
        #[method(setFragmentBuffer:offset:atIndex:)]
        unsafe fn setFragmentBuffer_offset_atIndex(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: NSUInteger,
            index: NSUInteger,
        );

        #[cfg(all(feature = "MTLBuffer", feature = "MTLResource"))]
        #[method(setVertexBuffer:offset:attributeStride:atIndex:)]
        unsafe fn setVertexBuffer_offset_attributeStride_atIndex(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: NSUInteger,
            stride: NSUInteger,
            index: NSUInteger,
        );

        #[cfg(all(feature = "MTLBuffer", feature = "MTLResource"))]
        #[method(drawPatches:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:instanceCount:baseInstance:tessellationFactorBuffer:tessellationFactorBufferOffset:tessellationFactorBufferInstanceStride:)]
        unsafe fn drawPatches_patchStart_patchCount_patchIndexBuffer_patchIndexBufferOffset_instanceCount_baseInstance_tessellationFactorBuffer_tessellationFactorBufferOffset_tessellationFactorBufferInstanceStride(
            &self,
            number_of_patch_control_points: NSUInteger,
            patch_start: NSUInteger,
            patch_count: NSUInteger,
            patch_index_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            patch_index_buffer_offset: NSUInteger,
            instance_count: NSUInteger,
            base_instance: NSUInteger,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: NSUInteger,
            instance_stride: NSUInteger,
        );

        #[cfg(all(feature = "MTLBuffer", feature = "MTLResource"))]
        #[method(drawIndexedPatches:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:controlPointIndexBuffer:controlPointIndexBufferOffset:instanceCount:baseInstance:tessellationFactorBuffer:tessellationFactorBufferOffset:tessellationFactorBufferInstanceStride:)]
        unsafe fn drawIndexedPatches_patchStart_patchCount_patchIndexBuffer_patchIndexBufferOffset_controlPointIndexBuffer_controlPointIndexBufferOffset_instanceCount_baseInstance_tessellationFactorBuffer_tessellationFactorBufferOffset_tessellationFactorBufferInstanceStride(
            &self,
            number_of_patch_control_points: NSUInteger,
            patch_start: NSUInteger,
            patch_count: NSUInteger,
            patch_index_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            patch_index_buffer_offset: NSUInteger,
            control_point_index_buffer: &ProtocolObject<dyn MTLBuffer>,
            control_point_index_buffer_offset: NSUInteger,
            instance_count: NSUInteger,
            base_instance: NSUInteger,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: NSUInteger,
            instance_stride: NSUInteger,
        );

        #[cfg(feature = "MTLRenderCommandEncoder")]
        #[method(drawPrimitives:vertexStart:vertexCount:instanceCount:baseInstance:)]
        unsafe fn drawPrimitives_vertexStart_vertexCount_instanceCount_baseInstance(
            &self,
            primitive_type: MTLPrimitiveType,
            vertex_start: NSUInteger,
            vertex_count: NSUInteger,
            instance_count: NSUInteger,
            base_instance: NSUInteger,
        );

        #[cfg(all(
            feature = "MTLBuffer",
            feature = "MTLRenderCommandEncoder",
            feature = "MTLResource",
            feature = "MTLStageInputOutputDescriptor"
        ))]
        #[method(drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:baseVertex:baseInstance:)]
        unsafe fn drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset_instanceCount_baseVertex_baseInstance(
            &self,
            primitive_type: MTLPrimitiveType,
            index_count: NSUInteger,
            index_type: MTLIndexType,
            index_buffer: &ProtocolObject<dyn MTLBuffer>,
            index_buffer_offset: NSUInteger,
            instance_count: NSUInteger,
            base_vertex: NSInteger,
            base_instance: NSUInteger,
        );

        #[method(setObjectThreadgroupMemoryLength:atIndex:)]
        unsafe fn setObjectThreadgroupMemoryLength_atIndex(
            &self,
            length: NSUInteger,
            index: NSUInteger,
        );

        #[cfg(all(feature = "MTLBuffer", feature = "MTLResource"))]
        #[method(setObjectBuffer:offset:atIndex:)]
        unsafe fn setObjectBuffer_offset_atIndex(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: NSUInteger,
            index: NSUInteger,
        );

        #[cfg(all(feature = "MTLBuffer", feature = "MTLResource"))]
        #[method(setMeshBuffer:offset:atIndex:)]
        unsafe fn setMeshBuffer_offset_atIndex(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: NSUInteger,
            index: NSUInteger,
        );

        #[cfg(feature = "MTLTypes")]
        #[method(drawMeshThreadgroups:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)]
        unsafe fn drawMeshThreadgroups_threadsPerObjectThreadgroup_threadsPerMeshThreadgroup(
            &self,
            threadgroups_per_grid: MTLSize,
            threads_per_object_threadgroup: MTLSize,
            threads_per_mesh_threadgroup: MTLSize,
        );

        #[cfg(feature = "MTLTypes")]
        #[method(drawMeshThreads:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)]
        unsafe fn drawMeshThreads_threadsPerObjectThreadgroup_threadsPerMeshThreadgroup(
            &self,
            threads_per_grid: MTLSize,
            threads_per_object_threadgroup: MTLSize,
            threads_per_mesh_threadgroup: MTLSize,
        );

        #[method(setBarrier)]
        unsafe fn setBarrier(&self);

        #[method(clearBarrier)]
        unsafe fn clearBarrier(&self);

        #[method(reset)]
        unsafe fn reset(&self);
    }

    unsafe impl ProtocolType for dyn MTLIndirectRenderCommand {}
);

extern_protocol!(
    pub unsafe trait MTLIndirectComputeCommand: NSObjectProtocol + IsRetainable {
        #[cfg(feature = "MTLComputePipeline")]
        #[method(setComputePipelineState:)]
        unsafe fn setComputePipelineState(
            &self,
            pipeline_state: &ProtocolObject<dyn MTLComputePipelineState>,
        );

        #[cfg(all(feature = "MTLBuffer", feature = "MTLResource"))]
        #[method(setKernelBuffer:offset:atIndex:)]
        unsafe fn setKernelBuffer_offset_atIndex(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: NSUInteger,
            index: NSUInteger,
        );

        #[cfg(all(feature = "MTLBuffer", feature = "MTLResource"))]
        #[method(setKernelBuffer:offset:attributeStride:atIndex:)]
        unsafe fn setKernelBuffer_offset_attributeStride_atIndex(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: NSUInteger,
            stride: NSUInteger,
            index: NSUInteger,
        );

        #[cfg(feature = "MTLTypes")]
        #[method(concurrentDispatchThreadgroups:threadsPerThreadgroup:)]
        unsafe fn concurrentDispatchThreadgroups_threadsPerThreadgroup(
            &self,
            threadgroups_per_grid: MTLSize,
            threads_per_threadgroup: MTLSize,
        );

        #[cfg(feature = "MTLTypes")]
        #[method(concurrentDispatchThreads:threadsPerThreadgroup:)]
        unsafe fn concurrentDispatchThreads_threadsPerThreadgroup(
            &self,
            threads_per_grid: MTLSize,
            threads_per_threadgroup: MTLSize,
        );

        #[method(setBarrier)]
        unsafe fn setBarrier(&self);

        #[method(clearBarrier)]
        unsafe fn clearBarrier(&self);

        #[method(setImageblockWidth:height:)]
        unsafe fn setImageblockWidth_height(&self, width: NSUInteger, height: NSUInteger);

        #[method(reset)]
        unsafe fn reset(&self);

        #[method(setThreadgroupMemoryLength:atIndex:)]
        unsafe fn setThreadgroupMemoryLength_atIndex(&self, length: NSUInteger, index: NSUInteger);

        #[cfg(feature = "MTLTypes")]
        #[method(setStageInRegion:)]
        unsafe fn setStageInRegion(&self, region: MTLRegion);
    }

    unsafe impl ProtocolType for dyn MTLIndirectComputeCommand {}
);

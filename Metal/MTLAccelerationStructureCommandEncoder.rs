//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLAccelerationStructureRefitOptions(pub NSUInteger);
bitflags::bitflags! {
    impl MTLAccelerationStructureRefitOptions: NSUInteger {
        const MTLAccelerationStructureRefitOptionVertexData = 1<<0;
        const MTLAccelerationStructureRefitOptionPerPrimitiveData = 1<<1;
    }
}

unsafe impl Encode for MTLAccelerationStructureRefitOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLAccelerationStructureRefitOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    #[cfg(feature = "MTLCommandEncoder")]
    pub unsafe trait MTLAccelerationStructureCommandEncoder: MTLCommandEncoder {
        #[cfg(all(
            feature = "MTLAccelerationStructure",
            feature = "MTLAllocation",
            feature = "MTLBuffer",
            feature = "MTLResource"
        ))]
        #[method(buildAccelerationStructure:descriptor:scratchBuffer:scratchBufferOffset:)]
        fn buildAccelerationStructure_descriptor_scratchBuffer_scratchBufferOffset(
            &self,
            acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
            descriptor: &MTLAccelerationStructureDescriptor,
            scratch_buffer: &ProtocolObject<dyn MTLBuffer>,
            scratch_buffer_offset: NSUInteger,
        );

        #[cfg(all(
            feature = "MTLAccelerationStructure",
            feature = "MTLAllocation",
            feature = "MTLBuffer",
            feature = "MTLResource"
        ))]
        #[method(refitAccelerationStructure:descriptor:destination:scratchBuffer:scratchBufferOffset:)]
        unsafe fn refitAccelerationStructure_descriptor_destination_scratchBuffer_scratchBufferOffset(
            &self,
            source_acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
            descriptor: &MTLAccelerationStructureDescriptor,
            destination_acceleration_structure: Option<
                &ProtocolObject<dyn MTLAccelerationStructure>,
            >,
            scratch_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            scratch_buffer_offset: NSUInteger,
        );

        #[cfg(all(
            feature = "MTLAccelerationStructure",
            feature = "MTLAllocation",
            feature = "MTLBuffer",
            feature = "MTLResource"
        ))]
        #[method(refitAccelerationStructure:descriptor:destination:scratchBuffer:scratchBufferOffset:options:)]
        unsafe fn refitAccelerationStructure_descriptor_destination_scratchBuffer_scratchBufferOffset_options(
            &self,
            source_acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
            descriptor: &MTLAccelerationStructureDescriptor,
            destination_acceleration_structure: Option<
                &ProtocolObject<dyn MTLAccelerationStructure>,
            >,
            scratch_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            scratch_buffer_offset: NSUInteger,
            options: MTLAccelerationStructureRefitOptions,
        );

        #[cfg(all(
            feature = "MTLAccelerationStructure",
            feature = "MTLAllocation",
            feature = "MTLResource"
        ))]
        #[method(copyAccelerationStructure:toAccelerationStructure:)]
        unsafe fn copyAccelerationStructure_toAccelerationStructure(
            &self,
            source_acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
            destination_acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
        );

        #[cfg(all(
            feature = "MTLAccelerationStructure",
            feature = "MTLAllocation",
            feature = "MTLBuffer",
            feature = "MTLResource"
        ))]
        #[method(writeCompactedAccelerationStructureSize:toBuffer:offset:)]
        fn writeCompactedAccelerationStructureSize_toBuffer_offset(
            &self,
            acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: NSUInteger,
        );

        #[cfg(all(
            feature = "MTLAccelerationStructure",
            feature = "MTLAllocation",
            feature = "MTLArgument",
            feature = "MTLBuffer",
            feature = "MTLResource"
        ))]
        #[method(writeCompactedAccelerationStructureSize:toBuffer:offset:sizeDataType:)]
        unsafe fn writeCompactedAccelerationStructureSize_toBuffer_offset_sizeDataType(
            &self,
            acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: NSUInteger,
            size_data_type: MTLDataType,
        );

        #[cfg(all(
            feature = "MTLAccelerationStructure",
            feature = "MTLAllocation",
            feature = "MTLResource"
        ))]
        #[method(copyAndCompactAccelerationStructure:toAccelerationStructure:)]
        fn copyAndCompactAccelerationStructure_toAccelerationStructure(
            &self,
            source_acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
            destination_acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
        );

        #[cfg(feature = "MTLFence")]
        #[method(updateFence:)]
        unsafe fn updateFence(&self, fence: &ProtocolObject<dyn MTLFence>);

        #[cfg(feature = "MTLFence")]
        #[method(waitForFence:)]
        unsafe fn waitForFence(&self, fence: &ProtocolObject<dyn MTLFence>);

        #[cfg(all(feature = "MTLAllocation", feature = "MTLResource"))]
        #[method(useResource:usage:)]
        unsafe fn useResource_usage(
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
        unsafe fn useHeap(&self, heap: &ProtocolObject<dyn MTLHeap>);

        #[cfg(all(feature = "MTLAllocation", feature = "MTLHeap"))]
        #[method(useHeaps:count:)]
        unsafe fn useHeaps_count(
            &self,
            heaps: NonNull<NonNull<ProtocolObject<dyn MTLHeap>>>,
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
    unsafe impl ProtocolType for dyn MTLAccelerationStructureCommandEncoder {}
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLAccelerationStructurePassSampleBufferAttachmentDescriptor;
);

unsafe impl NSCopying for MTLAccelerationStructurePassSampleBufferAttachmentDescriptor {}

unsafe impl CopyingHelper for MTLAccelerationStructurePassSampleBufferAttachmentDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLAccelerationStructurePassSampleBufferAttachmentDescriptor {}

extern_methods!(
    unsafe impl MTLAccelerationStructurePassSampleBufferAttachmentDescriptor {
        #[cfg(feature = "MTLCounters")]
        #[method_id(@__retain_semantics Other sampleBuffer)]
        pub unsafe fn sampleBuffer(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLCounterSampleBuffer>>>;

        #[cfg(feature = "MTLCounters")]
        #[method(setSampleBuffer:)]
        pub unsafe fn setSampleBuffer(
            &self,
            sample_buffer: Option<&ProtocolObject<dyn MTLCounterSampleBuffer>>,
        );

        #[method(startOfEncoderSampleIndex)]
        pub unsafe fn startOfEncoderSampleIndex(&self) -> NSUInteger;

        #[method(setStartOfEncoderSampleIndex:)]
        pub unsafe fn setStartOfEncoderSampleIndex(
            &self,
            start_of_encoder_sample_index: NSUInteger,
        );

        #[method(endOfEncoderSampleIndex)]
        pub unsafe fn endOfEncoderSampleIndex(&self) -> NSUInteger;

        #[method(setEndOfEncoderSampleIndex:)]
        pub unsafe fn setEndOfEncoderSampleIndex(&self, end_of_encoder_sample_index: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLAccelerationStructurePassSampleBufferAttachmentDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray;
);

unsafe impl NSObjectProtocol for MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray {}

extern_methods!(
    unsafe impl MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray {
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            attachment_index: NSUInteger,
        ) -> Retained<MTLAccelerationStructurePassSampleBufferAttachmentDescriptor>;

        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            attachment: Option<&MTLAccelerationStructurePassSampleBufferAttachmentDescriptor>,
            attachment_index: NSUInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLAccelerationStructurePassDescriptor;
);

unsafe impl NSCopying for MTLAccelerationStructurePassDescriptor {}

unsafe impl CopyingHelper for MTLAccelerationStructurePassDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLAccelerationStructurePassDescriptor {}

extern_methods!(
    unsafe impl MTLAccelerationStructurePassDescriptor {
        #[method_id(@__retain_semantics Other accelerationStructurePassDescriptor)]
        pub unsafe fn accelerationStructurePassDescriptor(
        ) -> Retained<MTLAccelerationStructurePassDescriptor>;

        #[method_id(@__retain_semantics Other sampleBufferAttachments)]
        pub unsafe fn sampleBufferAttachments(
            &self,
        ) -> Retained<MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLAccelerationStructurePassDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

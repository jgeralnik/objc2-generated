//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLComputePassSampleBufferAttachmentDescriptor;
);

unsafe impl NSCopying for MTLComputePassSampleBufferAttachmentDescriptor {}

unsafe impl CopyingHelper for MTLComputePassSampleBufferAttachmentDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLComputePassSampleBufferAttachmentDescriptor {}

extern_methods!(
    unsafe impl MTLComputePassSampleBufferAttachmentDescriptor {
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
    unsafe impl MTLComputePassSampleBufferAttachmentDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptorarray?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLComputePassSampleBufferAttachmentDescriptorArray;
);

unsafe impl NSObjectProtocol for MTLComputePassSampleBufferAttachmentDescriptorArray {}

extern_methods!(
    unsafe impl MTLComputePassSampleBufferAttachmentDescriptorArray {
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            attachment_index: NSUInteger,
        ) -> Retained<MTLComputePassSampleBufferAttachmentDescriptor>;

        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            attachment: Option<&MTLComputePassSampleBufferAttachmentDescriptor>,
            attachment_index: NSUInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLComputePassSampleBufferAttachmentDescriptorArray {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcomputepassdescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLComputePassDescriptor;
);

unsafe impl NSCopying for MTLComputePassDescriptor {}

unsafe impl CopyingHelper for MTLComputePassDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLComputePassDescriptor {}

extern_methods!(
    unsafe impl MTLComputePassDescriptor {
        #[method_id(@__retain_semantics Other computePassDescriptor)]
        pub unsafe fn computePassDescriptor() -> Retained<MTLComputePassDescriptor>;

        #[cfg(feature = "MTLCommandBuffer")]
        #[method(dispatchType)]
        pub unsafe fn dispatchType(&self) -> MTLDispatchType;

        #[cfg(feature = "MTLCommandBuffer")]
        #[method(setDispatchType:)]
        pub unsafe fn setDispatchType(&self, dispatch_type: MTLDispatchType);

        #[method_id(@__retain_semantics Other sampleBufferAttachments)]
        pub unsafe fn sampleBufferAttachments(
            &self,
        ) -> Retained<MTLComputePassSampleBufferAttachmentDescriptorArray>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLComputePassDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

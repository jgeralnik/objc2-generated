//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLResourceStatePassSampleBufferAttachmentDescriptor;

    unsafe impl ClassType for MTLResourceStatePassSampleBufferAttachmentDescriptor {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl MTLResourceStatePassSampleBufferAttachmentDescriptor {
        #[method_id(@__retain_semantics Other sampleBuffer)]
        pub unsafe fn sampleBuffer(&self) -> Option<Id<MTLCounterSampleBuffer, Shared>>;

        #[method(setSampleBuffer:)]
        pub unsafe fn setSampleBuffer(&self, sampleBuffer: Option<&MTLCounterSampleBuffer>);

        #[method(startOfEncoderSampleIndex)]
        pub unsafe fn startOfEncoderSampleIndex(&self) -> NSUInteger;

        #[method(setStartOfEncoderSampleIndex:)]
        pub unsafe fn setStartOfEncoderSampleIndex(&self, startOfEncoderSampleIndex: NSUInteger);

        #[method(endOfEncoderSampleIndex)]
        pub unsafe fn endOfEncoderSampleIndex(&self) -> NSUInteger;

        #[method(setEndOfEncoderSampleIndex:)]
        pub unsafe fn setEndOfEncoderSampleIndex(&self, endOfEncoderSampleIndex: NSUInteger);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLResourceStatePassSampleBufferAttachmentDescriptorArray;

    unsafe impl ClassType for MTLResourceStatePassSampleBufferAttachmentDescriptorArray {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl MTLResourceStatePassSampleBufferAttachmentDescriptorArray {
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            attachmentIndex: NSUInteger,
        ) -> Id<MTLResourceStatePassSampleBufferAttachmentDescriptor, Shared>;

        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            attachment: Option<&MTLResourceStatePassSampleBufferAttachmentDescriptor>,
            attachmentIndex: NSUInteger,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLResourceStatePassDescriptor;

    unsafe impl ClassType for MTLResourceStatePassDescriptor {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl MTLResourceStatePassDescriptor {
        #[method_id(@__retain_semantics Other resourceStatePassDescriptor)]
        pub unsafe fn resourceStatePassDescriptor() -> Id<MTLResourceStatePassDescriptor, Shared>;

        #[method_id(@__retain_semantics Other sampleBufferAttachments)]
        pub unsafe fn sampleBufferAttachments(
            &self,
        ) -> Id<MTLResourceStatePassSampleBufferAttachmentDescriptorArray, Shared>;
    }
);

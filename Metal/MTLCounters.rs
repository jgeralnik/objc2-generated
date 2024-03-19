//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

#[cfg(feature = "Foundation_NSString")]
typed_enum!(
    pub type MTLCommonCounter = NSString;
);

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static MTLCommonCounterTimestamp: &'static MTLCommonCounter;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static MTLCommonCounterTessellationInputPatches: &'static MTLCommonCounter;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static MTLCommonCounterVertexInvocations: &'static MTLCommonCounter;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static MTLCommonCounterPostTessellationVertexInvocations: &'static MTLCommonCounter;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static MTLCommonCounterClipperInvocations: &'static MTLCommonCounter;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static MTLCommonCounterClipperPrimitivesOut: &'static MTLCommonCounter;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static MTLCommonCounterFragmentInvocations: &'static MTLCommonCounter;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static MTLCommonCounterFragmentsPassed: &'static MTLCommonCounter;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static MTLCommonCounterComputeKernelInvocations: &'static MTLCommonCounter;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static MTLCommonCounterTotalCycles: &'static MTLCommonCounter;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static MTLCommonCounterVertexCycles: &'static MTLCommonCounter;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static MTLCommonCounterTessellationCycles: &'static MTLCommonCounter;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static MTLCommonCounterPostTessellationVertexCycles: &'static MTLCommonCounter;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static MTLCommonCounterFragmentCycles: &'static MTLCommonCounter;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static MTLCommonCounterRenderTargetWriteCycles: &'static MTLCommonCounter;
}

#[cfg(feature = "Foundation_NSString")]
typed_enum!(
    pub type MTLCommonCounterSet = NSString;
);

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static MTLCommonCounterSetTimestamp: &'static MTLCommonCounterSet;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static MTLCommonCounterSetStageUtilization: &'static MTLCommonCounterSet;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static MTLCommonCounterSetStatistic: &'static MTLCommonCounterSet;
}

extern_struct!(
    #[encoding_name("?")]
    pub struct MTLCounterResultTimestamp {
        pub timestamp: u64,
    }
);

extern_struct!(
    #[encoding_name("?")]
    pub struct MTLCounterResultStageUtilization {
        pub totalCycles: u64,
        pub vertexCycles: u64,
        pub tessellationCycles: u64,
        pub postTessellationVertexCycles: u64,
        pub fragmentCycles: u64,
        pub renderTargetCycles: u64,
    }
);

extern_struct!(
    #[encoding_name("?")]
    pub struct MTLCounterResultStatistic {
        pub tessellationInputPatches: u64,
        pub vertexInvocations: u64,
        pub postTessellationVertexInvocations: u64,
        pub clipperInvocations: u64,
        pub clipperPrimitivesOut: u64,
        pub fragmentInvocations: u64,
        pub fragmentsPassed: u64,
        pub computeKernelInvocations: u64,
    }
);

extern_protocol!(
    pub unsafe trait MTLCounter: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        unsafe fn name(&self) -> Id<NSString>;
    }

    unsafe impl ProtocolType for dyn MTLCounter {}
);

extern_protocol!(
    pub unsafe trait MTLCounterSet: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other counters)]
        unsafe fn counters(&self) -> Id<NSArray<ProtocolObject<dyn MTLCounter>>>;
    }

    unsafe impl ProtocolType for dyn MTLCounterSet {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLCounterSampleBufferDescriptor;

    unsafe impl ClassType for MTLCounterSampleBufferDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for MTLCounterSampleBufferDescriptor {}

unsafe impl NSObjectProtocol for MTLCounterSampleBufferDescriptor {}

extern_methods!(
    unsafe impl MTLCounterSampleBufferDescriptor {
        #[method_id(@__retain_semantics Other counterSet)]
        pub unsafe fn counterSet(&self) -> Option<Id<ProtocolObject<dyn MTLCounterSet>>>;

        #[method(setCounterSet:)]
        pub unsafe fn setCounterSet(&self, counter_set: Option<&ProtocolObject<dyn MTLCounterSet>>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: &NSString);

        #[cfg(feature = "Metal_MTLResource")]
        #[method(storageMode)]
        pub unsafe fn storageMode(&self) -> MTLStorageMode;

        #[cfg(feature = "Metal_MTLResource")]
        #[method(setStorageMode:)]
        pub unsafe fn setStorageMode(&self, storage_mode: MTLStorageMode);

        #[method(sampleCount)]
        pub unsafe fn sampleCount(&self) -> NSUInteger;

        #[method(setSampleCount:)]
        pub unsafe fn setSampleCount(&self, sample_count: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLCounterSampleBufferDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait MTLCounterSampleBuffer: NSObjectProtocol {
        #[cfg(feature = "Metal_MTLDevice")]
        #[method_id(@__retain_semantics Other device)]
        unsafe fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        unsafe fn label(&self) -> Id<NSString>;

        #[method(sampleCount)]
        unsafe fn sampleCount(&self) -> NSUInteger;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSRange"))]
        #[method_id(@__retain_semantics Other resolveCounterRange:)]
        unsafe fn resolveCounterRange(&self, range: NSRange) -> Option<Id<NSData>>;
    }

    unsafe impl ProtocolType for dyn MTLCounterSampleBuffer {}
);

extern "C" {
    #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
    pub static MTLCounterErrorDomain: &'static NSErrorDomain;
}

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MTLCounterSampleBufferError {
        #[doc(alias = "MTLCounterSampleBufferErrorOutOfMemory")]
        OutOfMemory = 0,
        #[doc(alias = "MTLCounterSampleBufferErrorInvalid")]
        Invalid = 1,
        #[doc(alias = "MTLCounterSampleBufferErrorInternal")]
        Internal = 2,
    }
);

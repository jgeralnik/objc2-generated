//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlheaptype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLHeapType(pub NSInteger);
impl MTLHeapType {
    #[doc(alias = "MTLHeapTypeAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "MTLHeapTypePlacement")]
    pub const Placement: Self = Self(1);
    #[doc(alias = "MTLHeapTypeSparse")]
    pub const Sparse: Self = Self(2);
}

unsafe impl Encode for MTLHeapType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MTLHeapType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlheapdescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLHeapDescriptor;
);

unsafe impl NSCopying for MTLHeapDescriptor {}

unsafe impl CopyingHelper for MTLHeapDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLHeapDescriptor {}

extern_methods!(
    unsafe impl MTLHeapDescriptor {
        #[method(size)]
        pub fn size(&self) -> NSUInteger;

        #[method(setSize:)]
        pub fn setSize(&self, size: NSUInteger);

        #[cfg(feature = "MTLResource")]
        #[method(storageMode)]
        pub fn storageMode(&self) -> MTLStorageMode;

        #[cfg(feature = "MTLResource")]
        #[method(setStorageMode:)]
        pub fn setStorageMode(&self, storage_mode: MTLStorageMode);

        #[cfg(feature = "MTLResource")]
        #[method(cpuCacheMode)]
        pub fn cpuCacheMode(&self) -> MTLCPUCacheMode;

        #[cfg(feature = "MTLResource")]
        #[method(setCpuCacheMode:)]
        pub fn setCpuCacheMode(&self, cpu_cache_mode: MTLCPUCacheMode);

        #[cfg(feature = "MTLDevice")]
        #[method(sparsePageSize)]
        pub unsafe fn sparsePageSize(&self) -> MTLSparsePageSize;

        #[cfg(feature = "MTLDevice")]
        #[method(setSparsePageSize:)]
        pub unsafe fn setSparsePageSize(&self, sparse_page_size: MTLSparsePageSize);

        #[cfg(feature = "MTLResource")]
        #[method(hazardTrackingMode)]
        pub fn hazardTrackingMode(&self) -> MTLHazardTrackingMode;

        #[cfg(feature = "MTLResource")]
        #[method(setHazardTrackingMode:)]
        pub fn setHazardTrackingMode(&self, hazard_tracking_mode: MTLHazardTrackingMode);

        #[cfg(feature = "MTLResource")]
        #[method(resourceOptions)]
        pub fn resourceOptions(&self) -> MTLResourceOptions;

        #[cfg(feature = "MTLResource")]
        #[method(setResourceOptions:)]
        pub fn setResourceOptions(&self, resource_options: MTLResourceOptions);

        #[method(type)]
        pub unsafe fn r#type(&self) -> MTLHeapType;

        #[method(setType:)]
        pub fn setType(&self, r#type: MTLHeapType);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLHeapDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlheap?language=objc)
    #[cfg(feature = "MTLAllocation")]
    pub unsafe trait MTLHeap: MTLAllocation {
        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Retained<NSString>>;

        #[method(setLabel:)]
        fn setLabel(&self, label: Option<&NSString>);

        #[cfg(feature = "MTLDevice")]
        #[method_id(@__retain_semantics Other device)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        #[cfg(feature = "MTLResource")]
        #[method(storageMode)]
        fn storageMode(&self) -> MTLStorageMode;

        #[cfg(feature = "MTLResource")]
        #[method(cpuCacheMode)]
        fn cpuCacheMode(&self) -> MTLCPUCacheMode;

        #[cfg(feature = "MTLResource")]
        #[method(hazardTrackingMode)]
        fn hazardTrackingMode(&self) -> MTLHazardTrackingMode;

        #[cfg(feature = "MTLResource")]
        #[method(resourceOptions)]
        fn resourceOptions(&self) -> MTLResourceOptions;

        #[method(size)]
        fn size(&self) -> NSUInteger;

        #[method(usedSize)]
        fn usedSize(&self) -> NSUInteger;

        #[method(currentAllocatedSize)]
        fn currentAllocatedSize(&self) -> NSUInteger;

        #[method(maxAvailableSizeWithAlignment:)]
        fn maxAvailableSizeWithAlignment(&self, alignment: NSUInteger) -> NSUInteger;

        #[cfg(all(feature = "MTLBuffer", feature = "MTLResource"))]
        #[method_id(@__retain_semantics New newBufferWithLength:options:)]
        fn newBufferWithLength_options(
            &self,
            length: NSUInteger,
            options: MTLResourceOptions,
        ) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        #[cfg(all(feature = "MTLResource", feature = "MTLTexture"))]
        #[method_id(@__retain_semantics New newTextureWithDescriptor:)]
        fn newTextureWithDescriptor(
            &self,
            descriptor: &MTLTextureDescriptor,
        ) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

        #[cfg(feature = "MTLResource")]
        #[method(setPurgeableState:)]
        fn setPurgeableState(&self, state: MTLPurgeableState) -> MTLPurgeableState;

        #[method(type)]
        unsafe fn r#type(&self) -> MTLHeapType;

        #[cfg(all(feature = "MTLBuffer", feature = "MTLResource"))]
        #[method_id(@__retain_semantics New newBufferWithLength:options:offset:)]
        unsafe fn newBufferWithLength_options_offset(
            &self,
            length: NSUInteger,
            options: MTLResourceOptions,
            offset: NSUInteger,
        ) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        #[cfg(all(feature = "MTLResource", feature = "MTLTexture"))]
        #[method_id(@__retain_semantics New newTextureWithDescriptor:offset:)]
        unsafe fn newTextureWithDescriptor_offset(
            &self,
            descriptor: &MTLTextureDescriptor,
            offset: NSUInteger,
        ) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

        #[cfg(all(feature = "MTLAccelerationStructure", feature = "MTLResource"))]
        #[method_id(@__retain_semantics New newAccelerationStructureWithSize:)]
        unsafe fn newAccelerationStructureWithSize(
            &self,
            size: NSUInteger,
        ) -> Option<Retained<ProtocolObject<dyn MTLAccelerationStructure>>>;

        #[cfg(all(feature = "MTLAccelerationStructure", feature = "MTLResource"))]
        #[method_id(@__retain_semantics New newAccelerationStructureWithDescriptor:)]
        unsafe fn newAccelerationStructureWithDescriptor(
            &self,
            descriptor: &MTLAccelerationStructureDescriptor,
        ) -> Option<Retained<ProtocolObject<dyn MTLAccelerationStructure>>>;

        #[cfg(all(feature = "MTLAccelerationStructure", feature = "MTLResource"))]
        #[method_id(@__retain_semantics New newAccelerationStructureWithSize:offset:)]
        unsafe fn newAccelerationStructureWithSize_offset(
            &self,
            size: NSUInteger,
            offset: NSUInteger,
        ) -> Option<Retained<ProtocolObject<dyn MTLAccelerationStructure>>>;

        #[cfg(all(feature = "MTLAccelerationStructure", feature = "MTLResource"))]
        #[method_id(@__retain_semantics New newAccelerationStructureWithDescriptor:offset:)]
        unsafe fn newAccelerationStructureWithDescriptor_offset(
            &self,
            descriptor: &MTLAccelerationStructureDescriptor,
            offset: NSUInteger,
        ) -> Option<Retained<ProtocolObject<dyn MTLAccelerationStructure>>>;
    }

    #[cfg(feature = "MTLAllocation")]
    unsafe impl ProtocolType for dyn MTLHeap {}
);

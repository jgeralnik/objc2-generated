//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLIntersectionFunctionSignature(pub NSUInteger);
bitflags::bitflags! {
    impl MTLIntersectionFunctionSignature: NSUInteger {
        #[doc(alias = "MTLIntersectionFunctionSignatureNone")]
        const None = 0;
        #[doc(alias = "MTLIntersectionFunctionSignatureInstancing")]
        const Instancing = 1<<0;
        #[doc(alias = "MTLIntersectionFunctionSignatureTriangleData")]
        const TriangleData = 1<<1;
        #[doc(alias = "MTLIntersectionFunctionSignatureWorldSpaceData")]
        const WorldSpaceData = 1<<2;
        #[doc(alias = "MTLIntersectionFunctionSignatureInstanceMotion")]
        const InstanceMotion = 1<<3;
        #[doc(alias = "MTLIntersectionFunctionSignaturePrimitiveMotion")]
        const PrimitiveMotion = 1<<4;
        #[doc(alias = "MTLIntersectionFunctionSignatureExtendedLimits")]
        const ExtendedLimits = 1<<5;
        #[doc(alias = "MTLIntersectionFunctionSignatureMaxLevels")]
        const MaxLevels = 1<<6;
        #[doc(alias = "MTLIntersectionFunctionSignatureCurveData")]
        const CurveData = 1<<7;
    }
}

unsafe impl Encode for MTLIntersectionFunctionSignature {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLIntersectionFunctionSignature {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLIntersectionFunctionTableDescriptor;

    unsafe impl ClassType for MTLIntersectionFunctionTableDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for MTLIntersectionFunctionTableDescriptor {}

unsafe impl NSObjectProtocol for MTLIntersectionFunctionTableDescriptor {}

extern_methods!(
    unsafe impl MTLIntersectionFunctionTableDescriptor {
        #[method_id(@__retain_semantics Other intersectionFunctionTableDescriptor)]
        pub unsafe fn intersectionFunctionTableDescriptor(
        ) -> Retained<MTLIntersectionFunctionTableDescriptor>;

        #[method(functionCount)]
        pub unsafe fn functionCount(&self) -> NSUInteger;

        #[method(setFunctionCount:)]
        pub fn setFunctionCount(&self, function_count: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLIntersectionFunctionTableDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Retained<Self>;
    }
);

impl DefaultRetained for MTLIntersectionFunctionTableDescriptor {
    #[inline]
    fn default_id() -> Retained<Self> {
        Self::new()
    }
}

extern_protocol!(
    #[cfg(feature = "MTLResource")]
    pub unsafe trait MTLIntersectionFunctionTable: MTLResource {
        #[cfg(feature = "MTLBuffer")]
        #[method(setBuffer:offset:atIndex:)]
        unsafe fn setBuffer_offset_atIndex(
            &self,
            buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            offset: NSUInteger,
            index: NSUInteger,
        );

        #[cfg(feature = "MTLBuffer")]
        #[method(setBuffers:offsets:withRange:)]
        unsafe fn setBuffers_offsets_withRange(
            &self,
            buffers: NonNull<*const ProtocolObject<dyn MTLBuffer>>,
            offsets: NonNull<NSUInteger>,
            range: NSRange,
        );

        #[cfg(feature = "MTLTypes")]
        #[method(gpuResourceID)]
        unsafe fn gpuResourceID(&self) -> MTLResourceID;

        #[cfg(feature = "MTLFunctionHandle")]
        #[method(setFunction:atIndex:)]
        fn setFunction_atIndex(
            &self,
            function: Option<&ProtocolObject<dyn MTLFunctionHandle>>,
            index: NSUInteger,
        );

        #[cfg(feature = "MTLFunctionHandle")]
        #[method(setFunctions:withRange:)]
        unsafe fn setFunctions_withRange(
            &self,
            functions: NonNull<*const ProtocolObject<dyn MTLFunctionHandle>>,
            range: NSRange,
        );

        #[method(setOpaqueTriangleIntersectionFunctionWithSignature:atIndex:)]
        unsafe fn setOpaqueTriangleIntersectionFunctionWithSignature_atIndex(
            &self,
            signature: MTLIntersectionFunctionSignature,
            index: NSUInteger,
        );

        #[method(setOpaqueTriangleIntersectionFunctionWithSignature:withRange:)]
        unsafe fn setOpaqueTriangleIntersectionFunctionWithSignature_withRange(
            &self,
            signature: MTLIntersectionFunctionSignature,
            range: NSRange,
        );

        #[method(setOpaqueCurveIntersectionFunctionWithSignature:atIndex:)]
        unsafe fn setOpaqueCurveIntersectionFunctionWithSignature_atIndex(
            &self,
            signature: MTLIntersectionFunctionSignature,
            index: NSUInteger,
        );

        #[method(setOpaqueCurveIntersectionFunctionWithSignature:withRange:)]
        unsafe fn setOpaqueCurveIntersectionFunctionWithSignature_withRange(
            &self,
            signature: MTLIntersectionFunctionSignature,
            range: NSRange,
        );

        #[cfg(feature = "MTLVisibleFunctionTable")]
        #[method(setVisibleFunctionTable:atBufferIndex:)]
        unsafe fn setVisibleFunctionTable_atBufferIndex(
            &self,
            function_table: Option<&ProtocolObject<dyn MTLVisibleFunctionTable>>,
            buffer_index: NSUInteger,
        );

        #[cfg(feature = "MTLVisibleFunctionTable")]
        #[method(setVisibleFunctionTables:withBufferRange:)]
        unsafe fn setVisibleFunctionTables_withBufferRange(
            &self,
            function_tables: NonNull<*const ProtocolObject<dyn MTLVisibleFunctionTable>>,
            buffer_range: NSRange,
        );
    }

    #[cfg(feature = "MTLResource")]
    unsafe impl ProtocolType for dyn MTLIntersectionFunctionTable {}
);

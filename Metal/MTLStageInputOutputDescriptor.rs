//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLAttributeFormat(pub NSUInteger);
impl MTLAttributeFormat {
    #[doc(alias = "MTLAttributeFormatInvalid")]
    pub const Invalid: Self = Self(0);
    #[doc(alias = "MTLAttributeFormatUChar2")]
    pub const UChar2: Self = Self(1);
    #[doc(alias = "MTLAttributeFormatUChar3")]
    pub const UChar3: Self = Self(2);
    #[doc(alias = "MTLAttributeFormatUChar4")]
    pub const UChar4: Self = Self(3);
    #[doc(alias = "MTLAttributeFormatChar2")]
    pub const Char2: Self = Self(4);
    #[doc(alias = "MTLAttributeFormatChar3")]
    pub const Char3: Self = Self(5);
    #[doc(alias = "MTLAttributeFormatChar4")]
    pub const Char4: Self = Self(6);
    #[doc(alias = "MTLAttributeFormatUChar2Normalized")]
    pub const UChar2Normalized: Self = Self(7);
    #[doc(alias = "MTLAttributeFormatUChar3Normalized")]
    pub const UChar3Normalized: Self = Self(8);
    #[doc(alias = "MTLAttributeFormatUChar4Normalized")]
    pub const UChar4Normalized: Self = Self(9);
    #[doc(alias = "MTLAttributeFormatChar2Normalized")]
    pub const Char2Normalized: Self = Self(10);
    #[doc(alias = "MTLAttributeFormatChar3Normalized")]
    pub const Char3Normalized: Self = Self(11);
    #[doc(alias = "MTLAttributeFormatChar4Normalized")]
    pub const Char4Normalized: Self = Self(12);
    #[doc(alias = "MTLAttributeFormatUShort2")]
    pub const UShort2: Self = Self(13);
    #[doc(alias = "MTLAttributeFormatUShort3")]
    pub const UShort3: Self = Self(14);
    #[doc(alias = "MTLAttributeFormatUShort4")]
    pub const UShort4: Self = Self(15);
    #[doc(alias = "MTLAttributeFormatShort2")]
    pub const Short2: Self = Self(16);
    #[doc(alias = "MTLAttributeFormatShort3")]
    pub const Short3: Self = Self(17);
    #[doc(alias = "MTLAttributeFormatShort4")]
    pub const Short4: Self = Self(18);
    #[doc(alias = "MTLAttributeFormatUShort2Normalized")]
    pub const UShort2Normalized: Self = Self(19);
    #[doc(alias = "MTLAttributeFormatUShort3Normalized")]
    pub const UShort3Normalized: Self = Self(20);
    #[doc(alias = "MTLAttributeFormatUShort4Normalized")]
    pub const UShort4Normalized: Self = Self(21);
    #[doc(alias = "MTLAttributeFormatShort2Normalized")]
    pub const Short2Normalized: Self = Self(22);
    #[doc(alias = "MTLAttributeFormatShort3Normalized")]
    pub const Short3Normalized: Self = Self(23);
    #[doc(alias = "MTLAttributeFormatShort4Normalized")]
    pub const Short4Normalized: Self = Self(24);
    #[doc(alias = "MTLAttributeFormatHalf2")]
    pub const Half2: Self = Self(25);
    #[doc(alias = "MTLAttributeFormatHalf3")]
    pub const Half3: Self = Self(26);
    #[doc(alias = "MTLAttributeFormatHalf4")]
    pub const Half4: Self = Self(27);
    #[doc(alias = "MTLAttributeFormatFloat")]
    pub const Float: Self = Self(28);
    #[doc(alias = "MTLAttributeFormatFloat2")]
    pub const Float2: Self = Self(29);
    #[doc(alias = "MTLAttributeFormatFloat3")]
    pub const Float3: Self = Self(30);
    #[doc(alias = "MTLAttributeFormatFloat4")]
    pub const Float4: Self = Self(31);
    #[doc(alias = "MTLAttributeFormatInt")]
    pub const Int: Self = Self(32);
    #[doc(alias = "MTLAttributeFormatInt2")]
    pub const Int2: Self = Self(33);
    #[doc(alias = "MTLAttributeFormatInt3")]
    pub const Int3: Self = Self(34);
    #[doc(alias = "MTLAttributeFormatInt4")]
    pub const Int4: Self = Self(35);
    #[doc(alias = "MTLAttributeFormatUInt")]
    pub const UInt: Self = Self(36);
    #[doc(alias = "MTLAttributeFormatUInt2")]
    pub const UInt2: Self = Self(37);
    #[doc(alias = "MTLAttributeFormatUInt3")]
    pub const UInt3: Self = Self(38);
    #[doc(alias = "MTLAttributeFormatUInt4")]
    pub const UInt4: Self = Self(39);
    #[doc(alias = "MTLAttributeFormatInt1010102Normalized")]
    pub const Int1010102Normalized: Self = Self(40);
    #[doc(alias = "MTLAttributeFormatUInt1010102Normalized")]
    pub const UInt1010102Normalized: Self = Self(41);
    #[doc(alias = "MTLAttributeFormatUChar4Normalized_BGRA")]
    pub const UChar4Normalized_BGRA: Self = Self(42);
    #[doc(alias = "MTLAttributeFormatUChar")]
    pub const UChar: Self = Self(45);
    #[doc(alias = "MTLAttributeFormatChar")]
    pub const Char: Self = Self(46);
    #[doc(alias = "MTLAttributeFormatUCharNormalized")]
    pub const UCharNormalized: Self = Self(47);
    #[doc(alias = "MTLAttributeFormatCharNormalized")]
    pub const CharNormalized: Self = Self(48);
    #[doc(alias = "MTLAttributeFormatUShort")]
    pub const UShort: Self = Self(49);
    #[doc(alias = "MTLAttributeFormatShort")]
    pub const Short: Self = Self(50);
    #[doc(alias = "MTLAttributeFormatUShortNormalized")]
    pub const UShortNormalized: Self = Self(51);
    #[doc(alias = "MTLAttributeFormatShortNormalized")]
    pub const ShortNormalized: Self = Self(52);
    #[doc(alias = "MTLAttributeFormatHalf")]
    pub const Half: Self = Self(53);
    #[doc(alias = "MTLAttributeFormatFloatRG11B10")]
    pub const FloatRG11B10: Self = Self(54);
    #[doc(alias = "MTLAttributeFormatFloatRGB9E5")]
    pub const FloatRGB9E5: Self = Self(55);
}

unsafe impl Encode for MTLAttributeFormat {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLAttributeFormat {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLIndexType(pub NSUInteger);
impl MTLIndexType {
    #[doc(alias = "MTLIndexTypeUInt16")]
    pub const UInt16: Self = Self(0);
    #[doc(alias = "MTLIndexTypeUInt32")]
    pub const UInt32: Self = Self(1);
}

unsafe impl Encode for MTLIndexType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLIndexType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLStepFunction(pub NSUInteger);
impl MTLStepFunction {
    #[doc(alias = "MTLStepFunctionConstant")]
    pub const Constant: Self = Self(0);
    #[doc(alias = "MTLStepFunctionPerVertex")]
    pub const PerVertex: Self = Self(1);
    #[doc(alias = "MTLStepFunctionPerInstance")]
    pub const PerInstance: Self = Self(2);
    #[doc(alias = "MTLStepFunctionPerPatch")]
    pub const PerPatch: Self = Self(3);
    #[doc(alias = "MTLStepFunctionPerPatchControlPoint")]
    pub const PerPatchControlPoint: Self = Self(4);
    #[doc(alias = "MTLStepFunctionThreadPositionInGridX")]
    pub const ThreadPositionInGridX: Self = Self(5);
    #[doc(alias = "MTLStepFunctionThreadPositionInGridY")]
    pub const ThreadPositionInGridY: Self = Self(6);
    #[doc(alias = "MTLStepFunctionThreadPositionInGridXIndexed")]
    pub const ThreadPositionInGridXIndexed: Self = Self(7);
    #[doc(alias = "MTLStepFunctionThreadPositionInGridYIndexed")]
    pub const ThreadPositionInGridYIndexed: Self = Self(8);
}

unsafe impl Encode for MTLStepFunction {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLStepFunction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLBufferLayoutDescriptor;

    unsafe impl ClassType for MTLBufferLayoutDescriptor {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for MTLBufferLayoutDescriptor {}

unsafe impl CopyingHelper for MTLBufferLayoutDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLBufferLayoutDescriptor {}

extern_methods!(
    unsafe impl MTLBufferLayoutDescriptor {
        #[method(stride)]
        pub fn stride(&self) -> NSUInteger;

        #[method(setStride:)]
        pub fn setStride(&self, stride: NSUInteger);

        #[method(stepFunction)]
        pub fn stepFunction(&self) -> MTLStepFunction;

        #[method(setStepFunction:)]
        pub fn setStepFunction(&self, step_function: MTLStepFunction);

        #[method(stepRate)]
        pub fn stepRate(&self) -> NSUInteger;

        #[method(setStepRate:)]
        pub fn setStepRate(&self, step_rate: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLBufferLayoutDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLBufferLayoutDescriptorArray;

    unsafe impl ClassType for MTLBufferLayoutDescriptorArray {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for MTLBufferLayoutDescriptorArray {}

extern_methods!(
    unsafe impl MTLBufferLayoutDescriptorArray {
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            index: NSUInteger,
        ) -> Retained<MTLBufferLayoutDescriptor>;

        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            buffer_desc: Option<&MTLBufferLayoutDescriptor>,
            index: NSUInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLBufferLayoutDescriptorArray {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLAttributeDescriptor;

    unsafe impl ClassType for MTLAttributeDescriptor {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for MTLAttributeDescriptor {}

unsafe impl CopyingHelper for MTLAttributeDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLAttributeDescriptor {}

extern_methods!(
    unsafe impl MTLAttributeDescriptor {
        #[method(format)]
        pub fn format(&self) -> MTLAttributeFormat;

        #[method(setFormat:)]
        pub fn setFormat(&self, format: MTLAttributeFormat);

        #[method(offset)]
        pub fn offset(&self) -> NSUInteger;

        #[method(setOffset:)]
        pub fn setOffset(&self, offset: NSUInteger);

        #[method(bufferIndex)]
        pub fn bufferIndex(&self) -> NSUInteger;

        #[method(setBufferIndex:)]
        pub unsafe fn setBufferIndex(&self, buffer_index: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLAttributeDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLAttributeDescriptorArray;

    unsafe impl ClassType for MTLAttributeDescriptorArray {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for MTLAttributeDescriptorArray {}

extern_methods!(
    unsafe impl MTLAttributeDescriptorArray {
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            index: NSUInteger,
        ) -> Retained<MTLAttributeDescriptor>;

        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            attribute_desc: Option<&MTLAttributeDescriptor>,
            index: NSUInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLAttributeDescriptorArray {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLStageInputOutputDescriptor;

    unsafe impl ClassType for MTLStageInputOutputDescriptor {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for MTLStageInputOutputDescriptor {}

unsafe impl CopyingHelper for MTLStageInputOutputDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLStageInputOutputDescriptor {}

extern_methods!(
    unsafe impl MTLStageInputOutputDescriptor {
        #[method_id(@__retain_semantics Other stageInputOutputDescriptor)]
        pub fn stageInputOutputDescriptor() -> Retained<MTLStageInputOutputDescriptor>;

        #[method_id(@__retain_semantics Other layouts)]
        pub fn layouts(&self) -> Retained<MTLBufferLayoutDescriptorArray>;

        #[method_id(@__retain_semantics Other attributes)]
        pub fn attributes(&self) -> Retained<MTLAttributeDescriptorArray>;

        #[method(indexType)]
        pub fn indexType(&self) -> MTLIndexType;

        #[method(setIndexType:)]
        pub unsafe fn setIndexType(&self, index_type: MTLIndexType);

        #[method(indexBufferIndex)]
        pub fn indexBufferIndex(&self) -> NSUInteger;

        #[method(setIndexBufferIndex:)]
        pub unsafe fn setIndexBufferIndex(&self, index_buffer_index: NSUInteger);

        #[method(reset)]
        pub fn reset(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLStageInputOutputDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

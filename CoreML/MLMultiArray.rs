//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MLMultiArrayDataType(pub NSInteger);
impl MLMultiArrayDataType {
    #[doc(alias = "MLMultiArrayDataTypeDouble")]
    pub const Double: Self = Self(0x10000 | 64);
    #[doc(alias = "MLMultiArrayDataTypeFloat64")]
    pub const Float64: Self = Self(0x10000 | 64);
    #[doc(alias = "MLMultiArrayDataTypeFloat32")]
    pub const Float32: Self = Self(0x10000 | 32);
    #[doc(alias = "MLMultiArrayDataTypeFloat16")]
    pub const Float16: Self = Self(0x10000 | 16);
    #[doc(alias = "MLMultiArrayDataTypeFloat")]
    pub const Float: Self = Self(0x10000 | 32);
    #[doc(alias = "MLMultiArrayDataTypeInt32")]
    pub const Int32: Self = Self(0x20000 | 32);
}

unsafe impl Encode for MLMultiArrayDataType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MLMultiArrayDataType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLMultiArray;

    unsafe impl ClassType for MLMultiArray {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for MLMultiArray {}

unsafe impl NSObjectProtocol for MLMultiArray {}

unsafe impl NSSecureCoding for MLMultiArray {}

extern_methods!(
    unsafe impl MLMultiArray {
        #[deprecated = "Use getBytesWithHandler or getMutableBytesWithHandler instead. For Swift, use withUnsafeBytes or withUnsafeMutableBytes."]
        #[method(dataPointer)]
        pub unsafe fn dataPointer(&self) -> NonNull<c_void>;

        #[method(dataType)]
        pub unsafe fn dataType(&self) -> MLMultiArrayDataType;

        #[method_id(@__retain_semantics Other shape)]
        pub unsafe fn shape(&self) -> Retained<NSArray<NSNumber>>;

        #[method_id(@__retain_semantics Other strides)]
        pub unsafe fn strides(&self) -> Retained<NSArray<NSNumber>>;

        #[method(count)]
        pub unsafe fn count(&self) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MLMultiArray {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// Creation
    unsafe impl MLMultiArray {
        #[method_id(@__retain_semantics Init initWithShape:dataType:error:_)]
        pub unsafe fn initWithShape_dataType_error(
            this: Allocated<Self>,
            shape: &NSArray<NSNumber>,
            data_type: MLMultiArrayDataType,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithDataPointer:shape:dataType:strides:deallocator:error:_)]
        pub unsafe fn initWithDataPointer_shape_dataType_strides_deallocator_error(
            this: Allocated<Self>,
            data_pointer: NonNull<c_void>,
            shape: &NSArray<NSNumber>,
            data_type: MLMultiArrayDataType,
            strides: &NSArray<NSNumber>,
            deallocator: Option<&block2::Block<dyn Fn(NonNull<c_void>)>>,
        ) -> Result<Retained<Self>, Retained<NSError>>;
    }
);

extern_methods!(
    /// ScopedBufferAccess
    unsafe impl MLMultiArray {
        #[cfg(feature = "block2")]
        #[method(getBytesWithHandler:)]
        pub unsafe fn getBytesWithHandler(
            &self,
            handler: &block2::Block<dyn Fn(NonNull<c_void>, NSInteger) + '_>,
        );

        #[cfg(feature = "block2")]
        #[method(getMutableBytesWithHandler:)]
        pub unsafe fn getMutableBytesWithHandler(
            &self,
            handler: &block2::Block<
                dyn Fn(NonNull<c_void>, NSInteger, NonNull<NSArray<NSNumber>>) + '_,
            >,
        );
    }
);

extern_methods!(
    /// Concatenating
    unsafe impl MLMultiArray {
        #[method_id(@__retain_semantics Other multiArrayByConcatenatingMultiArrays:alongAxis:dataType:)]
        pub unsafe fn multiArrayByConcatenatingMultiArrays_alongAxis_dataType(
            multi_arrays: &NSArray<MLMultiArray>,
            axis: NSInteger,
            data_type: MLMultiArrayDataType,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// NSNumberDataAccess
    unsafe impl MLMultiArray {
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(&self, idx: NSInteger) -> Retained<NSNumber>;

        #[method_id(@__retain_semantics Other objectForKeyedSubscript:)]
        pub unsafe fn objectForKeyedSubscript(&self, key: &NSArray<NSNumber>)
            -> Retained<NSNumber>;

        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(&self, obj: &NSNumber, idx: NSInteger);

        #[method(setObject:forKeyedSubscript:)]
        pub unsafe fn setObject_forKeyedSubscript(&self, obj: &NSNumber, key: &NSArray<NSNumber>);
    }
);

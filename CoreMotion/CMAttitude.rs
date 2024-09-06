//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CMRotationMatrix {
    pub m11: c_double,
    pub m12: c_double,
    pub m13: c_double,
    pub m21: c_double,
    pub m22: c_double,
    pub m23: c_double,
    pub m31: c_double,
    pub m32: c_double,
    pub m33: c_double,
}

unsafe impl Encode for CMRotationMatrix {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <c_double>::ENCODING,
            <c_double>::ENCODING,
            <c_double>::ENCODING,
            <c_double>::ENCODING,
            <c_double>::ENCODING,
            <c_double>::ENCODING,
            <c_double>::ENCODING,
            <c_double>::ENCODING,
            <c_double>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for CMRotationMatrix {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CMQuaternion {
    pub x: c_double,
    pub y: c_double,
    pub z: c_double,
    pub w: c_double,
}

unsafe impl Encode for CMQuaternion {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <c_double>::ENCODING,
            <c_double>::ENCODING,
            <c_double>::ENCODING,
            <c_double>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for CMQuaternion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CMAttitudeReferenceFrame(pub NSUInteger);
bitflags::bitflags! {
    impl CMAttitudeReferenceFrame: NSUInteger {
        #[doc(alias = "CMAttitudeReferenceFrameXArbitraryZVertical")]
        const XArbitraryZVertical = 1<<0;
        #[doc(alias = "CMAttitudeReferenceFrameXArbitraryCorrectedZVertical")]
        const XArbitraryCorrectedZVertical = 1<<1;
        #[doc(alias = "CMAttitudeReferenceFrameXMagneticNorthZVertical")]
        const XMagneticNorthZVertical = 1<<2;
        #[doc(alias = "CMAttitudeReferenceFrameXTrueNorthZVertical")]
        const XTrueNorthZVertical = 1<<3;
    }
}

unsafe impl Encode for CMAttitudeReferenceFrame {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for CMAttitudeReferenceFrame {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CMAttitude;

    unsafe impl ClassType for CMAttitude {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for CMAttitude {}

unsafe impl NSCopying for CMAttitude {}

unsafe impl CopyingHelper for CMAttitude {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CMAttitude {}

unsafe impl NSSecureCoding for CMAttitude {}

extern_methods!(
    unsafe impl CMAttitude {
        #[method(roll)]
        pub unsafe fn roll(&self) -> c_double;

        #[method(pitch)]
        pub unsafe fn pitch(&self) -> c_double;

        #[method(yaw)]
        pub unsafe fn yaw(&self) -> c_double;

        #[method(rotationMatrix)]
        pub unsafe fn rotationMatrix(&self) -> CMRotationMatrix;

        #[method(quaternion)]
        pub unsafe fn quaternion(&self) -> CMQuaternion;

        #[method(multiplyByInverseOfAttitude:)]
        pub unsafe fn multiplyByInverseOfAttitude(&self, attitude: &CMAttitude);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CMAttitude {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

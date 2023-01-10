//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::Foundation::*;

extern_struct!(
    pub struct CATransform3D {
        pub m11: CGFloat,
        pub m12: CGFloat,
        pub m13: CGFloat,
        pub m14: CGFloat,
        pub m21: CGFloat,
        pub m22: CGFloat,
        pub m23: CGFloat,
        pub m24: CGFloat,
        pub m31: CGFloat,
        pub m32: CGFloat,
        pub m33: CGFloat,
        pub m34: CGFloat,
        pub m41: CGFloat,
        pub m42: CGFloat,
        pub m43: CGFloat,
        pub m44: CGFloat,
    }
);

extern_static!(CATransform3DIdentity: CATransform3D);

extern_fn!(
    pub fn CATransform3DIsIdentity(t: CATransform3D) -> bool;
);

extern_fn!(
    pub fn CATransform3DEqualToTransform(a: CATransform3D, b: CATransform3D) -> bool;
);

extern_fn!(
    pub fn CATransform3DMakeTranslation(tx: CGFloat, ty: CGFloat, tz: CGFloat) -> CATransform3D;
);

extern_fn!(
    pub fn CATransform3DMakeScale(sx: CGFloat, sy: CGFloat, sz: CGFloat) -> CATransform3D;
);

extern_fn!(
    pub fn CATransform3DMakeRotation(
        angle: CGFloat,
        x: CGFloat,
        y: CGFloat,
        z: CGFloat,
    ) -> CATransform3D;
);

extern_fn!(
    pub fn CATransform3DTranslate(
        t: CATransform3D,
        tx: CGFloat,
        ty: CGFloat,
        tz: CGFloat,
    ) -> CATransform3D;
);

extern_fn!(
    pub fn CATransform3DScale(
        t: CATransform3D,
        sx: CGFloat,
        sy: CGFloat,
        sz: CGFloat,
    ) -> CATransform3D;
);

extern_fn!(
    pub fn CATransform3DRotate(
        t: CATransform3D,
        angle: CGFloat,
        x: CGFloat,
        y: CGFloat,
        z: CGFloat,
    ) -> CATransform3D;
);

extern_fn!(
    pub fn CATransform3DConcat(a: CATransform3D, b: CATransform3D) -> CATransform3D;
);

extern_fn!(
    pub fn CATransform3DInvert(t: CATransform3D) -> CATransform3D;
);

extern_fn!(
    pub fn CATransform3DIsAffine(t: CATransform3D) -> bool;
);

extern_methods!(
    /// CATransform3DAdditions
    #[cfg(feature = "QuartzCore_NSValue")]
    unsafe impl NSValue {
        #[method_id(@__retain_semantics Other valueWithCATransform3D:)]
        pub unsafe fn valueWithCATransform3D(t: CATransform3D) -> Id<NSValue, Shared>;

        #[method(CATransform3DValue)]
        pub unsafe fn CATransform3DValue(&self) -> CATransform3D;
    }
);

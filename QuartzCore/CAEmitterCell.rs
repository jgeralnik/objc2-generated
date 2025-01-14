//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/caemittercell?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAEmitterCell;
);

#[cfg(feature = "CAMediaTiming")]
unsafe impl CAMediaTiming for CAEmitterCell {}

unsafe impl NSCoding for CAEmitterCell {}

unsafe impl NSObjectProtocol for CAEmitterCell {}

unsafe impl NSSecureCoding for CAEmitterCell {}

extern_methods!(
    unsafe impl CAEmitterCell {
        #[method_id(@__retain_semantics Other emitterCell)]
        pub unsafe fn emitterCell() -> Retained<Self>;

        #[method_id(@__retain_semantics Other defaultValueForKey:)]
        pub unsafe fn defaultValueForKey(key: &NSString) -> Option<Retained<AnyObject>>;

        #[method(shouldArchiveValueForKey:)]
        pub unsafe fn shouldArchiveValueForKey(&self, key: &NSString) -> bool;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Retained<NSString>>;

        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(birthRate)]
        pub unsafe fn birthRate(&self) -> c_float;

        #[method(setBirthRate:)]
        pub unsafe fn setBirthRate(&self, birth_rate: c_float);

        #[method(lifetime)]
        pub unsafe fn lifetime(&self) -> c_float;

        #[method(setLifetime:)]
        pub unsafe fn setLifetime(&self, lifetime: c_float);

        #[method(lifetimeRange)]
        pub unsafe fn lifetimeRange(&self) -> c_float;

        #[method(setLifetimeRange:)]
        pub unsafe fn setLifetimeRange(&self, lifetime_range: c_float);

        #[method(emissionLatitude)]
        pub unsafe fn emissionLatitude(&self) -> CGFloat;

        #[method(setEmissionLatitude:)]
        pub unsafe fn setEmissionLatitude(&self, emission_latitude: CGFloat);

        #[method(emissionLongitude)]
        pub unsafe fn emissionLongitude(&self) -> CGFloat;

        #[method(setEmissionLongitude:)]
        pub unsafe fn setEmissionLongitude(&self, emission_longitude: CGFloat);

        #[method(emissionRange)]
        pub unsafe fn emissionRange(&self) -> CGFloat;

        #[method(setEmissionRange:)]
        pub unsafe fn setEmissionRange(&self, emission_range: CGFloat);

        #[method(velocity)]
        pub unsafe fn velocity(&self) -> CGFloat;

        #[method(setVelocity:)]
        pub unsafe fn setVelocity(&self, velocity: CGFloat);

        #[method(velocityRange)]
        pub unsafe fn velocityRange(&self) -> CGFloat;

        #[method(setVelocityRange:)]
        pub unsafe fn setVelocityRange(&self, velocity_range: CGFloat);

        #[method(xAcceleration)]
        pub unsafe fn xAcceleration(&self) -> CGFloat;

        #[method(setXAcceleration:)]
        pub unsafe fn setXAcceleration(&self, x_acceleration: CGFloat);

        #[method(yAcceleration)]
        pub unsafe fn yAcceleration(&self) -> CGFloat;

        #[method(setYAcceleration:)]
        pub unsafe fn setYAcceleration(&self, y_acceleration: CGFloat);

        #[method(zAcceleration)]
        pub unsafe fn zAcceleration(&self) -> CGFloat;

        #[method(setZAcceleration:)]
        pub unsafe fn setZAcceleration(&self, z_acceleration: CGFloat);

        #[method(scale)]
        pub unsafe fn scale(&self) -> CGFloat;

        #[method(setScale:)]
        pub unsafe fn setScale(&self, scale: CGFloat);

        #[method(scaleRange)]
        pub unsafe fn scaleRange(&self) -> CGFloat;

        #[method(setScaleRange:)]
        pub unsafe fn setScaleRange(&self, scale_range: CGFloat);

        #[method(scaleSpeed)]
        pub unsafe fn scaleSpeed(&self) -> CGFloat;

        #[method(setScaleSpeed:)]
        pub unsafe fn setScaleSpeed(&self, scale_speed: CGFloat);

        #[method(spin)]
        pub unsafe fn spin(&self) -> CGFloat;

        #[method(setSpin:)]
        pub unsafe fn setSpin(&self, spin: CGFloat);

        #[method(spinRange)]
        pub unsafe fn spinRange(&self) -> CGFloat;

        #[method(setSpinRange:)]
        pub unsafe fn setSpinRange(&self, spin_range: CGFloat);

        #[method(redRange)]
        pub unsafe fn redRange(&self) -> c_float;

        #[method(setRedRange:)]
        pub unsafe fn setRedRange(&self, red_range: c_float);

        #[method(greenRange)]
        pub unsafe fn greenRange(&self) -> c_float;

        #[method(setGreenRange:)]
        pub unsafe fn setGreenRange(&self, green_range: c_float);

        #[method(blueRange)]
        pub unsafe fn blueRange(&self) -> c_float;

        #[method(setBlueRange:)]
        pub unsafe fn setBlueRange(&self, blue_range: c_float);

        #[method(alphaRange)]
        pub unsafe fn alphaRange(&self) -> c_float;

        #[method(setAlphaRange:)]
        pub unsafe fn setAlphaRange(&self, alpha_range: c_float);

        #[method(redSpeed)]
        pub unsafe fn redSpeed(&self) -> c_float;

        #[method(setRedSpeed:)]
        pub unsafe fn setRedSpeed(&self, red_speed: c_float);

        #[method(greenSpeed)]
        pub unsafe fn greenSpeed(&self) -> c_float;

        #[method(setGreenSpeed:)]
        pub unsafe fn setGreenSpeed(&self, green_speed: c_float);

        #[method(blueSpeed)]
        pub unsafe fn blueSpeed(&self) -> c_float;

        #[method(setBlueSpeed:)]
        pub unsafe fn setBlueSpeed(&self, blue_speed: c_float);

        #[method(alphaSpeed)]
        pub unsafe fn alphaSpeed(&self) -> c_float;

        #[method(setAlphaSpeed:)]
        pub unsafe fn setAlphaSpeed(&self, alpha_speed: c_float);

        #[method_id(@__retain_semantics Other contents)]
        pub unsafe fn contents(&self) -> Option<Retained<AnyObject>>;

        #[method(setContents:)]
        pub unsafe fn setContents(&self, contents: Option<&AnyObject>);

        #[method(contentsRect)]
        pub unsafe fn contentsRect(&self) -> CGRect;

        #[method(setContentsRect:)]
        pub unsafe fn setContentsRect(&self, contents_rect: CGRect);

        #[method(contentsScale)]
        pub unsafe fn contentsScale(&self) -> CGFloat;

        #[method(setContentsScale:)]
        pub unsafe fn setContentsScale(&self, contents_scale: CGFloat);

        #[method_id(@__retain_semantics Other minificationFilter)]
        pub unsafe fn minificationFilter(&self) -> Retained<NSString>;

        #[method(setMinificationFilter:)]
        pub unsafe fn setMinificationFilter(&self, minification_filter: &NSString);

        #[method_id(@__retain_semantics Other magnificationFilter)]
        pub unsafe fn magnificationFilter(&self) -> Retained<NSString>;

        #[method(setMagnificationFilter:)]
        pub unsafe fn setMagnificationFilter(&self, magnification_filter: &NSString);

        #[method(minificationFilterBias)]
        pub unsafe fn minificationFilterBias(&self) -> c_float;

        #[method(setMinificationFilterBias:)]
        pub unsafe fn setMinificationFilterBias(&self, minification_filter_bias: c_float);

        #[method_id(@__retain_semantics Other emitterCells)]
        pub unsafe fn emitterCells(&self) -> Option<Retained<NSArray<CAEmitterCell>>>;

        #[method(setEmitterCells:)]
        pub unsafe fn setEmitterCells(&self, emitter_cells: Option<&NSArray<CAEmitterCell>>);

        #[method_id(@__retain_semantics Other style)]
        pub unsafe fn style(&self) -> Option<Retained<NSDictionary>>;

        #[method(setStyle:)]
        pub unsafe fn setStyle(&self, style: Option<&NSDictionary>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CAEmitterCell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

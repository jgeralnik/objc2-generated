//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GCRacingWheelInputState;
);

#[cfg(feature = "GCDevicePhysicalInputState")]
unsafe impl GCDevicePhysicalInputState for GCRacingWheelInputState {}

unsafe impl NSObjectProtocol for GCRacingWheelInputState {}

extern_methods!(
    unsafe impl GCRacingWheelInputState {
        #[cfg(feature = "GCSteeringWheelElement")]
        #[method_id(@__retain_semantics Other wheel)]
        pub unsafe fn wheel(&self) -> Retained<GCSteeringWheelElement>;

        #[cfg(all(feature = "GCButtonElement", feature = "GCPhysicalInputElement"))]
        #[method_id(@__retain_semantics Other acceleratorPedal)]
        pub unsafe fn acceleratorPedal(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn GCButtonElement>>>;

        #[cfg(all(feature = "GCButtonElement", feature = "GCPhysicalInputElement"))]
        #[method_id(@__retain_semantics Other brakePedal)]
        pub unsafe fn brakePedal(&self) -> Option<Retained<ProtocolObject<dyn GCButtonElement>>>;

        #[cfg(all(feature = "GCButtonElement", feature = "GCPhysicalInputElement"))]
        #[method_id(@__retain_semantics Other clutchPedal)]
        pub unsafe fn clutchPedal(&self) -> Option<Retained<ProtocolObject<dyn GCButtonElement>>>;

        #[cfg(feature = "GCGearShifterElement")]
        #[method_id(@__retain_semantics Other shifter)]
        pub unsafe fn shifter(&self) -> Option<Retained<GCGearShifterElement>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GCRacingWheelInputState {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(GCRacingWheelInputState, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GCRacingWheelInput;
);

#[cfg(all(
    feature = "GCDevicePhysicalInput",
    feature = "GCDevicePhysicalInputState"
))]
unsafe impl GCDevicePhysicalInput for GCRacingWheelInput {}

#[cfg(feature = "GCDevicePhysicalInputState")]
unsafe impl GCDevicePhysicalInputState for GCRacingWheelInput {}

unsafe impl NSObjectProtocol for GCRacingWheelInput {}

extern_methods!(
    unsafe impl GCRacingWheelInput {
        #[method_id(@__retain_semantics Other capture)]
        pub unsafe fn capture(&self) -> Retained<GCRacingWheelInputState>;

        #[cfg(feature = "GCDevicePhysicalInputStateDiff")]
        #[method_id(@__retain_semantics Other nextInputState)]
        pub unsafe fn nextInputState(&self) -> Option<Retained<GCRacingWheelInputState>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GCRacingWheelInput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

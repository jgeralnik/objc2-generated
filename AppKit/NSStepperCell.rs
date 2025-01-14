//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssteppercell?language=objc)
    #[unsafe(super(NSActionCell, NSCell, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    pub struct NSStepperCell;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSActionCell",
    feature = "NSCell"
))]
unsafe impl NSAccessibility for NSStepperCell {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSActionCell",
    feature = "NSCell"
))]
unsafe impl NSAccessibilityElementProtocol for NSStepperCell {}

#[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
unsafe impl NSCoding for NSStepperCell {}

#[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
unsafe impl NSCopying for NSStepperCell {}

#[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
unsafe impl CopyingHelper for NSStepperCell {
    type Result = Self;
}

#[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
unsafe impl NSObjectProtocol for NSStepperCell {}

#[cfg(all(
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSUserInterfaceItemIdentification"
))]
unsafe impl NSUserInterfaceItemIdentification for NSStepperCell {}

extern_methods!(
    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    unsafe impl NSStepperCell {
        #[method(minValue)]
        pub unsafe fn minValue(&self) -> c_double;

        #[method(setMinValue:)]
        pub unsafe fn setMinValue(&self, min_value: c_double);

        #[method(maxValue)]
        pub unsafe fn maxValue(&self) -> c_double;

        #[method(setMaxValue:)]
        pub unsafe fn setMaxValue(&self, max_value: c_double);

        #[method(increment)]
        pub unsafe fn increment(&self) -> c_double;

        #[method(setIncrement:)]
        pub unsafe fn setIncrement(&self, increment: c_double);

        #[method(valueWraps)]
        pub unsafe fn valueWraps(&self) -> bool;

        #[method(setValueWraps:)]
        pub unsafe fn setValueWraps(&self, value_wraps: bool);

        #[method(autorepeat)]
        pub unsafe fn autorepeat(&self) -> bool;

        #[method(setAutorepeat:)]
        pub unsafe fn setAutorepeat(&self, autorepeat: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    unsafe impl NSStepperCell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(this: Allocated<Self>, string: &NSString) -> Retained<Self>;

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Allocated<Self>,
            image: Option<&NSImage>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    unsafe impl NSStepperCell {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

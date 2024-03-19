//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSProgressIndicatorStyle {
        #[doc(alias = "NSProgressIndicatorStyleBar")]
        Bar = 0,
        #[doc(alias = "NSProgressIndicatorStyleSpinning")]
        Spinning = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    pub struct NSProgressIndicator;

    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl ClassType for NSProgressIndicator {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibility for NSProgressIndicator {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSProgressIndicator {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibilityGroup for NSProgressIndicator {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibilityProgressIndicator for NSProgressIndicator {}

#[cfg(all(
    feature = "AppKit_NSAnimation",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAnimatablePropertyContainer for NSProgressIndicator {}

#[cfg(all(
    feature = "AppKit_NSAppearance",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAppearanceCustomization for NSProgressIndicator {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCoding for NSProgressIndicator {}

#[cfg(all(
    feature = "AppKit_NSDragging",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSDraggingDestination for NSProgressIndicator {}

#[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
unsafe impl NSObjectProtocol for NSProgressIndicator {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSUserInterfaceItemIdentification",
    feature = "AppKit_NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSProgressIndicator {}

extern_methods!(
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl NSProgressIndicator {
        #[method(isIndeterminate)]
        pub unsafe fn isIndeterminate(&self) -> bool;

        #[method(setIndeterminate:)]
        pub unsafe fn setIndeterminate(&self, indeterminate: bool);

        #[cfg(feature = "AppKit_NSCell")]
        #[method(controlSize)]
        pub unsafe fn controlSize(&self) -> NSControlSize;

        #[cfg(feature = "AppKit_NSCell")]
        #[method(setControlSize:)]
        pub unsafe fn setControlSize(&self, control_size: NSControlSize);

        #[method(doubleValue)]
        pub unsafe fn doubleValue(&self) -> c_double;

        #[method(setDoubleValue:)]
        pub unsafe fn setDoubleValue(&self, double_value: c_double);

        #[method(incrementBy:)]
        pub unsafe fn incrementBy(&self, delta: c_double);

        #[method(minValue)]
        pub unsafe fn minValue(&self) -> c_double;

        #[method(setMinValue:)]
        pub unsafe fn setMinValue(&self, min_value: c_double);

        #[method(maxValue)]
        pub unsafe fn maxValue(&self) -> c_double;

        #[method(setMaxValue:)]
        pub unsafe fn setMaxValue(&self, max_value: c_double);

        #[cfg(feature = "Foundation_NSProgress")]
        #[method_id(@__retain_semantics Other observedProgress)]
        pub unsafe fn observedProgress(&self) -> Option<Id<NSProgress>>;

        #[cfg(feature = "Foundation_NSProgress")]
        #[method(setObservedProgress:)]
        pub unsafe fn setObservedProgress(&self, observed_progress: Option<&NSProgress>);

        #[method(usesThreadedAnimation)]
        pub unsafe fn usesThreadedAnimation(&self) -> bool;

        #[method(setUsesThreadedAnimation:)]
        pub unsafe fn setUsesThreadedAnimation(&self, uses_threaded_animation: bool);

        #[method(startAnimation:)]
        pub unsafe fn startAnimation(&self, sender: Option<&AnyObject>);

        #[method(stopAnimation:)]
        pub unsafe fn stopAnimation(&self, sender: Option<&AnyObject>);

        #[method(style)]
        pub unsafe fn style(&self) -> NSProgressIndicatorStyle;

        #[method(setStyle:)]
        pub unsafe fn setStyle(&self, style: NSProgressIndicatorStyle);

        #[method(sizeToFit)]
        pub unsafe fn sizeToFit(&self);

        #[method(isDisplayedWhenStopped)]
        pub unsafe fn isDisplayedWhenStopped(&self) -> bool;

        #[method(setDisplayedWhenStopped:)]
        pub unsafe fn setDisplayedWhenStopped(&self, displayed_when_stopped: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl NSProgressIndicator {
        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl NSProgressIndicator {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl NSProgressIndicator {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    #[deprecated = "These constants do not accurately represent the geometry of NSProgressIndicator.  Use `controlSize` and `sizeToFit` instead."]
    pub enum NSProgressIndicatorThickness {
        #[deprecated = "These constants do not accurately represent the geometry of NSProgressIndicator.  Use `controlSize` and `sizeToFit` instead."]
        NSProgressIndicatorPreferredThickness = 14,
        #[deprecated = "These constants do not accurately represent the geometry of NSProgressIndicator.  Use `controlSize` and `sizeToFit` instead."]
        NSProgressIndicatorPreferredSmallThickness = 10,
        #[deprecated = "These constants do not accurately represent the geometry of NSProgressIndicator.  Use `controlSize` and `sizeToFit` instead."]
        NSProgressIndicatorPreferredLargeThickness = 18,
        #[deprecated = "These constants do not accurately represent the geometry of NSProgressIndicator.  Use `controlSize` and `sizeToFit` instead."]
        NSProgressIndicatorPreferredAquaThickness = 12,
    }
);

pub static NSProgressIndicatorBarStyle: NSProgressIndicatorStyle =
    NSProgressIndicatorStyle(NSProgressIndicatorStyle::Bar.0);

pub static NSProgressIndicatorSpinningStyle: NSProgressIndicatorStyle =
    NSProgressIndicatorStyle(NSProgressIndicatorStyle::Spinning.0);

extern_methods!(
    /// NSProgressIndicatorDeprecated
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl NSProgressIndicator {
        #[cfg(feature = "Foundation_NSDate")]
        #[deprecated = "The animationDelay property does nothing."]
        #[method(animationDelay)]
        pub unsafe fn animationDelay(&self) -> NSTimeInterval;

        #[cfg(feature = "Foundation_NSDate")]
        #[deprecated = "The animationDelay property does nothing."]
        #[method(setAnimationDelay:)]
        pub unsafe fn setAnimationDelay(&self, delay: NSTimeInterval);

        #[deprecated = "Use -startAnimation and -stopAnimation instead."]
        #[method(animate:)]
        pub unsafe fn animate(&self, sender: Option<&AnyObject>);

        #[deprecated = "The bezeled property is not respected on 10.15 and later"]
        #[method(isBezeled)]
        pub unsafe fn isBezeled(&self) -> bool;

        #[deprecated = "The bezeled property is not respected on 10.15 and later"]
        #[method(setBezeled:)]
        pub unsafe fn setBezeled(&self, bezeled: bool);

        #[cfg(feature = "AppKit_NSCell")]
        #[deprecated = "The controlTint property is not respected on 10.15 and later"]
        #[method(controlTint)]
        pub unsafe fn controlTint(&self) -> NSControlTint;

        #[cfg(feature = "AppKit_NSCell")]
        #[deprecated = "The controlTint property is not respected on 10.15 and later"]
        #[method(setControlTint:)]
        pub unsafe fn setControlTint(&self, control_tint: NSControlTint);
    }
);

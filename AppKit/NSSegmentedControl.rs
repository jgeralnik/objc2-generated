//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssegmentswitchtracking?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSSegmentSwitchTracking(pub NSUInteger);
impl NSSegmentSwitchTracking {
    #[doc(alias = "NSSegmentSwitchTrackingSelectOne")]
    pub const SelectOne: Self = Self(0);
    #[doc(alias = "NSSegmentSwitchTrackingSelectAny")]
    pub const SelectAny: Self = Self(1);
    #[doc(alias = "NSSegmentSwitchTrackingMomentary")]
    pub const Momentary: Self = Self(2);
    #[doc(alias = "NSSegmentSwitchTrackingMomentaryAccelerator")]
    pub const MomentaryAccelerator: Self = Self(3);
}

unsafe impl Encode for NSSegmentSwitchTracking {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSSegmentSwitchTracking {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssegmentstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSSegmentStyle(pub NSInteger);
impl NSSegmentStyle {
    #[doc(alias = "NSSegmentStyleAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "NSSegmentStyleRounded")]
    pub const Rounded: Self = Self(1);
    #[doc(alias = "NSSegmentStyleRoundRect")]
    pub const RoundRect: Self = Self(3);
    #[doc(alias = "NSSegmentStyleTexturedSquare")]
    pub const TexturedSquare: Self = Self(4);
    #[doc(alias = "NSSegmentStyleSmallSquare")]
    pub const SmallSquare: Self = Self(6);
    #[doc(alias = "NSSegmentStyleSeparated")]
    pub const Separated: Self = Self(8);
    #[doc(alias = "NSSegmentStyleTexturedRounded")]
    pub const TexturedRounded: Self = Self(2);
    #[doc(alias = "NSSegmentStyleCapsule")]
    pub const Capsule: Self = Self(5);
}

unsafe impl Encode for NSSegmentStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSSegmentStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssegmentdistribution?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSSegmentDistribution(pub NSInteger);
impl NSSegmentDistribution {
    #[doc(alias = "NSSegmentDistributionFit")]
    pub const Fit: Self = Self(0);
    #[doc(alias = "NSSegmentDistributionFill")]
    pub const Fill: Self = Self(1);
    #[doc(alias = "NSSegmentDistributionFillEqually")]
    pub const FillEqually: Self = Self(2);
    #[doc(alias = "NSSegmentDistributionFillProportionally")]
    pub const FillProportionally: Self = Self(3);
}

unsafe impl Encode for NSSegmentDistribution {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSSegmentDistribution {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssegmentedcontrol?language=objc)
    #[unsafe(super(NSControl, NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    pub struct NSSegmentedControl;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSSegmentedControl {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSSegmentedControl {}

#[cfg(all(
    feature = "NSAnimation",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAnimatablePropertyContainer for NSSegmentedControl {}

#[cfg(all(
    feature = "NSAppearance",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAppearanceCustomization for NSSegmentedControl {}

#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSCoding for NSSegmentedControl {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSDragging",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSDraggingDestination for NSSegmentedControl {}

#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSObjectProtocol for NSSegmentedControl {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSUserInterfaceCompression",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceCompression for NSSegmentedControl {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSSegmentedControl {}

extern_methods!(
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSSegmentedControl {
        #[method(segmentCount)]
        pub unsafe fn segmentCount(&self) -> NSInteger;

        #[method(setSegmentCount:)]
        pub unsafe fn setSegmentCount(&self, segment_count: NSInteger);

        #[method(selectedSegment)]
        pub unsafe fn selectedSegment(&self) -> NSInteger;

        #[method(setSelectedSegment:)]
        pub unsafe fn setSelectedSegment(&self, selected_segment: NSInteger);

        #[method(selectSegmentWithTag:)]
        pub unsafe fn selectSegmentWithTag(&self, tag: NSInteger) -> bool;

        #[method(setWidth:forSegment:)]
        pub unsafe fn setWidth_forSegment(&self, width: CGFloat, segment: NSInteger);

        #[method(widthForSegment:)]
        pub unsafe fn widthForSegment(&self, segment: NSInteger) -> CGFloat;

        #[cfg(feature = "NSImage")]
        #[method(setImage:forSegment:)]
        pub unsafe fn setImage_forSegment(&self, image: Option<&NSImage>, segment: NSInteger);

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other imageForSegment:)]
        pub unsafe fn imageForSegment(&self, segment: NSInteger) -> Option<Retained<NSImage>>;

        #[cfg(feature = "NSCell")]
        #[method(setImageScaling:forSegment:)]
        pub unsafe fn setImageScaling_forSegment(
            &self,
            scaling: NSImageScaling,
            segment: NSInteger,
        );

        #[cfg(feature = "NSCell")]
        #[method(imageScalingForSegment:)]
        pub unsafe fn imageScalingForSegment(&self, segment: NSInteger) -> NSImageScaling;

        #[method(setLabel:forSegment:)]
        pub unsafe fn setLabel_forSegment(&self, label: &NSString, segment: NSInteger);

        #[method_id(@__retain_semantics Other labelForSegment:)]
        pub unsafe fn labelForSegment(&self, segment: NSInteger) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSMenu")]
        #[method(setMenu:forSegment:)]
        pub unsafe fn setMenu_forSegment(&self, menu: Option<&NSMenu>, segment: NSInteger);

        #[cfg(feature = "NSMenu")]
        #[method_id(@__retain_semantics Other menuForSegment:)]
        pub unsafe fn menuForSegment(&self, segment: NSInteger) -> Option<Retained<NSMenu>>;

        #[method(setSelected:forSegment:)]
        pub unsafe fn setSelected_forSegment(&self, selected: bool, segment: NSInteger);

        #[method(isSelectedForSegment:)]
        pub unsafe fn isSelectedForSegment(&self, segment: NSInteger) -> bool;

        #[method(setEnabled:forSegment:)]
        pub unsafe fn setEnabled_forSegment(&self, enabled: bool, segment: NSInteger);

        #[method(isEnabledForSegment:)]
        pub unsafe fn isEnabledForSegment(&self, segment: NSInteger) -> bool;

        #[method(setToolTip:forSegment:)]
        pub unsafe fn setToolTip_forSegment(&self, tool_tip: Option<&NSString>, segment: NSInteger);

        #[method_id(@__retain_semantics Other toolTipForSegment:)]
        pub unsafe fn toolTipForSegment(&self, segment: NSInteger) -> Option<Retained<NSString>>;

        #[method(setTag:forSegment:)]
        pub unsafe fn setTag_forSegment(&self, tag: NSInteger, segment: NSInteger);

        #[method(tagForSegment:)]
        pub unsafe fn tagForSegment(&self, segment: NSInteger) -> NSInteger;

        #[method(setShowsMenuIndicator:forSegment:)]
        pub unsafe fn setShowsMenuIndicator_forSegment(
            &self,
            shows_menu_indicator: bool,
            segment: NSInteger,
        );

        #[method(showsMenuIndicatorForSegment:)]
        pub unsafe fn showsMenuIndicatorForSegment(&self, segment: NSInteger) -> bool;

        #[method(segmentStyle)]
        pub unsafe fn segmentStyle(&self) -> NSSegmentStyle;

        #[method(setSegmentStyle:)]
        pub unsafe fn setSegmentStyle(&self, segment_style: NSSegmentStyle);

        #[method(isSpringLoaded)]
        pub unsafe fn isSpringLoaded(&self) -> bool;

        #[method(setSpringLoaded:)]
        pub unsafe fn setSpringLoaded(&self, spring_loaded: bool);

        #[method(trackingMode)]
        pub unsafe fn trackingMode(&self) -> NSSegmentSwitchTracking;

        #[method(setTrackingMode:)]
        pub unsafe fn setTrackingMode(&self, tracking_mode: NSSegmentSwitchTracking);

        #[method(doubleValueForSelectedSegment)]
        pub unsafe fn doubleValueForSelectedSegment(&self) -> c_double;

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other selectedSegmentBezelColor)]
        pub unsafe fn selectedSegmentBezelColor(&self) -> Option<Retained<NSColor>>;

        #[cfg(feature = "NSColor")]
        #[method(setSelectedSegmentBezelColor:)]
        pub unsafe fn setSelectedSegmentBezelColor(
            &self,
            selected_segment_bezel_color: Option<&NSColor>,
        );

        #[method(indexOfSelectedItem)]
        pub unsafe fn indexOfSelectedItem(&self) -> NSInteger;

        #[cfg(feature = "NSText")]
        #[method(setAlignment:forSegment:)]
        pub unsafe fn setAlignment_forSegment(
            &self,
            alignment: NSTextAlignment,
            segment: NSInteger,
        );

        #[cfg(feature = "NSText")]
        #[method(alignmentForSegment:)]
        pub unsafe fn alignmentForSegment(&self, segment: NSInteger) -> NSTextAlignment;

        #[method(segmentDistribution)]
        pub unsafe fn segmentDistribution(&self) -> NSSegmentDistribution;

        #[method(setSegmentDistribution:)]
        pub unsafe fn setSegmentDistribution(&self, segment_distribution: NSSegmentDistribution);

        #[cfg(feature = "NSUserInterfaceCompression")]
        #[method(compressWithPrioritizedCompressionOptions:)]
        pub unsafe fn compressWithPrioritizedCompressionOptions(
            &self,
            prioritized_options: &NSArray<NSUserInterfaceCompressionOptions>,
        );

        #[cfg(feature = "NSUserInterfaceCompression")]
        #[method(minimumSizeWithPrioritizedCompressionOptions:)]
        pub unsafe fn minimumSizeWithPrioritizedCompressionOptions(
            &self,
            prioritized_options: &NSArray<NSUserInterfaceCompressionOptions>,
        ) -> NSSize;

        #[cfg(feature = "NSUserInterfaceCompression")]
        #[method_id(@__retain_semantics Other activeCompressionOptions)]
        pub unsafe fn activeCompressionOptions(
            &self,
        ) -> Retained<NSUserInterfaceCompressionOptions>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSSegmentedControl {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSSegmentedControl {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSSegmentedControl {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// NSSegmentedControlConvenience
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSSegmentedControl {
        #[method_id(@__retain_semantics Other segmentedControlWithLabels:trackingMode:target:action:)]
        pub unsafe fn segmentedControlWithLabels_trackingMode_target_action(
            labels: &NSArray<NSString>,
            tracking_mode: NSSegmentSwitchTracking,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other segmentedControlWithImages:trackingMode:target:action:)]
        pub unsafe fn segmentedControlWithImages_trackingMode_target_action(
            images: &NSArray<NSImage>,
            tracking_mode: NSSegmentSwitchTracking,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;
    }
);

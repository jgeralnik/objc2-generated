//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    pub struct NSSegmentedCell;

    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    unsafe impl ClassType for NSSegmentedCell {
        #[inherits(NSCell, NSObject)]
        type Super = NSActionCell;
    }
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSActionCell",
    feature = "NSCell"
))]
unsafe impl NSAccessibility for NSSegmentedCell {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSActionCell",
    feature = "NSCell"
))]
unsafe impl NSAccessibilityElementProtocol for NSSegmentedCell {}

#[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
unsafe impl NSCoding for NSSegmentedCell {}

#[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
unsafe impl NSCopying for NSSegmentedCell {}

#[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
unsafe impl CopyingHelper for NSSegmentedCell {
    type Result = Self;
}

#[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
unsafe impl NSObjectProtocol for NSSegmentedCell {}

#[cfg(all(
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSUserInterfaceItemIdentification"
))]
unsafe impl NSUserInterfaceItemIdentification for NSSegmentedCell {}

extern_methods!(
    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    unsafe impl NSSegmentedCell {
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

        #[method(makeNextSegmentKey)]
        pub unsafe fn makeNextSegmentKey(&self);

        #[method(makePreviousSegmentKey)]
        pub unsafe fn makePreviousSegmentKey(&self);

        #[cfg(feature = "NSSegmentedControl")]
        #[method(trackingMode)]
        pub unsafe fn trackingMode(&self) -> NSSegmentSwitchTracking;

        #[cfg(feature = "NSSegmentedControl")]
        #[method(setTrackingMode:)]
        pub unsafe fn setTrackingMode(&self, tracking_mode: NSSegmentSwitchTracking);

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

        #[method(setImageScaling:forSegment:)]
        pub unsafe fn setImageScaling_forSegment(
            &self,
            scaling: NSImageScaling,
            segment: NSInteger,
        );

        #[method(imageScalingForSegment:)]
        pub unsafe fn imageScalingForSegment(&self, segment: NSInteger) -> NSImageScaling;

        #[method(setLabel:forSegment:)]
        pub unsafe fn setLabel_forSegment(&self, label: &NSString, segment: NSInteger);

        #[method_id(@__retain_semantics Other labelForSegment:)]
        pub unsafe fn labelForSegment(&self, segment: NSInteger) -> Option<Retained<NSString>>;

        #[method(setSelected:forSegment:)]
        pub unsafe fn setSelected_forSegment(&self, selected: bool, segment: NSInteger);

        #[method(isSelectedForSegment:)]
        pub unsafe fn isSelectedForSegment(&self, segment: NSInteger) -> bool;

        #[method(setEnabled:forSegment:)]
        pub unsafe fn setEnabled_forSegment(&self, enabled: bool, segment: NSInteger);

        #[method(isEnabledForSegment:)]
        pub unsafe fn isEnabledForSegment(&self, segment: NSInteger) -> bool;

        #[cfg(feature = "NSMenu")]
        #[method(setMenu:forSegment:)]
        pub unsafe fn setMenu_forSegment(&self, menu: Option<&NSMenu>, segment: NSInteger);

        #[cfg(feature = "NSMenu")]
        #[method_id(@__retain_semantics Other menuForSegment:)]
        pub unsafe fn menuForSegment(&self, segment: NSInteger) -> Option<Retained<NSMenu>>;

        #[method(setToolTip:forSegment:)]
        pub unsafe fn setToolTip_forSegment(&self, tool_tip: Option<&NSString>, segment: NSInteger);

        #[method_id(@__retain_semantics Other toolTipForSegment:)]
        pub unsafe fn toolTipForSegment(&self, segment: NSInteger) -> Option<Retained<NSString>>;

        #[method(setTag:forSegment:)]
        pub unsafe fn setTag_forSegment(&self, tag: NSInteger, segment: NSInteger);

        #[method(tagForSegment:)]
        pub unsafe fn tagForSegment(&self, segment: NSInteger) -> NSInteger;

        #[cfg(feature = "NSSegmentedControl")]
        #[method(segmentStyle)]
        pub unsafe fn segmentStyle(&self) -> NSSegmentStyle;

        #[cfg(feature = "NSSegmentedControl")]
        #[method(setSegmentStyle:)]
        pub unsafe fn setSegmentStyle(&self, segment_style: NSSegmentStyle);

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method(drawSegment:inFrame:withView:)]
        pub unsafe fn drawSegment_inFrame_withView(
            &self,
            segment: NSInteger,
            frame: NSRect,
            control_view: &NSView,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    unsafe impl NSSegmentedCell {
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
    unsafe impl NSSegmentedCell {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// NSSegmentBackgroundStyle
    #[cfg(all(feature = "NSActionCell", feature = "NSCell"))]
    unsafe impl NSSegmentedCell {
        #[method(interiorBackgroundStyleForSegment:)]
        pub unsafe fn interiorBackgroundStyleForSegment(
            &self,
            segment: NSInteger,
        ) -> NSBackgroundStyle;
    }
);

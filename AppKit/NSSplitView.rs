//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSSplitViewAutosaveName = NSString;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSSplitViewDividerStyle {
        NSSplitViewDividerStyleThick = 1,
        NSSplitViewDividerStyleThin = 2,
        NSSplitViewDividerStylePaneSplitter = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSplitView;

    unsafe impl ClassType for NSSplitView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSSplitView")]
    unsafe impl NSSplitView {
        #[method(isVertical)]
        pub unsafe fn isVertical(&self) -> bool;

        #[method(setVertical:)]
        pub unsafe fn setVertical(&self, vertical: bool);

        #[method(dividerStyle)]
        pub unsafe fn dividerStyle(&self) -> NSSplitViewDividerStyle;

        #[method(setDividerStyle:)]
        pub unsafe fn setDividerStyle(&self, dividerStyle: NSSplitViewDividerStyle);

        #[cfg(feature = "AppKit_NSSplitViewAutosaveName")]
        #[method_id(@__retain_semantics Other autosaveName)]
        pub unsafe fn autosaveName(&self) -> Option<Id<NSSplitViewAutosaveName, Shared>>;

        #[cfg(feature = "AppKit_NSSplitViewAutosaveName")]
        #[method(setAutosaveName:)]
        pub unsafe fn setAutosaveName(&self, autosaveName: Option<&NSSplitViewAutosaveName>);

        #[cfg(feature = "AppKit_NSSplitViewDelegate")]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSSplitViewDelegate, Shared>>;

        #[cfg(feature = "AppKit_NSSplitViewDelegate")]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSSplitViewDelegate>);

        #[method(drawDividerInRect:)]
        pub unsafe fn drawDividerInRect(&self, rect: NSRect);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other dividerColor)]
        pub unsafe fn dividerColor(&self) -> Id<NSColor, Shared>;

        #[method(dividerThickness)]
        pub unsafe fn dividerThickness(&self) -> CGFloat;

        #[method(adjustSubviews)]
        pub unsafe fn adjustSubviews(&self);

        #[method(isSubviewCollapsed:)]
        pub unsafe fn isSubviewCollapsed(&self, subview: &NSView) -> bool;

        #[method(minPossiblePositionOfDividerAtIndex:)]
        pub unsafe fn minPossiblePositionOfDividerAtIndex(
            &self,
            dividerIndex: NSInteger,
        ) -> CGFloat;

        #[method(maxPossiblePositionOfDividerAtIndex:)]
        pub unsafe fn maxPossiblePositionOfDividerAtIndex(
            &self,
            dividerIndex: NSInteger,
        ) -> CGFloat;

        #[method(setPosition:ofDividerAtIndex:)]
        pub unsafe fn setPosition_ofDividerAtIndex(
            &self,
            position: CGFloat,
            dividerIndex: NSInteger,
        );

        #[method(holdingPriorityForSubviewAtIndex:)]
        pub unsafe fn holdingPriorityForSubviewAtIndex(
            &self,
            subviewIndex: NSInteger,
        ) -> NSLayoutPriority;

        #[method(setHoldingPriority:forSubviewAtIndex:)]
        pub unsafe fn setHoldingPriority_forSubviewAtIndex(
            &self,
            priority: NSLayoutPriority,
            subviewIndex: NSInteger,
        );
    }
);

extern_methods!(
    /// NSSplitViewArrangedSubviews
    #[cfg(feature = "AppKit_NSSplitView")]
    unsafe impl NSSplitView {
        #[method(arrangesAllSubviews)]
        pub unsafe fn arrangesAllSubviews(&self) -> bool;

        #[method(setArrangesAllSubviews:)]
        pub unsafe fn setArrangesAllSubviews(&self, arrangesAllSubviews: bool);

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other arrangedSubviews)]
        pub unsafe fn arrangedSubviews(&self) -> Id<NSArray<NSView>, Shared>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(addArrangedSubview:)]
        pub unsafe fn addArrangedSubview(&self, view: &NSView);

        #[cfg(feature = "AppKit_NSView")]
        #[method(insertArrangedSubview:atIndex:)]
        pub unsafe fn insertArrangedSubview_atIndex(&self, view: &NSView, index: NSInteger);

        #[cfg(feature = "AppKit_NSView")]
        #[method(removeArrangedSubview:)]
        pub unsafe fn removeArrangedSubview(&self, view: &NSView);
    }
);

extern_protocol!(
    pub struct NSSplitViewDelegate;

    unsafe impl ProtocolType for NSSplitViewDelegate {
        #[optional]
        #[method(splitView:canCollapseSubview:)]
        pub unsafe fn splitView_canCollapseSubview(
            &self,
            splitView: &NSSplitView,
            subview: &NSView,
        ) -> bool;

        #[optional]
        #[method(splitView:shouldCollapseSubview:forDoubleClickOnDividerAtIndex:)]
        pub unsafe fn splitView_shouldCollapseSubview_forDoubleClickOnDividerAtIndex(
            &self,
            splitView: &NSSplitView,
            subview: &NSView,
            dividerIndex: NSInteger,
        ) -> bool;

        #[optional]
        #[method(splitView:constrainMinCoordinate:ofSubviewAt:)]
        pub unsafe fn splitView_constrainMinCoordinate_ofSubviewAt(
            &self,
            splitView: &NSSplitView,
            proposedMinimumPosition: CGFloat,
            dividerIndex: NSInteger,
        ) -> CGFloat;

        #[optional]
        #[method(splitView:constrainMaxCoordinate:ofSubviewAt:)]
        pub unsafe fn splitView_constrainMaxCoordinate_ofSubviewAt(
            &self,
            splitView: &NSSplitView,
            proposedMaximumPosition: CGFloat,
            dividerIndex: NSInteger,
        ) -> CGFloat;

        #[optional]
        #[method(splitView:constrainSplitPosition:ofSubviewAt:)]
        pub unsafe fn splitView_constrainSplitPosition_ofSubviewAt(
            &self,
            splitView: &NSSplitView,
            proposedPosition: CGFloat,
            dividerIndex: NSInteger,
        ) -> CGFloat;

        #[optional]
        #[method(splitView:resizeSubviewsWithOldSize:)]
        pub unsafe fn splitView_resizeSubviewsWithOldSize(
            &self,
            splitView: &NSSplitView,
            oldSize: NSSize,
        );

        #[optional]
        #[method(splitView:shouldAdjustSizeOfSubview:)]
        pub unsafe fn splitView_shouldAdjustSizeOfSubview(
            &self,
            splitView: &NSSplitView,
            view: &NSView,
        ) -> bool;

        #[optional]
        #[method(splitView:shouldHideDividerAtIndex:)]
        pub unsafe fn splitView_shouldHideDividerAtIndex(
            &self,
            splitView: &NSSplitView,
            dividerIndex: NSInteger,
        ) -> bool;

        #[optional]
        #[method(splitView:effectiveRect:forDrawnRect:ofDividerAtIndex:)]
        pub unsafe fn splitView_effectiveRect_forDrawnRect_ofDividerAtIndex(
            &self,
            splitView: &NSSplitView,
            proposedEffectiveRect: NSRect,
            drawnRect: NSRect,
            dividerIndex: NSInteger,
        ) -> NSRect;

        #[optional]
        #[method(splitView:additionalEffectiveRectOfDividerAtIndex:)]
        pub unsafe fn splitView_additionalEffectiveRectOfDividerAtIndex(
            &self,
            splitView: &NSSplitView,
            dividerIndex: NSInteger,
        ) -> NSRect;

        #[optional]
        #[method(splitViewWillResizeSubviews:)]
        pub unsafe fn splitViewWillResizeSubviews(&self, notification: &NSNotification);

        #[optional]
        #[method(splitViewDidResizeSubviews:)]
        pub unsafe fn splitViewDidResizeSubviews(&self, notification: &NSNotification);
    }
);

extern_static!(NSSplitViewWillResizeSubviewsNotification: &'static NSNotificationName);

extern_static!(NSSplitViewDidResizeSubviewsNotification: &'static NSNotificationName);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSSplitView")]
    unsafe impl NSSplitView {
        #[method(setIsPaneSplitter:)]
        pub unsafe fn setIsPaneSplitter(&self, flag: bool);

        #[method(isPaneSplitter)]
        pub unsafe fn isPaneSplitter(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSSplitView")]
    unsafe impl NSSplitView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: NSRect,
        ) -> Id<Self, Shared>;
    }
);

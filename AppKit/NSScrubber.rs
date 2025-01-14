//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsscrubberdatasource?language=objc)
    pub unsafe trait NSScrubberDataSource: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method(numberOfItemsForScrubber:)]
        unsafe fn numberOfItemsForScrubber(&self, scrubber: &NSScrubber) -> NSInteger;

        #[cfg(all(
            feature = "NSResponder",
            feature = "NSScrubberItemView",
            feature = "NSView"
        ))]
        #[method_id(@__retain_semantics Other scrubber:viewForItemAtIndex:)]
        unsafe fn scrubber_viewForItemAtIndex(
            &self,
            scrubber: &NSScrubber,
            index: NSInteger,
        ) -> Retained<NSScrubberItemView>;
    }

    unsafe impl ProtocolType for dyn NSScrubberDataSource {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsscrubberdelegate?language=objc)
    pub unsafe trait NSScrubberDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method(scrubber:didSelectItemAtIndex:)]
        unsafe fn scrubber_didSelectItemAtIndex(
            &self,
            scrubber: &NSScrubber,
            selected_index: NSInteger,
        );

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method(scrubber:didHighlightItemAtIndex:)]
        unsafe fn scrubber_didHighlightItemAtIndex(
            &self,
            scrubber: &NSScrubber,
            highlighted_index: NSInteger,
        );

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method(scrubber:didChangeVisibleRange:)]
        unsafe fn scrubber_didChangeVisibleRange(
            &self,
            scrubber: &NSScrubber,
            visible_range: NSRange,
        );

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method(didBeginInteractingWithScrubber:)]
        unsafe fn didBeginInteractingWithScrubber(&self, scrubber: &NSScrubber);

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method(didFinishInteractingWithScrubber:)]
        unsafe fn didFinishInteractingWithScrubber(&self, scrubber: &NSScrubber);

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method(didCancelInteractingWithScrubber:)]
        unsafe fn didCancelInteractingWithScrubber(&self, scrubber: &NSScrubber);
    }

    unsafe impl ProtocolType for dyn NSScrubberDelegate {}
);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsscrubbermode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSScrubberMode(pub NSInteger);
impl NSScrubberMode {
    #[doc(alias = "NSScrubberModeFixed")]
    pub const Fixed: Self = Self(0);
    #[doc(alias = "NSScrubberModeFree")]
    pub const Free: Self = Self(1);
}

unsafe impl Encode for NSScrubberMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSScrubberMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsscrubberalignment?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSScrubberAlignment(pub NSInteger);
impl NSScrubberAlignment {
    #[doc(alias = "NSScrubberAlignmentNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "NSScrubberAlignmentLeading")]
    pub const Leading: Self = Self(1);
    #[doc(alias = "NSScrubberAlignmentTrailing")]
    pub const Trailing: Self = Self(2);
    #[doc(alias = "NSScrubberAlignmentCenter")]
    pub const Center: Self = Self(3);
}

unsafe impl Encode for NSScrubberAlignment {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSScrubberAlignment {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsscrubberselectionstyle?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSScrubberSelectionStyle;
);

unsafe impl NSCoding for NSScrubberSelectionStyle {}

unsafe impl NSObjectProtocol for NSScrubberSelectionStyle {}

extern_methods!(
    unsafe impl NSScrubberSelectionStyle {
        #[method_id(@__retain_semantics Other outlineOverlayStyle)]
        pub unsafe fn outlineOverlayStyle(
            mtm: MainThreadMarker,
        ) -> Retained<NSScrubberSelectionStyle>;

        #[method_id(@__retain_semantics Other roundedBackgroundStyle)]
        pub unsafe fn roundedBackgroundStyle(
            mtm: MainThreadMarker,
        ) -> Retained<NSScrubberSelectionStyle>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[cfg(all(
            feature = "NSResponder",
            feature = "NSScrubberItemView",
            feature = "NSView"
        ))]
        #[method_id(@__retain_semantics Other makeSelectionView)]
        pub unsafe fn makeSelectionView(&self) -> Option<Retained<NSScrubberSelectionView>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSScrubberSelectionStyle {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsscrubber?language=objc)
    #[unsafe(super(NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    pub struct NSScrubber;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSScrubber {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSScrubber {}

#[cfg(all(feature = "NSAnimation", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSAnimatablePropertyContainer for NSScrubber {}

#[cfg(all(feature = "NSAppearance", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSAppearanceCustomization for NSScrubber {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSCoding for NSScrubber {}

#[cfg(all(feature = "NSDragging", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSDraggingDestination for NSScrubber {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSObjectProtocol for NSScrubber {}

#[cfg(all(
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSScrubber {}

extern_methods!(
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSScrubber {
        #[method_id(@__retain_semantics Other dataSource)]
        pub unsafe fn dataSource(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSScrubberDataSource>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDataSource:)]
        pub unsafe fn setDataSource(
            &self,
            data_source: Option<&ProtocolObject<dyn NSScrubberDataSource>>,
        );

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn NSScrubberDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSScrubberDelegate>>);

        #[cfg(feature = "NSScrubberLayout")]
        #[method_id(@__retain_semantics Other scrubberLayout)]
        pub unsafe fn scrubberLayout(&self) -> Retained<NSScrubberLayout>;

        #[cfg(feature = "NSScrubberLayout")]
        #[method(setScrubberLayout:)]
        pub unsafe fn setScrubberLayout(&self, scrubber_layout: &NSScrubberLayout);

        #[method(numberOfItems)]
        pub unsafe fn numberOfItems(&self) -> NSInteger;

        #[method(highlightedIndex)]
        pub unsafe fn highlightedIndex(&self) -> NSInteger;

        #[method(selectedIndex)]
        pub unsafe fn selectedIndex(&self) -> NSInteger;

        #[method(setSelectedIndex:)]
        pub unsafe fn setSelectedIndex(&self, selected_index: NSInteger);

        #[method(mode)]
        pub unsafe fn mode(&self) -> NSScrubberMode;

        #[method(setMode:)]
        pub unsafe fn setMode(&self, mode: NSScrubberMode);

        #[method(itemAlignment)]
        pub unsafe fn itemAlignment(&self) -> NSScrubberAlignment;

        #[method(setItemAlignment:)]
        pub unsafe fn setItemAlignment(&self, item_alignment: NSScrubberAlignment);

        #[method(isContinuous)]
        pub unsafe fn isContinuous(&self) -> bool;

        #[method(setContinuous:)]
        pub unsafe fn setContinuous(&self, continuous: bool);

        #[method(floatsSelectionViews)]
        pub unsafe fn floatsSelectionViews(&self) -> bool;

        #[method(setFloatsSelectionViews:)]
        pub unsafe fn setFloatsSelectionViews(&self, floats_selection_views: bool);

        #[method_id(@__retain_semantics Other selectionBackgroundStyle)]
        pub unsafe fn selectionBackgroundStyle(&self)
            -> Option<Retained<NSScrubberSelectionStyle>>;

        #[method(setSelectionBackgroundStyle:)]
        pub unsafe fn setSelectionBackgroundStyle(
            &self,
            selection_background_style: Option<&NSScrubberSelectionStyle>,
        );

        #[method_id(@__retain_semantics Other selectionOverlayStyle)]
        pub unsafe fn selectionOverlayStyle(&self) -> Option<Retained<NSScrubberSelectionStyle>>;

        #[method(setSelectionOverlayStyle:)]
        pub unsafe fn setSelectionOverlayStyle(
            &self,
            selection_overlay_style: Option<&NSScrubberSelectionStyle>,
        );

        #[method(showsArrowButtons)]
        pub unsafe fn showsArrowButtons(&self) -> bool;

        #[method(setShowsArrowButtons:)]
        pub unsafe fn setShowsArrowButtons(&self, shows_arrow_buttons: bool);

        #[method(showsAdditionalContentIndicators)]
        pub unsafe fn showsAdditionalContentIndicators(&self) -> bool;

        #[method(setShowsAdditionalContentIndicators:)]
        pub unsafe fn setShowsAdditionalContentIndicators(
            &self,
            shows_additional_content_indicators: bool,
        );

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Retained<NSColor>>;

        #[cfg(feature = "NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: Option<&NSColor>);

        #[method_id(@__retain_semantics Other backgroundView)]
        pub unsafe fn backgroundView(&self) -> Option<Retained<NSView>>;

        #[method(setBackgroundView:)]
        pub unsafe fn setBackgroundView(&self, background_view: Option<&NSView>);

        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[method(reloadData)]
        pub unsafe fn reloadData(&self);

        #[cfg(feature = "block2")]
        #[method(performSequentialBatchUpdates:)]
        pub unsafe fn performSequentialBatchUpdates(
            &self,
            update_block: &block2::Block<dyn Fn() + '_>,
        );

        #[method(insertItemsAtIndexes:)]
        pub unsafe fn insertItemsAtIndexes(&self, indexes: &NSIndexSet);

        #[method(removeItemsAtIndexes:)]
        pub unsafe fn removeItemsAtIndexes(&self, indexes: &NSIndexSet);

        #[method(reloadItemsAtIndexes:)]
        pub unsafe fn reloadItemsAtIndexes(&self, indexes: &NSIndexSet);

        #[method(moveItemAtIndex:toIndex:)]
        pub unsafe fn moveItemAtIndex_toIndex(&self, old_index: NSInteger, new_index: NSInteger);

        #[method(scrollItemAtIndex:toAlignment:)]
        pub unsafe fn scrollItemAtIndex_toAlignment(
            &self,
            index: NSInteger,
            alignment: NSScrubberAlignment,
        );

        #[cfg(feature = "NSScrubberItemView")]
        #[method_id(@__retain_semantics Other itemViewForItemAtIndex:)]
        pub unsafe fn itemViewForItemAtIndex(
            &self,
            index: NSInteger,
        ) -> Option<Retained<NSScrubberItemView>>;

        #[cfg(feature = "NSUserInterfaceItemIdentification")]
        #[method(registerClass:forItemIdentifier:)]
        pub unsafe fn registerClass_forItemIdentifier(
            &self,
            item_view_class: Option<&AnyClass>,
            item_identifier: &NSUserInterfaceItemIdentifier,
        );

        #[cfg(all(feature = "NSNib", feature = "NSUserInterfaceItemIdentification"))]
        #[method(registerNib:forItemIdentifier:)]
        pub unsafe fn registerNib_forItemIdentifier(
            &self,
            nib: Option<&NSNib>,
            item_identifier: &NSUserInterfaceItemIdentifier,
        );

        #[cfg(all(
            feature = "NSScrubberItemView",
            feature = "NSUserInterfaceItemIdentification"
        ))]
        #[method_id(@__retain_semantics Other makeItemWithIdentifier:owner:)]
        pub unsafe fn makeItemWithIdentifier_owner(
            &self,
            item_identifier: &NSUserInterfaceItemIdentifier,
            owner: Option<&AnyObject>,
        ) -> Option<Retained<NSScrubberItemView>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSScrubber {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSScrubber {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

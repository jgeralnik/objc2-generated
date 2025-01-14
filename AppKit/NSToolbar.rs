//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstoolbaridentifier?language=objc)
pub type NSToolbarIdentifier = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstoolbaritemidentifier?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type NSToolbarItemIdentifier = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstoolbaruserinfokey?language=objc)
// NS_TYPED_ENUM
pub type NSToolbarUserInfoKey = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstoolbaritemkey?language=objc)
    pub static NSToolbarItemKey: &'static NSToolbarUserInfoKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstoolbarnewindexkey?language=objc)
    pub static NSToolbarNewIndexKey: &'static NSToolbarUserInfoKey;
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstoolbardisplaymode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSToolbarDisplayMode(pub NSUInteger);
impl NSToolbarDisplayMode {
    #[doc(alias = "NSToolbarDisplayModeDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "NSToolbarDisplayModeIconAndLabel")]
    pub const IconAndLabel: Self = Self(1);
    #[doc(alias = "NSToolbarDisplayModeIconOnly")]
    pub const IconOnly: Self = Self(2);
    #[doc(alias = "NSToolbarDisplayModeLabelOnly")]
    pub const LabelOnly: Self = Self(3);
}

unsafe impl Encode for NSToolbarDisplayMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSToolbarDisplayMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstoolbarsizemode?language=objc)
// NS_ENUM
#[deprecated = "NSToolbarSizeMode is no longer recommended and will be ignored in the future"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSToolbarSizeMode(pub NSUInteger);
impl NSToolbarSizeMode {
    #[deprecated = "NSToolbarSizeMode is no longer recommended and will be ignored in the future"]
    #[doc(alias = "NSToolbarSizeModeDefault")]
    pub const Default: Self = Self(0);
    #[deprecated = "NSToolbarSizeMode is no longer recommended and will be ignored in the future"]
    #[doc(alias = "NSToolbarSizeModeRegular")]
    pub const Regular: Self = Self(1);
    #[deprecated = "NSToolbarSizeMode is no longer recommended and will be ignored in the future"]
    #[doc(alias = "NSToolbarSizeModeSmall")]
    pub const Small: Self = Self(2);
}

unsafe impl Encode for NSToolbarSizeMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSToolbarSizeMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstoolbar?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSToolbar;
);

unsafe impl NSObjectProtocol for NSToolbar {}

extern_methods!(
    unsafe impl NSToolbar {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Allocated<Self>,
            identifier: &NSToolbarIdentifier,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(insertItemWithItemIdentifier:atIndex:)]
        pub unsafe fn insertItemWithItemIdentifier_atIndex(
            &self,
            item_identifier: &NSToolbarItemIdentifier,
            index: NSInteger,
        );

        #[method(removeItemAtIndex:)]
        pub unsafe fn removeItemAtIndex(&self, index: NSInteger);

        #[method(removeItemWithItemIdentifier:)]
        pub unsafe fn removeItemWithItemIdentifier(
            &self,
            item_identifier: &NSToolbarItemIdentifier,
        );

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn NSToolbarDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSToolbarDelegate>>);

        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;

        #[method(setVisible:)]
        pub unsafe fn setVisible(&self, visible: bool);

        #[method(runCustomizationPalette:)]
        pub unsafe fn runCustomizationPalette(&self, sender: Option<&AnyObject>);

        #[method(customizationPaletteIsRunning)]
        pub unsafe fn customizationPaletteIsRunning(&self) -> bool;

        #[method(displayMode)]
        pub unsafe fn displayMode(&self) -> NSToolbarDisplayMode;

        #[method(setDisplayMode:)]
        pub unsafe fn setDisplayMode(&self, display_mode: NSToolbarDisplayMode);

        #[method_id(@__retain_semantics Other selectedItemIdentifier)]
        pub unsafe fn selectedItemIdentifier(&self) -> Option<Retained<NSToolbarItemIdentifier>>;

        #[method(setSelectedItemIdentifier:)]
        pub unsafe fn setSelectedItemIdentifier(
            &self,
            selected_item_identifier: Option<&NSToolbarItemIdentifier>,
        );

        #[method(allowsUserCustomization)]
        pub unsafe fn allowsUserCustomization(&self) -> bool;

        #[method(setAllowsUserCustomization:)]
        pub unsafe fn setAllowsUserCustomization(&self, allows_user_customization: bool);

        #[method(allowsDisplayModeCustomization)]
        pub unsafe fn allowsDisplayModeCustomization(&self) -> bool;

        #[method(setAllowsDisplayModeCustomization:)]
        pub unsafe fn setAllowsDisplayModeCustomization(
            &self,
            allows_display_mode_customization: bool,
        );

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSToolbarIdentifier>;

        #[cfg(feature = "NSToolbarItem")]
        #[method_id(@__retain_semantics Other items)]
        pub unsafe fn items(&self) -> Retained<NSArray<NSToolbarItem>>;

        #[cfg(feature = "NSToolbarItem")]
        #[method_id(@__retain_semantics Other visibleItems)]
        pub unsafe fn visibleItems(&self) -> Option<Retained<NSArray<NSToolbarItem>>>;

        #[method_id(@__retain_semantics Other itemIdentifiers)]
        pub unsafe fn itemIdentifiers(&self) -> Retained<NSArray<NSToolbarItemIdentifier>>;

        #[method(setItemIdentifiers:)]
        pub unsafe fn setItemIdentifiers(
            &self,
            item_identifiers: &NSArray<NSToolbarItemIdentifier>,
        );

        #[method_id(@__retain_semantics Other centeredItemIdentifiers)]
        pub unsafe fn centeredItemIdentifiers(&self) -> Retained<NSSet<NSToolbarItemIdentifier>>;

        #[method(setCenteredItemIdentifiers:)]
        pub unsafe fn setCenteredItemIdentifiers(
            &self,
            centered_item_identifiers: &NSSet<NSToolbarItemIdentifier>,
        );

        #[method(autosavesConfiguration)]
        pub unsafe fn autosavesConfiguration(&self) -> bool;

        #[method(setAutosavesConfiguration:)]
        pub unsafe fn setAutosavesConfiguration(&self, autosaves_configuration: bool);

        #[method(validateVisibleItems)]
        pub unsafe fn validateVisibleItems(&self);

        #[method(allowsExtensionItems)]
        pub unsafe fn allowsExtensionItems(&self) -> bool;

        #[method(setAllowsExtensionItems:)]
        pub unsafe fn setAllowsExtensionItems(&self, allows_extension_items: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSToolbar {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstoolbardelegate?language=objc)
    pub unsafe trait NSToolbarDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(feature = "NSToolbarItem")]
        #[optional]
        #[method_id(@__retain_semantics Other toolbar:itemForItemIdentifier:willBeInsertedIntoToolbar:)]
        unsafe fn toolbar_itemForItemIdentifier_willBeInsertedIntoToolbar(
            &self,
            toolbar: &NSToolbar,
            item_identifier: &NSToolbarItemIdentifier,
            flag: bool,
        ) -> Option<Retained<NSToolbarItem>>;

        #[optional]
        #[method_id(@__retain_semantics Other toolbarDefaultItemIdentifiers:)]
        unsafe fn toolbarDefaultItemIdentifiers(
            &self,
            toolbar: &NSToolbar,
        ) -> Retained<NSArray<NSToolbarItemIdentifier>>;

        #[optional]
        #[method_id(@__retain_semantics Other toolbarAllowedItemIdentifiers:)]
        unsafe fn toolbarAllowedItemIdentifiers(
            &self,
            toolbar: &NSToolbar,
        ) -> Retained<NSArray<NSToolbarItemIdentifier>>;

        #[optional]
        #[method_id(@__retain_semantics Other toolbarSelectableItemIdentifiers:)]
        unsafe fn toolbarSelectableItemIdentifiers(
            &self,
            toolbar: &NSToolbar,
        ) -> Retained<NSArray<NSToolbarItemIdentifier>>;

        #[optional]
        #[method_id(@__retain_semantics Other toolbarImmovableItemIdentifiers:)]
        unsafe fn toolbarImmovableItemIdentifiers(
            &self,
            toolbar: &NSToolbar,
        ) -> Retained<NSSet<NSToolbarItemIdentifier>>;

        #[optional]
        #[method(toolbar:itemIdentifier:canBeInsertedAtIndex:)]
        unsafe fn toolbar_itemIdentifier_canBeInsertedAtIndex(
            &self,
            toolbar: &NSToolbar,
            item_identifier: &NSToolbarItemIdentifier,
            index: NSInteger,
        ) -> bool;

        #[optional]
        #[method(toolbarWillAddItem:)]
        unsafe fn toolbarWillAddItem(&self, notification: &NSNotification);

        #[optional]
        #[method(toolbarDidRemoveItem:)]
        unsafe fn toolbarDidRemoveItem(&self, notification: &NSNotification);
    }

    unsafe impl ProtocolType for dyn NSToolbarDelegate {}
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstoolbarwilladditemnotification?language=objc)
    pub static NSToolbarWillAddItemNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstoolbardidremoveitemnotification?language=objc)
    pub static NSToolbarDidRemoveItemNotification: &'static NSNotificationName;
}

extern_methods!(
    /// NSDeprecated
    unsafe impl NSToolbar {
        #[deprecated = "NSToolbarSizeMode is no longer recommended and will be ignored in the future"]
        #[method(sizeMode)]
        pub unsafe fn sizeMode(&self) -> NSToolbarSizeMode;

        #[deprecated = "NSToolbarSizeMode is no longer recommended and will be ignored in the future"]
        #[method(setSizeMode:)]
        pub unsafe fn setSizeMode(&self, size_mode: NSToolbarSizeMode);

        #[deprecated = "Use the centeredItemIdentifiers property instead"]
        #[method_id(@__retain_semantics Other centeredItemIdentifier)]
        pub unsafe fn centeredItemIdentifier(&self) -> Option<Retained<NSToolbarItemIdentifier>>;

        #[deprecated = "Use the centeredItemIdentifiers property instead"]
        #[method(setCenteredItemIdentifier:)]
        pub unsafe fn setCenteredItemIdentifier(
            &self,
            centered_item_identifier: Option<&NSToolbarItemIdentifier>,
        );

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[deprecated = "Use NSTitlebarAccessoryViewController with NSWindow instead"]
        #[method_id(@__retain_semantics Other fullScreenAccessoryView)]
        pub unsafe fn fullScreenAccessoryView(&self) -> Option<Retained<NSView>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[deprecated = "Use NSTitlebarAccessoryViewController with NSWindow instead"]
        #[method(setFullScreenAccessoryView:)]
        pub unsafe fn setFullScreenAccessoryView(
            &self,
            full_screen_accessory_view: Option<&NSView>,
        );

        #[deprecated = "Use NSTitlebarAccessoryViewController and its fullScreenMinHeight property with NSWindow instead."]
        #[method(fullScreenAccessoryViewMinHeight)]
        pub unsafe fn fullScreenAccessoryViewMinHeight(&self) -> CGFloat;

        #[deprecated = "Use NSTitlebarAccessoryViewController and its fullScreenMinHeight property with NSWindow instead."]
        #[method(setFullScreenAccessoryViewMinHeight:)]
        pub unsafe fn setFullScreenAccessoryViewMinHeight(
            &self,
            full_screen_accessory_view_min_height: CGFloat,
        );

        #[deprecated = "Use NSTitlebarAccessoryViewController with NSWindow instead. The max height of a titlebar accessory is implied by its view's height."]
        #[method(fullScreenAccessoryViewMaxHeight)]
        pub unsafe fn fullScreenAccessoryViewMaxHeight(&self) -> CGFloat;

        #[deprecated = "Use NSTitlebarAccessoryViewController with NSWindow instead. The max height of a titlebar accessory is implied by its view's height."]
        #[method(setFullScreenAccessoryViewMaxHeight:)]
        pub unsafe fn setFullScreenAccessoryViewMaxHeight(
            &self,
            full_screen_accessory_view_max_height: CGFloat,
        );

        #[deprecated = "No longer supported"]
        #[method(showsBaselineSeparator)]
        pub unsafe fn showsBaselineSeparator(&self) -> bool;

        #[deprecated = "No longer supported"]
        #[method(setShowsBaselineSeparator:)]
        pub unsafe fn setShowsBaselineSeparator(&self, shows_baseline_separator: bool);

        #[deprecated = "Use -itemIdentifiers and -displayMode instead."]
        #[method_id(@__retain_semantics Other configurationDictionary)]
        pub unsafe fn configurationDictionary(&self)
            -> Retained<NSDictionary<NSString, AnyObject>>;

        #[deprecated = "Use -setItemIdentifiers: and -setDisplayMode: instead."]
        #[method(setConfigurationFromDictionary:)]
        pub unsafe fn setConfigurationFromDictionary(
            &self,
            config_dict: &NSDictionary<NSString, AnyObject>,
        );
    }
);

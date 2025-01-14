//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitycustomrotordirection?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIAccessibilityCustomRotorDirection(pub NSInteger);
impl UIAccessibilityCustomRotorDirection {
    #[doc(alias = "UIAccessibilityCustomRotorDirectionPrevious")]
    pub const Previous: Self = Self(0);
    #[doc(alias = "UIAccessibilityCustomRotorDirectionNext")]
    pub const Next: Self = Self(1);
}

unsafe impl Encode for UIAccessibilityCustomRotorDirection {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIAccessibilityCustomRotorDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitycustomsystemrotortype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIAccessibilityCustomSystemRotorType(pub NSInteger);
impl UIAccessibilityCustomSystemRotorType {
    #[doc(alias = "UIAccessibilityCustomSystemRotorTypeNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "UIAccessibilityCustomSystemRotorTypeLink")]
    pub const Link: Self = Self(1);
    #[doc(alias = "UIAccessibilityCustomSystemRotorTypeVisitedLink")]
    pub const VisitedLink: Self = Self(2);
    #[doc(alias = "UIAccessibilityCustomSystemRotorTypeHeading")]
    pub const Heading: Self = Self(3);
    #[doc(alias = "UIAccessibilityCustomSystemRotorTypeHeadingLevel1")]
    pub const HeadingLevel1: Self = Self(4);
    #[doc(alias = "UIAccessibilityCustomSystemRotorTypeHeadingLevel2")]
    pub const HeadingLevel2: Self = Self(5);
    #[doc(alias = "UIAccessibilityCustomSystemRotorTypeHeadingLevel3")]
    pub const HeadingLevel3: Self = Self(6);
    #[doc(alias = "UIAccessibilityCustomSystemRotorTypeHeadingLevel4")]
    pub const HeadingLevel4: Self = Self(7);
    #[doc(alias = "UIAccessibilityCustomSystemRotorTypeHeadingLevel5")]
    pub const HeadingLevel5: Self = Self(8);
    #[doc(alias = "UIAccessibilityCustomSystemRotorTypeHeadingLevel6")]
    pub const HeadingLevel6: Self = Self(9);
    #[doc(alias = "UIAccessibilityCustomSystemRotorTypeBoldText")]
    pub const BoldText: Self = Self(10);
    #[doc(alias = "UIAccessibilityCustomSystemRotorTypeItalicText")]
    pub const ItalicText: Self = Self(11);
    #[doc(alias = "UIAccessibilityCustomSystemRotorTypeUnderlineText")]
    pub const UnderlineText: Self = Self(12);
    #[doc(alias = "UIAccessibilityCustomSystemRotorTypeMisspelledWord")]
    pub const MisspelledWord: Self = Self(13);
    #[doc(alias = "UIAccessibilityCustomSystemRotorTypeImage")]
    pub const Image: Self = Self(14);
    #[doc(alias = "UIAccessibilityCustomSystemRotorTypeTextField")]
    pub const TextField: Self = Self(15);
    #[doc(alias = "UIAccessibilityCustomSystemRotorTypeTable")]
    pub const Table: Self = Self(16);
    #[doc(alias = "UIAccessibilityCustomSystemRotorTypeList")]
    pub const List: Self = Self(17);
    #[doc(alias = "UIAccessibilityCustomSystemRotorTypeLandmark")]
    pub const Landmark: Self = Self(18);
}

unsafe impl Encode for UIAccessibilityCustomSystemRotorType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIAccessibilityCustomSystemRotorType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitycustomrotorsearch?language=objc)
#[cfg(feature = "block2")]
pub type UIAccessibilityCustomRotorSearch = *mut block2::Block<
    dyn Fn(
        NonNull<UIAccessibilityCustomRotorSearchPredicate>,
    ) -> *mut UIAccessibilityCustomRotorItemResult,
>;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/axcustomrotorsreturnblock?language=objc)
#[cfg(feature = "block2")]
pub type AXCustomRotorsReturnBlock =
    *mut block2::Block<dyn Fn() -> *mut NSArray<UIAccessibilityCustomRotor>>;

extern_category!(
    /// Category "UIAccessibilityCustomRotor" on [`NSObject`].
    #[doc(alias = "UIAccessibilityCustomRotor")]
    pub unsafe trait NSObjectUIAccessibilityCustomRotor {
        #[method_id(@__retain_semantics Other accessibilityCustomRotors)]
        unsafe fn accessibilityCustomRotors(
            &self,
            mtm: MainThreadMarker,
        ) -> Option<Retained<NSArray<UIAccessibilityCustomRotor>>>;

        #[method(setAccessibilityCustomRotors:)]
        unsafe fn setAccessibilityCustomRotors(
            &self,
            accessibility_custom_rotors: Option<&NSArray<UIAccessibilityCustomRotor>>,
            mtm: MainThreadMarker,
        );

        #[cfg(feature = "block2")]
        #[method(accessibilityCustomRotorsBlock)]
        unsafe fn accessibilityCustomRotorsBlock(
            &self,
            mtm: MainThreadMarker,
        ) -> AXCustomRotorsReturnBlock;

        #[cfg(feature = "block2")]
        #[method(setAccessibilityCustomRotorsBlock:)]
        unsafe fn setAccessibilityCustomRotorsBlock(
            &self,
            accessibility_custom_rotors_block: AXCustomRotorsReturnBlock,
            mtm: MainThreadMarker,
        );
    }

    unsafe impl NSObjectUIAccessibilityCustomRotor for NSObject {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitycustomrotorsearchpredicate?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIAccessibilityCustomRotorSearchPredicate;
);

unsafe impl NSObjectProtocol for UIAccessibilityCustomRotorSearchPredicate {}

extern_methods!(
    unsafe impl UIAccessibilityCustomRotorSearchPredicate {
        #[method_id(@__retain_semantics Other currentItem)]
        pub unsafe fn currentItem(&self) -> Retained<UIAccessibilityCustomRotorItemResult>;

        #[method(setCurrentItem:)]
        pub unsafe fn setCurrentItem(&self, current_item: &UIAccessibilityCustomRotorItemResult);

        #[method(searchDirection)]
        pub unsafe fn searchDirection(&self) -> UIAccessibilityCustomRotorDirection;

        #[method(setSearchDirection:)]
        pub unsafe fn setSearchDirection(
            &self,
            search_direction: UIAccessibilityCustomRotorDirection,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIAccessibilityCustomRotorSearchPredicate {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitycustomrotor?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIAccessibilityCustomRotor;
);

unsafe impl NSObjectProtocol for UIAccessibilityCustomRotor {}

extern_methods!(
    unsafe impl UIAccessibilityCustomRotor {
        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithName:itemSearchBlock:)]
        pub unsafe fn initWithName_itemSearchBlock(
            this: Allocated<Self>,
            name: &NSString,
            item_search_block: UIAccessibilityCustomRotorSearch,
        ) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithAttributedName:itemSearchBlock:)]
        pub unsafe fn initWithAttributedName_itemSearchBlock(
            this: Allocated<Self>,
            attributed_name: &NSAttributedString,
            item_search_block: UIAccessibilityCustomRotorSearch,
        ) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithSystemType:itemSearchBlock:)]
        pub unsafe fn initWithSystemType_itemSearchBlock(
            this: Allocated<Self>,
            r#type: UIAccessibilityCustomSystemRotorType,
            item_search_block: UIAccessibilityCustomRotorSearch,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[method(setName:)]
        pub unsafe fn setName(&self, name: &NSString);

        #[method_id(@__retain_semantics Other attributedName)]
        pub unsafe fn attributedName(&self) -> Retained<NSAttributedString>;

        #[method(setAttributedName:)]
        pub unsafe fn setAttributedName(&self, attributed_name: &NSAttributedString);

        #[cfg(feature = "block2")]
        #[method(itemSearchBlock)]
        pub unsafe fn itemSearchBlock(&self) -> UIAccessibilityCustomRotorSearch;

        #[cfg(feature = "block2")]
        #[method(setItemSearchBlock:)]
        pub unsafe fn setItemSearchBlock(
            &self,
            item_search_block: UIAccessibilityCustomRotorSearch,
        );

        #[method(systemRotorType)]
        pub unsafe fn systemRotorType(&self) -> UIAccessibilityCustomSystemRotorType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIAccessibilityCustomRotor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitycustomrotoritemresult?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIAccessibilityCustomRotorItemResult;
);

unsafe impl NSObjectProtocol for UIAccessibilityCustomRotorItemResult {}

extern_methods!(
    unsafe impl UIAccessibilityCustomRotorItemResult {
        #[cfg(feature = "UITextInput")]
        #[method_id(@__retain_semantics Init initWithTargetElement:targetRange:)]
        pub unsafe fn initWithTargetElement_targetRange(
            this: Allocated<Self>,
            target_element: &ProtocolObject<dyn NSObjectProtocol>,
            target_range: Option<&UITextRange>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other targetElement)]
        pub unsafe fn targetElement(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSObjectProtocol>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setTargetElement:)]
        pub unsafe fn setTargetElement(
            &self,
            target_element: Option<&ProtocolObject<dyn NSObjectProtocol>>,
        );

        #[cfg(feature = "UITextInput")]
        #[method_id(@__retain_semantics Other targetRange)]
        pub unsafe fn targetRange(&self) -> Option<Retained<UITextRange>>;

        #[cfg(feature = "UITextInput")]
        #[method(setTargetRange:)]
        pub unsafe fn setTargetRange(&self, target_range: Option<&UITextRange>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIAccessibilityCustomRotorItemResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

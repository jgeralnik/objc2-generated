//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    #[deprecated]
    pub struct DOMCSSStyleDeclaration;

    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl ClassType for DOMCSSStyleDeclaration {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSCopying for DOMCSSStyleDeclaration {}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl CopyingHelper for DOMCSSStyleDeclaration {
    type Result = Self;
}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSObjectProtocol for DOMCSSStyleDeclaration {}

extern_methods!(
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMCSSStyleDeclaration {
        #[deprecated]
        #[method_id(@__retain_semantics Other cssText)]
        pub unsafe fn cssText(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setCssText:)]
        pub unsafe fn setCssText(&self, css_text: Option<&NSString>);

        #[deprecated]
        #[method(length)]
        pub unsafe fn length(&self) -> c_uint;

        #[cfg(feature = "DOMCSSRule")]
        #[deprecated]
        #[method_id(@__retain_semantics Other parentRule)]
        pub unsafe fn parentRule(&self) -> Option<Retained<DOMCSSRule>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other getPropertyValue:)]
        pub unsafe fn getPropertyValue(
            &self,
            property_name: Option<&NSString>,
        ) -> Option<Retained<NSString>>;

        #[cfg(feature = "DOMCSSValue")]
        #[deprecated]
        #[method_id(@__retain_semantics Other getPropertyCSSValue:)]
        pub unsafe fn getPropertyCSSValue(
            &self,
            property_name: Option<&NSString>,
        ) -> Option<Retained<DOMCSSValue>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other removeProperty:)]
        pub unsafe fn removeProperty(
            &self,
            property_name: Option<&NSString>,
        ) -> Option<Retained<NSString>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other getPropertyPriority:)]
        pub unsafe fn getPropertyPriority(
            &self,
            property_name: Option<&NSString>,
        ) -> Option<Retained<NSString>>;

        #[method(setProperty:value:priority:)]
        pub unsafe fn setProperty_value_priority(
            &self,
            property_name: Option<&NSString>,
            value: Option<&NSString>,
            priority: Option<&NSString>,
        );

        #[deprecated]
        #[method_id(@__retain_semantics Other item:)]
        pub unsafe fn item(&self, index: c_uint) -> Option<Retained<NSString>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other getPropertyShorthand:)]
        pub unsafe fn getPropertyShorthand(
            &self,
            property_name: Option<&NSString>,
        ) -> Option<Retained<NSString>>;

        #[method(isPropertyImplicit:)]
        pub unsafe fn isPropertyImplicit(&self, property_name: Option<&NSString>) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMCSSStyleDeclaration {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMCSSStyleDeclaration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// DOMCSSStyleDeclarationDeprecated
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMCSSStyleDeclaration {
        #[deprecated]
        #[method(setProperty:::)]
        pub unsafe fn setProperty(
            &self,
            property_name: Option<&NSString>,
            value: Option<&NSString>,
            priority: Option<&NSString>,
        );
    }
);

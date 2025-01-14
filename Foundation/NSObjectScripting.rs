//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_category!(
    /// Category "NSScripting" on [`NSObject`].
    #[doc(alias = "NSScripting")]
    pub unsafe trait NSObjectNSScripting {
        #[cfg(feature = "NSScriptObjectSpecifiers")]
        #[method_id(@__retain_semantics Other scriptingValueForSpecifier:)]
        unsafe fn scriptingValueForSpecifier(
            &self,
            object_specifier: &NSScriptObjectSpecifier,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other scriptingProperties)]
        unsafe fn scriptingProperties(&self)
            -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method(setScriptingProperties:)]
        unsafe fn setScriptingProperties(
            &self,
            scripting_properties: Option<&NSDictionary<NSString, AnyObject>>,
        );

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Copy copyScriptingValue:forKey:withProperties:)]
        unsafe fn copyScriptingValue_forKey_withProperties(
            &self,
            value: &AnyObject,
            key: &NSString,
            properties: &NSDictionary<NSString, AnyObject>,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics New newScriptingObjectOfClass:forValueForKey:withContentsValue:properties:)]
        unsafe fn newScriptingObjectOfClass_forValueForKey_withContentsValue_properties(
            &self,
            object_class: &AnyClass,
            key: &NSString,
            contents_value: Option<&AnyObject>,
            properties: &NSDictionary<NSString, AnyObject>,
        ) -> Option<Retained<AnyObject>>;
    }

    unsafe impl NSObjectNSScripting for NSObject {}
);

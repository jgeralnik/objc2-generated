//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSXMLNode")]
    pub struct NSXMLDTD;

    #[cfg(feature = "NSXMLNode")]
    unsafe impl ClassType for NSXMLDTD {
        #[inherits(NSObject)]
        type Super = NSXMLNode;
    }
);

#[cfg(all(feature = "NSObject", feature = "NSXMLNode"))]
unsafe impl NSCopying for NSXMLDTD {}

#[cfg(all(feature = "NSObject", feature = "NSXMLNode"))]
unsafe impl CopyingHelper for NSXMLDTD {
    type Result = Self;
}

#[cfg(feature = "NSXMLNode")]
unsafe impl NSObjectProtocol for NSXMLDTD {}

extern_methods!(
    #[cfg(feature = "NSXMLNode")]
    unsafe impl NSXMLDTD {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "NSXMLNodeOptions")]
        #[method_id(@__retain_semantics Init initWithKind:options:)]
        pub unsafe fn initWithKind_options(
            this: Allocated<Self>,
            kind: NSXMLNodeKind,
            options: NSXMLNodeOptions,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSError", feature = "NSURL", feature = "NSXMLNodeOptions"))]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:options:error:_)]
        pub unsafe fn initWithContentsOfURL_options_error(
            this: Allocated<Self>,
            url: &NSURL,
            mask: NSXMLNodeOptions,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(all(feature = "NSData", feature = "NSError", feature = "NSXMLNodeOptions"))]
        #[method_id(@__retain_semantics Init initWithData:options:error:_)]
        pub unsafe fn initWithData_options_error(
            this: Allocated<Self>,
            data: &NSData,
            mask: NSXMLNodeOptions,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other publicID)]
        pub unsafe fn publicID(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method(setPublicID:)]
        pub unsafe fn setPublicID(&self, public_id: Option<&NSString>);

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other systemID)]
        pub unsafe fn systemID(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method(setSystemID:)]
        pub unsafe fn setSystemID(&self, system_id: Option<&NSString>);

        #[method(insertChild:atIndex:)]
        pub unsafe fn insertChild_atIndex(&self, child: &NSXMLNode, index: NSUInteger);

        #[cfg(feature = "NSArray")]
        #[method(insertChildren:atIndex:)]
        pub unsafe fn insertChildren_atIndex(
            &self,
            children: &NSArray<NSXMLNode>,
            index: NSUInteger,
        );

        #[method(removeChildAtIndex:)]
        pub unsafe fn removeChildAtIndex(&self, index: NSUInteger);

        #[cfg(feature = "NSArray")]
        #[method(setChildren:)]
        pub unsafe fn setChildren(&self, children: Option<&NSArray<NSXMLNode>>);

        #[method(addChild:)]
        pub unsafe fn addChild(&self, child: &NSXMLNode);

        #[method(replaceChildAtIndex:withNode:)]
        pub unsafe fn replaceChildAtIndex_withNode(&self, index: NSUInteger, node: &NSXMLNode);

        #[cfg(all(feature = "NSString", feature = "NSXMLDTDNode"))]
        #[method_id(@__retain_semantics Other entityDeclarationForName:)]
        pub unsafe fn entityDeclarationForName(
            &self,
            name: &NSString,
        ) -> Option<Retained<NSXMLDTDNode>>;

        #[cfg(all(feature = "NSString", feature = "NSXMLDTDNode"))]
        #[method_id(@__retain_semantics Other notationDeclarationForName:)]
        pub unsafe fn notationDeclarationForName(
            &self,
            name: &NSString,
        ) -> Option<Retained<NSXMLDTDNode>>;

        #[cfg(all(feature = "NSString", feature = "NSXMLDTDNode"))]
        #[method_id(@__retain_semantics Other elementDeclarationForName:)]
        pub unsafe fn elementDeclarationForName(
            &self,
            name: &NSString,
        ) -> Option<Retained<NSXMLDTDNode>>;

        #[cfg(all(feature = "NSString", feature = "NSXMLDTDNode"))]
        #[method_id(@__retain_semantics Other attributeDeclarationForName:elementName:)]
        pub unsafe fn attributeDeclarationForName_elementName(
            &self,
            name: &NSString,
            element_name: &NSString,
        ) -> Option<Retained<NSXMLDTDNode>>;

        #[cfg(all(feature = "NSString", feature = "NSXMLDTDNode"))]
        #[method_id(@__retain_semantics Other predefinedEntityDeclarationForName:)]
        pub unsafe fn predefinedEntityDeclarationForName(
            name: &NSString,
        ) -> Option<Retained<NSXMLDTDNode>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSXMLNode`
    #[cfg(feature = "NSXMLNode")]
    unsafe impl NSXMLDTD {
        #[method_id(@__retain_semantics Init initWithKind:)]
        pub unsafe fn initWithKind(this: Allocated<Self>, kind: NSXMLNodeKind) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSXMLNode")]
    unsafe impl NSXMLDTD {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSXMLNodeOptions {
        #[doc(alias = "NSXMLNodeOptionsNone")]
        None = 0,
        NSXMLNodeIsCDATA = 1 << 0,
        NSXMLNodeExpandEmptyElement = 1 << 1,
        NSXMLNodeCompactEmptyElement = 1 << 2,
        NSXMLNodeUseSingleQuotes = 1 << 3,
        NSXMLNodeUseDoubleQuotes = 1 << 4,
        NSXMLNodeNeverEscapeContents = 1 << 5,
        NSXMLDocumentTidyHTML = 1 << 9,
        NSXMLDocumentTidyXML = 1 << 10,
        NSXMLDocumentValidate = 1 << 13,
        NSXMLNodeLoadExternalEntitiesAlways = 1 << 14,
        NSXMLNodeLoadExternalEntitiesSameOriginOnly = 1 << 15,
        NSXMLNodeLoadExternalEntitiesNever = 1 << 19,
        NSXMLDocumentXInclude = 1 << 16,
        NSXMLNodePrettyPrint = 1 << 17,
        NSXMLDocumentIncludeContentTypeDeclaration = 1 << 18,
        NSXMLNodePreserveNamespaceOrder = 1 << 20,
        NSXMLNodePreserveAttributeOrder = 1 << 21,
        NSXMLNodePreserveEntities = 1 << 22,
        NSXMLNodePreservePrefixes = 1 << 23,
        NSXMLNodePreserveCDATA = 1 << 24,
        NSXMLNodePreserveWhitespace = 1 << 25,
        NSXMLNodePreserveDTD = 1 << 26,
        NSXMLNodePreserveCharacterReferences = 1 << 27,
        NSXMLNodePromoteSignificantWhitespace = 1 << 28,
        NSXMLNodePreserveEmptyElements = NSXMLNodeOptions::NSXMLNodeExpandEmptyElement.0
            | NSXMLNodeOptions::NSXMLNodeCompactEmptyElement.0,
        NSXMLNodePreserveQuotes = NSXMLNodeOptions::NSXMLNodeUseSingleQuotes.0
            | NSXMLNodeOptions::NSXMLNodeUseDoubleQuotes.0,
        NSXMLNodePreserveAll = NSXMLNodeOptions::NSXMLNodePreserveNamespaceOrder.0
            | NSXMLNodeOptions::NSXMLNodePreserveAttributeOrder.0
            | NSXMLNodeOptions::NSXMLNodePreserveEntities.0
            | NSXMLNodeOptions::NSXMLNodePreservePrefixes.0
            | NSXMLNodeOptions::NSXMLNodePreserveCDATA.0
            | NSXMLNodeOptions::NSXMLNodePreserveEmptyElements.0
            | NSXMLNodeOptions::NSXMLNodePreserveQuotes.0
            | NSXMLNodeOptions::NSXMLNodePreserveWhitespace.0
            | NSXMLNodeOptions::NSXMLNodePreserveDTD.0
            | NSXMLNodeOptions::NSXMLNodePreserveCharacterReferences.0
            | 0xFFF00000,
    }
);

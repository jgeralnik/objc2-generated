//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextCheckingController;

    unsafe impl ClassType for NSTextCheckingController {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTextCheckingController")]
    unsafe impl NSTextCheckingController {
        #[cfg(feature = "AppKit_NSTextCheckingClient")]
        #[method_id(@__retain_semantics Init initWithClient:)]
        pub unsafe fn initWithClient(
            this: Option<Allocated<Self>>,
            client: &NSTextCheckingClient,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSTextCheckingClient")]
        #[method_id(@__retain_semantics Other client)]
        pub unsafe fn client(&self) -> Id<NSTextCheckingClient, Shared>;

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[method(didChangeTextInRange:)]
        pub unsafe fn didChangeTextInRange(&self, range: NSRange);

        #[method(insertedTextInRange:)]
        pub unsafe fn insertedTextInRange(&self, range: NSRange);

        #[method(didChangeSelectedRange)]
        pub unsafe fn didChangeSelectedRange(&self);

        #[method(considerTextCheckingForRange:)]
        pub unsafe fn considerTextCheckingForRange(&self, range: NSRange);

        #[cfg(feature = "AppKit_NSTextCheckingOptionKey")]
        #[method(checkTextInRange:types:options:)]
        pub unsafe fn checkTextInRange_types_options(
            &self,
            range: NSRange,
            checkingTypes: NSTextCheckingTypes,
            options: &NSDictionary<NSTextCheckingOptionKey, Object>,
        );

        #[method(checkTextInSelection:)]
        pub unsafe fn checkTextInSelection(&self, sender: Option<&Object>);

        #[method(checkTextInDocument:)]
        pub unsafe fn checkTextInDocument(&self, sender: Option<&Object>);

        #[method(orderFrontSubstitutionsPanel:)]
        pub unsafe fn orderFrontSubstitutionsPanel(&self, sender: Option<&Object>);

        #[method(checkSpelling:)]
        pub unsafe fn checkSpelling(&self, sender: Option<&Object>);

        #[method(showGuessPanel:)]
        pub unsafe fn showGuessPanel(&self, sender: Option<&Object>);

        #[method(changeSpelling:)]
        pub unsafe fn changeSpelling(&self, sender: Option<&Object>);

        #[method(ignoreSpelling:)]
        pub unsafe fn ignoreSpelling(&self, sender: Option<&Object>);

        #[method(updateCandidates)]
        pub unsafe fn updateCandidates(&self);

        #[method_id(@__retain_semantics Other validAnnotations)]
        pub unsafe fn validAnnotations(&self) -> Id<NSArray<NSAttributedStringKey>, Shared>;

        #[cfg(feature = "AppKit_NSMenu")]
        #[method_id(@__retain_semantics Other menuAtIndex:clickedOnSelection:effectiveRange:)]
        pub unsafe fn menuAtIndex_clickedOnSelection_effectiveRange(
            &self,
            location: NSUInteger,
            clickedOnSelection: bool,
            effectiveRange: NSRangePointer,
        ) -> Option<Id<NSMenu, Shared>>;

        #[method(spellCheckerDocumentTag)]
        pub unsafe fn spellCheckerDocumentTag(&self) -> NSInteger;

        #[method(setSpellCheckerDocumentTag:)]
        pub unsafe fn setSpellCheckerDocumentTag(&self, spellCheckerDocumentTag: NSInteger);
    }
);

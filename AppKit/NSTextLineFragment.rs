//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextLineFragment;

    unsafe impl ClassType for NSTextLineFragment {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTextLineFragment")]
    unsafe impl NSTextLineFragment {
        #[method_id(@__retain_semantics Init initWithAttributedString:range:)]
        pub unsafe fn initWithAttributedString_range(
            this: Option<Allocated<Self>>,
            attributedString: &NSAttributedString,
            range: NSRange,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            aDecoder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithString:attributes:range:)]
        pub unsafe fn initWithString_attributes_range(
            this: Option<Allocated<Self>>,
            string: &NSString,
            attributes: &NSDictionary<NSAttributedStringKey, Object>,
            range: NSRange,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other attributedString)]
        pub unsafe fn attributedString(&self) -> Id<NSAttributedString, Shared>;

        #[method(characterRange)]
        pub unsafe fn characterRange(&self) -> NSRange;

        #[method(typographicBounds)]
        pub unsafe fn typographicBounds(&self) -> CGRect;

        #[method(glyphOrigin)]
        pub unsafe fn glyphOrigin(&self) -> CGPoint;

        #[method(locationForCharacterAtIndex:)]
        pub unsafe fn locationForCharacterAtIndex(&self, index: NSInteger) -> CGPoint;

        #[method(characterIndexForPoint:)]
        pub unsafe fn characterIndexForPoint(&self, point: CGPoint) -> NSInteger;

        #[method(fractionOfDistanceThroughGlyphForPoint:)]
        pub unsafe fn fractionOfDistanceThroughGlyphForPoint(&self, point: CGPoint) -> CGFloat;
    }
);

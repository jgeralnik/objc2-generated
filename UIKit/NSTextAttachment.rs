//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

pub const NSAttachmentCharacter: c_uint = 0xFFFC;

extern_protocol!(
    pub unsafe trait NSTextAttachmentLayout: NSObjectProtocol {
        #[cfg(all(
            feature = "NSTextContainer",
            feature = "NSTextRange",
            feature = "UIImage"
        ))]
        #[method_id(@__retain_semantics Other imageForBounds:attributes:location:textContainer:)]
        unsafe fn imageForBounds_attributes_location_textContainer(
            &self,
            bounds: CGRect,
            attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
            location: &ProtocolObject<dyn NSTextLocation>,
            text_container: Option<&NSTextContainer>,
        ) -> Option<Retained<UIImage>>;

        #[cfg(all(feature = "NSTextContainer", feature = "NSTextRange"))]
        #[method(attachmentBoundsForAttributes:location:textContainer:proposedLineFragment:position:)]
        unsafe fn attachmentBoundsForAttributes_location_textContainer_proposedLineFragment_position(
            &self,
            attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
            location: &ProtocolObject<dyn NSTextLocation>,
            text_container: Option<&NSTextContainer>,
            proposed_line_fragment: CGRect,
            position: CGPoint,
        ) -> CGRect;

        #[cfg(all(
            feature = "NSTextContainer",
            feature = "NSTextRange",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        #[method_id(@__retain_semantics Other viewProviderForParentView:location:textContainer:)]
        unsafe fn viewProviderForParentView_location_textContainer(
            &self,
            parent_view: Option<&UIView>,
            location: &ProtocolObject<dyn NSTextLocation>,
            text_container: Option<&NSTextContainer>,
        ) -> Option<Retained<NSTextAttachmentViewProvider>>;
    }

    unsafe impl ProtocolType for dyn NSTextAttachmentLayout {}
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextAttachment;
);

unsafe impl NSCoding for NSTextAttachment {}

unsafe impl NSObjectProtocol for NSTextAttachment {}

unsafe impl NSSecureCoding for NSTextAttachment {}

unsafe impl NSTextAttachmentLayout for NSTextAttachment {}

extern_methods!(
    unsafe impl NSTextAttachment {
        #[method_id(@__retain_semantics Init initWithData:ofType:)]
        pub unsafe fn initWithData_ofType(
            this: Allocated<Self>,
            content_data: Option<&NSData>,
            uti: Option<&NSString>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other contents)]
        pub unsafe fn contents(&self) -> Option<Retained<NSData>>;

        #[method(setContents:)]
        pub unsafe fn setContents(&self, contents: Option<&NSData>);

        #[method_id(@__retain_semantics Other fileType)]
        pub unsafe fn fileType(&self) -> Option<Retained<NSString>>;

        #[method(setFileType:)]
        pub unsafe fn setFileType(&self, file_type: Option<&NSString>);

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&UIImage>);

        #[method(bounds)]
        pub unsafe fn bounds(&self) -> CGRect;

        #[method(setBounds:)]
        pub unsafe fn setBounds(&self, bounds: CGRect);

        #[method_id(@__retain_semantics Other fileWrapper)]
        pub unsafe fn fileWrapper(&self) -> Option<Retained<NSFileWrapper>>;

        #[method(setFileWrapper:)]
        pub unsafe fn setFileWrapper(&self, file_wrapper: Option<&NSFileWrapper>);

        #[method(lineLayoutPadding)]
        pub unsafe fn lineLayoutPadding(&self) -> CGFloat;

        #[method(setLineLayoutPadding:)]
        pub unsafe fn setLineLayoutPadding(&self, line_layout_padding: CGFloat);

        #[method(textAttachmentViewProviderClassForFileType:)]
        pub unsafe fn textAttachmentViewProviderClassForFileType(
            file_type: &NSString,
        ) -> Option<&'static AnyClass>;

        #[method(registerTextAttachmentViewProviderClass:forFileType:)]
        pub unsafe fn registerTextAttachmentViewProviderClass_forFileType(
            text_attachment_view_provider_class: &AnyClass,
            file_type: &NSString,
        );

        #[method(allowsTextAttachmentView)]
        pub unsafe fn allowsTextAttachmentView(&self) -> bool;

        #[method(setAllowsTextAttachmentView:)]
        pub unsafe fn setAllowsTextAttachmentView(&self, allows_text_attachment_view: bool);

        #[method(usesTextAttachmentView)]
        pub unsafe fn usesTextAttachmentView(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTextAttachment {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_category!(
    /// Category on [`NSAttributedString`].
    pub unsafe trait NSAttributedStringAttachmentConveniences {
        #[method_id(@__retain_semantics Other attributedStringWithAttachment:)]
        unsafe fn attributedStringWithAttachment(
            attachment: &NSTextAttachment,
        ) -> Retained<NSAttributedString>;

        #[method_id(@__retain_semantics Other attributedStringWithAttachment:attributes:)]
        unsafe fn attributedStringWithAttachment_attributes(
            attachment: &NSTextAttachment,
            attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
        ) -> Retained<Self>;
    }

    unsafe impl NSAttributedStringAttachmentConveniences for NSAttributedString {}
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextAttachmentViewProvider;
);

unsafe impl NSObjectProtocol for NSTextAttachmentViewProvider {}

extern_methods!(
    unsafe impl NSTextAttachmentViewProvider {
        #[cfg(all(
            feature = "NSTextLayoutManager",
            feature = "NSTextRange",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        #[method_id(@__retain_semantics Init initWithTextAttachment:parentView:textLayoutManager:location:)]
        pub unsafe fn initWithTextAttachment_parentView_textLayoutManager_location(
            this: Allocated<Self>,
            text_attachment: &NSTextAttachment,
            parent_view: Option<&UIView>,
            text_layout_manager: Option<&NSTextLayoutManager>,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other textAttachment)]
        pub unsafe fn textAttachment(&self) -> Option<Retained<NSTextAttachment>>;

        #[cfg(feature = "NSTextLayoutManager")]
        #[method_id(@__retain_semantics Other textLayoutManager)]
        pub unsafe fn textLayoutManager(&self) -> Option<Retained<NSTextLayoutManager>>;

        #[cfg(feature = "NSTextRange")]
        #[method_id(@__retain_semantics Other location)]
        pub unsafe fn location(&self) -> Retained<ProtocolObject<dyn NSTextLocation>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self, mtm: MainThreadMarker) -> Option<Retained<UIView>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method(setView:)]
        pub unsafe fn setView(&self, view: Option<&UIView>);

        #[method(loadView)]
        pub unsafe fn loadView(&self);

        #[method(tracksTextAttachmentViewBounds)]
        pub unsafe fn tracksTextAttachmentViewBounds(&self) -> bool;

        #[method(setTracksTextAttachmentViewBounds:)]
        pub unsafe fn setTracksTextAttachmentViewBounds(
            &self,
            tracks_text_attachment_view_bounds: bool,
        );

        #[cfg(all(feature = "NSTextContainer", feature = "NSTextRange"))]
        #[method(attachmentBoundsForAttributes:location:textContainer:proposedLineFragment:position:)]
        pub unsafe fn attachmentBoundsForAttributes_location_textContainer_proposedLineFragment_position(
            &self,
            attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
            location: &ProtocolObject<dyn NSTextLocation>,
            text_container: Option<&NSTextContainer>,
            proposed_line_fragment: CGRect,
            position: CGPoint,
        ) -> CGRect;
    }
);

extern_protocol!(
    pub unsafe trait NSTextAttachmentContainer: NSObjectProtocol {
        #[cfg(all(feature = "NSTextContainer", feature = "UIImage"))]
        #[method_id(@__retain_semantics Other imageForBounds:textContainer:characterIndex:)]
        unsafe fn imageForBounds_textContainer_characterIndex(
            &self,
            image_bounds: CGRect,
            text_container: Option<&NSTextContainer>,
            char_index: NSUInteger,
        ) -> Option<Retained<UIImage>>;

        #[cfg(feature = "NSTextContainer")]
        #[method(attachmentBoundsForTextContainer:proposedLineFragment:glyphPosition:characterIndex:)]
        unsafe fn attachmentBoundsForTextContainer_proposedLineFragment_glyphPosition_characterIndex(
            &self,
            text_container: Option<&NSTextContainer>,
            line_frag: CGRect,
            position: CGPoint,
            char_index: NSUInteger,
        ) -> CGRect;
    }

    unsafe impl ProtocolType for dyn NSTextAttachmentContainer {}
);

extern_methods!(
    /// NSTextAttachment_Deprecation
    unsafe impl NSTextAttachment {}
);

unsafe impl NSTextAttachmentContainer for NSTextAttachment {}

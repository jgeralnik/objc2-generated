//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photosui/phprojecttypedescription?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHProjectTypeDescription;
);

unsafe impl Send for PHProjectTypeDescription {}

unsafe impl Sync for PHProjectTypeDescription {}

unsafe impl NSCoding for PHProjectTypeDescription {}

unsafe impl NSObjectProtocol for PHProjectTypeDescription {}

unsafe impl NSSecureCoding for PHProjectTypeDescription {}

extern_methods!(
    unsafe impl PHProjectTypeDescription {
        #[cfg(feature = "PhotosUITypes")]
        #[method_id(@__retain_semantics Other projectType)]
        pub unsafe fn projectType(&self) -> Retained<PHProjectType>;

        #[method_id(@__retain_semantics Other localizedTitle)]
        pub unsafe fn localizedTitle(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other localizedDescription)]
        pub unsafe fn localizedDescription(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other localizedAttributedDescription)]
        pub unsafe fn localizedAttributedDescription(&self)
            -> Option<Retained<NSAttributedString>>;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Retained<NSImage>>;

        #[method_id(@__retain_semantics Other subtypeDescriptions)]
        pub unsafe fn subtypeDescriptions(&self) -> Retained<NSArray<PHProjectTypeDescription>>;

        #[method(canProvideSubtypes)]
        pub unsafe fn canProvideSubtypes(&self) -> bool;

        #[cfg(all(feature = "PhotosUITypes", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Init initWithProjectType:title:description:image:subtypeDescriptions:)]
        pub unsafe fn initWithProjectType_title_description_image_subtypeDescriptions(
            this: Allocated<Self>,
            project_type: &PHProjectType,
            localized_title: &NSString,
            localized_description: Option<&NSString>,
            image: Option<&NSImage>,
            subtype_descriptions: &NSArray<PHProjectTypeDescription>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "PhotosUITypes", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Init initWithProjectType:title:attributedDescription:image:subtypeDescriptions:)]
        pub unsafe fn initWithProjectType_title_attributedDescription_image_subtypeDescriptions(
            this: Allocated<Self>,
            project_type: &PHProjectType,
            localized_title: &NSString,
            localized_attributed_description: Option<&NSAttributedString>,
            image: Option<&NSImage>,
            subtype_descriptions: &NSArray<PHProjectTypeDescription>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "PhotosUITypes", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Init initWithProjectType:title:description:image:)]
        pub unsafe fn initWithProjectType_title_description_image(
            this: Allocated<Self>,
            project_type: &PHProjectType,
            localized_title: &NSString,
            localized_description: Option<&NSString>,
            image: Option<&NSImage>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "PhotosUITypes", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Init initWithProjectType:title:description:image:canProvideSubtypes:)]
        pub unsafe fn initWithProjectType_title_description_image_canProvideSubtypes(
            this: Allocated<Self>,
            project_type: &PHProjectType,
            localized_title: &NSString,
            localized_description: Option<&NSString>,
            image: Option<&NSImage>,
            can_provide_subtypes: bool,
        ) -> Retained<Self>;

        #[cfg(all(feature = "PhotosUITypes", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Init initWithProjectType:title:attributedDescription:image:canProvideSubtypes:)]
        pub unsafe fn initWithProjectType_title_attributedDescription_image_canProvideSubtypes(
            this: Allocated<Self>,
            project_type: &PHProjectType,
            localized_title: &NSString,
            localized_attributed_description: Option<&NSAttributedString>,
            image: Option<&NSImage>,
            can_provide_subtypes: bool,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait PHProjectTypeDescriptionDataSource: NSObjectProtocol {
        #[cfg(all(feature = "PHProjectTypeDescription", feature = "PhotosUITypes"))]
        #[method_id(@__retain_semantics Other subtypesForProjectType:)]
        unsafe fn subtypesForProjectType(
            &self,
            project_type: &PHProjectType,
        ) -> Retained<NSArray<PHProjectTypeDescription>>;

        #[cfg(all(feature = "PHProjectTypeDescription", feature = "PhotosUITypes"))]
        #[method_id(@__retain_semantics Other typeDescriptionForProjectType:)]
        unsafe fn typeDescriptionForProjectType(
            &self,
            project_type: &PHProjectType,
        ) -> Option<Retained<PHProjectTypeDescription>>;

        #[cfg(feature = "PhotosUITypes")]
        #[method_id(@__retain_semantics Other footerTextForSubtypesOfProjectType:)]
        unsafe fn footerTextForSubtypesOfProjectType(
            &self,
            project_type: &PHProjectType,
        ) -> Option<Retained<NSAttributedString>>;

        #[optional]
        #[method(extensionWillDiscardDataSource)]
        unsafe fn extensionWillDiscardDataSource(&self);
    }

    unsafe impl ProtocolType for dyn PHProjectTypeDescriptionDataSource {}
);

extern_protocol!(
    pub unsafe trait PHProjectTypeDescriptionInvalidator: NSObjectProtocol {
        #[cfg(feature = "PhotosUITypes")]
        #[method(invalidateTypeDescriptionForProjectType:)]
        unsafe fn invalidateTypeDescriptionForProjectType(&self, project_type: &PHProjectType);

        #[cfg(feature = "PhotosUITypes")]
        #[method(invalidateFooterTextForSubtypesOfProjectType:)]
        unsafe fn invalidateFooterTextForSubtypesOfProjectType(&self, project_type: &PHProjectType);
    }

    unsafe impl ProtocolType for dyn PHProjectTypeDescriptionInvalidator {}
);
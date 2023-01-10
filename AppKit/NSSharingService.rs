//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

typed_extensible_enum!(
    pub type NSSharingServiceName = NSString;
);

extern_static!(NSSharingServiceNameComposeEmail: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameComposeMessage: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameSendViaAirDrop: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameAddToSafariReadingList: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameAddToIPhoto: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameAddToAperture: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameUseAsDesktopPicture: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostOnFacebook: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostOnTwitter: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostOnSinaWeibo: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostOnTencentWeibo: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostOnLinkedIn: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameUseAsTwitterProfileImage: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameUseAsFacebookProfileImage: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameUseAsLinkedInProfileImage: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostImageOnFlickr: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostVideoOnVimeo: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostVideoOnYouku: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostVideoOnTudou: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameCloudSharing: &'static NSSharingServiceName);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSharingService;

    unsafe impl ClassType for NSSharingService {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSSharingService")]
    unsafe impl NSSharingService {
        #[cfg(feature = "AppKit_NSSharingServiceDelegate")]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSSharingServiceDelegate, Shared>>;

        #[cfg(feature = "AppKit_NSSharingServiceDelegate")]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSSharingServiceDelegate>);

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Id<NSImage, Shared>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other alternateImage)]
        pub unsafe fn alternateImage(&self) -> Option<Id<NSImage, Shared>>;

        #[method_id(@__retain_semantics Other menuItemTitle)]
        pub unsafe fn menuItemTitle(&self) -> Id<NSString, Shared>;

        #[method(setMenuItemTitle:)]
        pub unsafe fn setMenuItemTitle(&self, menuItemTitle: &NSString);

        #[method_id(@__retain_semantics Other recipients)]
        pub unsafe fn recipients(&self) -> Option<Id<NSArray<NSString>, Shared>>;

        #[method(setRecipients:)]
        pub unsafe fn setRecipients(&self, recipients: Option<&NSArray<NSString>>);

        #[method_id(@__retain_semantics Other subject)]
        pub unsafe fn subject(&self) -> Option<Id<NSString, Shared>>;

        #[method(setSubject:)]
        pub unsafe fn setSubject(&self, subject: Option<&NSString>);

        #[method_id(@__retain_semantics Other messageBody)]
        pub unsafe fn messageBody(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other permanentLink)]
        pub unsafe fn permanentLink(&self) -> Option<Id<NSURL, Shared>>;

        #[method_id(@__retain_semantics Other accountName)]
        pub unsafe fn accountName(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other attachmentFileURLs)]
        pub unsafe fn attachmentFileURLs(&self) -> Option<Id<NSArray<NSURL>, Shared>>;

        #[method_id(@__retain_semantics Other sharingServicesForItems:)]
        pub unsafe fn sharingServicesForItems(
            items: &NSArray,
        ) -> Id<NSArray<NSSharingService>, Shared>;

        #[cfg(feature = "AppKit_NSSharingServiceName")]
        #[method_id(@__retain_semantics Other sharingServiceNamed:)]
        pub unsafe fn sharingServiceNamed(
            serviceName: &NSSharingServiceName,
        ) -> Option<Id<NSSharingService, Shared>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Init initWithTitle:image:alternateImage:handler:)]
        pub unsafe fn initWithTitle_image_alternateImage_handler(
            this: Option<Allocated<Self>>,
            title: &NSString,
            image: &NSImage,
            alternateImage: Option<&NSImage>,
            block: &Block<(), ()>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method(canPerformWithItems:)]
        pub unsafe fn canPerformWithItems(&self, items: Option<&NSArray>) -> bool;

        #[method(performWithItems:)]
        pub unsafe fn performWithItems(&self, items: &NSArray);
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSSharingContentScope {
        NSSharingContentScopeItem = 0,
        NSSharingContentScopePartial = 1,
        NSSharingContentScopeFull = 2,
    }
);

extern_protocol!(
    pub struct NSSharingServiceDelegate;

    unsafe impl ProtocolType for NSSharingServiceDelegate {
        #[optional]
        #[method(sharingService:willShareItems:)]
        pub unsafe fn sharingService_willShareItems(
            &self,
            sharingService: &NSSharingService,
            items: &NSArray,
        );

        #[optional]
        #[method(sharingService:didFailToShareItems:error:)]
        pub unsafe fn sharingService_didFailToShareItems_error(
            &self,
            sharingService: &NSSharingService,
            items: &NSArray,
            error: &NSError,
        );

        #[optional]
        #[method(sharingService:didShareItems:)]
        pub unsafe fn sharingService_didShareItems(
            &self,
            sharingService: &NSSharingService,
            items: &NSArray,
        );

        #[optional]
        #[method(sharingService:sourceFrameOnScreenForShareItem:)]
        pub unsafe fn sharingService_sourceFrameOnScreenForShareItem(
            &self,
            sharingService: &NSSharingService,
            item: &Object,
        ) -> NSRect;

        #[optional]
        #[method_id(@__retain_semantics Other sharingService:transitionImageForShareItem:contentRect:)]
        pub unsafe fn sharingService_transitionImageForShareItem_contentRect(
            &self,
            sharingService: &NSSharingService,
            item: &Object,
            contentRect: NonNull<NSRect>,
        ) -> Option<Id<NSImage, Shared>>;

        #[optional]
        #[method_id(@__retain_semantics Other sharingService:sourceWindowForShareItems:sharingContentScope:)]
        pub unsafe fn sharingService_sourceWindowForShareItems_sharingContentScope(
            &self,
            sharingService: &NSSharingService,
            items: &NSArray,
            sharingContentScope: NonNull<NSSharingContentScope>,
        ) -> Option<Id<NSWindow, Shared>>;

        #[optional]
        #[method_id(@__retain_semantics Other anchoringViewForSharingService:showRelativeToRect:preferredEdge:)]
        pub unsafe fn anchoringViewForSharingService_showRelativeToRect_preferredEdge(
            &self,
            sharingService: &NSSharingService,
            positioningRect: NonNull<NSRect>,
            preferredEdge: NonNull<NSRectEdge>,
        ) -> Option<Id<NSView, Shared>>;
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSCloudKitSharingServiceOptions {
        NSCloudKitSharingServiceStandard = 0,
        NSCloudKitSharingServiceAllowPublic = 1 << 0,
        NSCloudKitSharingServiceAllowPrivate = 1 << 1,
        NSCloudKitSharingServiceAllowReadOnly = 1 << 4,
        NSCloudKitSharingServiceAllowReadWrite = 1 << 5,
    }
);

extern_protocol!(
    pub struct NSCloudSharingServiceDelegate;

    unsafe impl ProtocolType for NSCloudSharingServiceDelegate {
        #[optional]
        #[method(sharingService:didCompleteForItems:error:)]
        pub unsafe fn sharingService_didCompleteForItems_error(
            &self,
            sharingService: &NSSharingService,
            items: &NSArray,
            error: Option<&NSError>,
        );

        #[optional]
        #[method(optionsForSharingService:shareProvider:)]
        pub unsafe fn optionsForSharingService_shareProvider(
            &self,
            cloudKitSharingService: &NSSharingService,
            provider: &NSItemProvider,
        ) -> NSCloudKitSharingServiceOptions;
    }
);

extern_methods!(
    /// NSCloudKitSharing
    #[cfg(feature = "AppKit_NSItemProvider")]
    unsafe impl NSItemProvider {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSharingServicePicker;

    unsafe impl ClassType for NSSharingServicePicker {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSSharingServicePicker")]
    unsafe impl NSSharingServicePicker {
        #[cfg(feature = "AppKit_NSSharingServicePickerDelegate")]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSSharingServicePickerDelegate, Shared>>;

        #[cfg(feature = "AppKit_NSSharingServicePickerDelegate")]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSSharingServicePickerDelegate>);

        #[method_id(@__retain_semantics Init initWithItems:)]
        pub unsafe fn initWithItems(
            this: Option<Allocated<Self>>,
            items: &NSArray,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(showRelativeToRect:ofView:preferredEdge:)]
        pub unsafe fn showRelativeToRect_ofView_preferredEdge(
            &self,
            rect: NSRect,
            view: &NSView,
            preferredEdge: NSRectEdge,
        );
    }
);

extern_protocol!(
    pub struct NSSharingServicePickerDelegate;

    unsafe impl ProtocolType for NSSharingServicePickerDelegate {
        #[optional]
        #[method_id(@__retain_semantics Other sharingServicePicker:sharingServicesForItems:proposedSharingServices:)]
        pub unsafe fn sharingServicePicker_sharingServicesForItems_proposedSharingServices(
            &self,
            sharingServicePicker: &NSSharingServicePicker,
            items: &NSArray,
            proposedServices: &NSArray<NSSharingService>,
        ) -> Id<NSArray<NSSharingService>, Shared>;

        #[optional]
        #[method_id(@__retain_semantics Other sharingServicePicker:delegateForSharingService:)]
        pub unsafe fn sharingServicePicker_delegateForSharingService(
            &self,
            sharingServicePicker: &NSSharingServicePicker,
            sharingService: &NSSharingService,
        ) -> Option<Id<NSSharingServiceDelegate, Shared>>;

        #[optional]
        #[method(sharingServicePicker:didChooseSharingService:)]
        pub unsafe fn sharingServicePicker_didChooseSharingService(
            &self,
            sharingServicePicker: &NSSharingServicePicker,
            service: Option<&NSSharingService>,
        );
    }
);

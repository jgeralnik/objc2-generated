//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

pub const NSBundleExecutableArchitectureI386: c_uint = 0x00000007;
pub const NSBundleExecutableArchitecturePPC: c_uint = 0x00000012;
pub const NSBundleExecutableArchitectureX86_64: c_uint = 0x01000007;
pub const NSBundleExecutableArchitecturePPC64: c_uint = 0x01000012;
pub const NSBundleExecutableArchitectureARM64: c_uint = 0x0100000c;

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    pub struct NSBundle;

    unsafe impl ClassType for NSBundle {
        type Super = NSObject;
    }
);

unsafe impl Send for NSBundle {}

unsafe impl Sync for NSBundle {}

unsafe impl NSObjectProtocol for NSBundle {}

extern_methods!(
    unsafe impl NSBundle {
        #[method_id(@__retain_semantics Other mainBundle)]
        pub fn mainBundle() -> Retained<NSBundle>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other bundleWithPath:)]
        pub unsafe fn bundleWithPath(path: &NSString) -> Option<Retained<Self>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Init initWithPath:)]
        pub unsafe fn initWithPath(
            this: Allocated<Self>,
            path: &NSString,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other bundleWithURL:)]
        pub unsafe fn bundleWithURL(url: &NSURL) -> Option<Retained<Self>>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Init initWithURL:)]
        pub unsafe fn initWithURL(this: Allocated<Self>, url: &NSURL) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other bundleForClass:)]
        pub unsafe fn bundleForClass(a_class: &AnyClass) -> Retained<NSBundle>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other bundleWithIdentifier:)]
        pub unsafe fn bundleWithIdentifier(identifier: &NSString) -> Option<Retained<NSBundle>>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other allBundles)]
        pub unsafe fn allBundles() -> Retained<NSArray<NSBundle>>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other allFrameworks)]
        pub unsafe fn allFrameworks() -> Retained<NSArray<NSBundle>>;

        #[method(load)]
        pub unsafe fn load(&self) -> bool;

        #[method(isLoaded)]
        pub unsafe fn isLoaded(&self) -> bool;

        #[method(unload)]
        pub unsafe fn unload(&self) -> bool;

        #[cfg(feature = "NSError")]
        #[method(preflightAndReturnError:_)]
        pub unsafe fn preflightAndReturnError(&self) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "NSError")]
        #[method(loadAndReturnError:_)]
        pub unsafe fn loadAndReturnError(&self) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other bundleURL)]
        pub unsafe fn bundleURL(&self) -> Retained<NSURL>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other resourceURL)]
        pub unsafe fn resourceURL(&self) -> Option<Retained<NSURL>>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other executableURL)]
        pub unsafe fn executableURL(&self) -> Option<Retained<NSURL>>;

        #[cfg(all(feature = "NSString", feature = "NSURL"))]
        #[method_id(@__retain_semantics Other URLForAuxiliaryExecutable:)]
        pub unsafe fn URLForAuxiliaryExecutable(
            &self,
            executable_name: &NSString,
        ) -> Option<Retained<NSURL>>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other privateFrameworksURL)]
        pub unsafe fn privateFrameworksURL(&self) -> Option<Retained<NSURL>>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other sharedFrameworksURL)]
        pub unsafe fn sharedFrameworksURL(&self) -> Option<Retained<NSURL>>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other sharedSupportURL)]
        pub unsafe fn sharedSupportURL(&self) -> Option<Retained<NSURL>>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other builtInPlugInsURL)]
        pub unsafe fn builtInPlugInsURL(&self) -> Option<Retained<NSURL>>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other appStoreReceiptURL)]
        pub unsafe fn appStoreReceiptURL(&self) -> Option<Retained<NSURL>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other bundlePath)]
        pub unsafe fn bundlePath(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other resourcePath)]
        pub unsafe fn resourcePath(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other executablePath)]
        pub unsafe fn executablePath(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other pathForAuxiliaryExecutable:)]
        pub unsafe fn pathForAuxiliaryExecutable(
            &self,
            executable_name: &NSString,
        ) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other privateFrameworksPath)]
        pub unsafe fn privateFrameworksPath(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other sharedFrameworksPath)]
        pub unsafe fn sharedFrameworksPath(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other sharedSupportPath)]
        pub unsafe fn sharedSupportPath(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other builtInPlugInsPath)]
        pub unsafe fn builtInPlugInsPath(&self) -> Option<Retained<NSString>>;

        #[cfg(all(feature = "NSString", feature = "NSURL"))]
        #[method_id(@__retain_semantics Other URLForResource:withExtension:subdirectory:inBundleWithURL:)]
        pub unsafe fn URLForResource_withExtension_subdirectory_inBundleWithURL(
            name: Option<&NSString>,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
            bundle_url: &NSURL,
        ) -> Option<Retained<NSURL>>;

        #[cfg(all(feature = "NSArray", feature = "NSString", feature = "NSURL"))]
        #[method_id(@__retain_semantics Other URLsForResourcesWithExtension:subdirectory:inBundleWithURL:)]
        pub unsafe fn URLsForResourcesWithExtension_subdirectory_inBundleWithURL(
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
            bundle_url: &NSURL,
        ) -> Option<Retained<NSArray<NSURL>>>;

        #[cfg(all(feature = "NSString", feature = "NSURL"))]
        #[method_id(@__retain_semantics Other URLForResource:withExtension:)]
        pub unsafe fn URLForResource_withExtension(
            &self,
            name: Option<&NSString>,
            ext: Option<&NSString>,
        ) -> Option<Retained<NSURL>>;

        #[cfg(all(feature = "NSString", feature = "NSURL"))]
        #[method_id(@__retain_semantics Other URLForResource:withExtension:subdirectory:)]
        pub unsafe fn URLForResource_withExtension_subdirectory(
            &self,
            name: Option<&NSString>,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
        ) -> Option<Retained<NSURL>>;

        #[cfg(all(feature = "NSString", feature = "NSURL"))]
        #[method_id(@__retain_semantics Other URLForResource:withExtension:subdirectory:localization:)]
        pub unsafe fn URLForResource_withExtension_subdirectory_localization(
            &self,
            name: Option<&NSString>,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
            localization_name: Option<&NSString>,
        ) -> Option<Retained<NSURL>>;

        #[cfg(all(feature = "NSArray", feature = "NSString", feature = "NSURL"))]
        #[method_id(@__retain_semantics Other URLsForResourcesWithExtension:subdirectory:)]
        pub unsafe fn URLsForResourcesWithExtension_subdirectory(
            &self,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
        ) -> Option<Retained<NSArray<NSURL>>>;

        #[cfg(all(feature = "NSArray", feature = "NSString", feature = "NSURL"))]
        #[method_id(@__retain_semantics Other URLsForResourcesWithExtension:subdirectory:localization:)]
        pub unsafe fn URLsForResourcesWithExtension_subdirectory_localization(
            &self,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
            localization_name: Option<&NSString>,
        ) -> Option<Retained<NSArray<NSURL>>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other pathForResource:ofType:inDirectory:)]
        pub unsafe fn pathForResource_ofType_inDirectory_class(
            name: Option<&NSString>,
            ext: Option<&NSString>,
            bundle_path: &NSString,
        ) -> Option<Retained<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other pathsForResourcesOfType:inDirectory:)]
        pub unsafe fn pathsForResourcesOfType_inDirectory_class(
            ext: Option<&NSString>,
            bundle_path: &NSString,
        ) -> Retained<NSArray<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other pathForResource:ofType:)]
        pub unsafe fn pathForResource_ofType(
            &self,
            name: Option<&NSString>,
            ext: Option<&NSString>,
        ) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other pathForResource:ofType:inDirectory:)]
        pub unsafe fn pathForResource_ofType_inDirectory(
            &self,
            name: Option<&NSString>,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
        ) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other pathForResource:ofType:inDirectory:forLocalization:)]
        pub unsafe fn pathForResource_ofType_inDirectory_forLocalization(
            &self,
            name: Option<&NSString>,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
            localization_name: Option<&NSString>,
        ) -> Option<Retained<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other pathsForResourcesOfType:inDirectory:)]
        pub unsafe fn pathsForResourcesOfType_inDirectory(
            &self,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
        ) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other pathsForResourcesOfType:inDirectory:forLocalization:)]
        pub unsafe fn pathsForResourcesOfType_inDirectory_forLocalization(
            &self,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
            localization_name: Option<&NSString>,
        ) -> Retained<NSArray<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other localizedStringForKey:value:table:)]
        pub unsafe fn localizedStringForKey_value_table(
            &self,
            key: &NSString,
            value: Option<&NSString>,
            table_name: Option<&NSString>,
        ) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other bundleIdentifier)]
        pub unsafe fn bundleIdentifier(&self) -> Option<Retained<NSString>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other infoDictionary)]
        pub fn infoDictionary(&self) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other localizedInfoDictionary)]
        pub unsafe fn localizedInfoDictionary(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other objectForInfoDictionaryKey:)]
        pub unsafe fn objectForInfoDictionaryKey(
            &self,
            key: &NSString,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSString")]
        #[method(classNamed:)]
        pub unsafe fn classNamed(&self, class_name: &NSString) -> Option<&'static AnyClass>;

        #[method(principalClass)]
        pub unsafe fn principalClass(&self) -> Option<&'static AnyClass>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other preferredLocalizations)]
        pub unsafe fn preferredLocalizations(&self) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other localizations)]
        pub unsafe fn localizations(&self) -> Retained<NSArray<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other developmentLocalization)]
        pub unsafe fn developmentLocalization(&self) -> Option<Retained<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other preferredLocalizationsFromArray:)]
        pub unsafe fn preferredLocalizationsFromArray(
            localizations_array: &NSArray<NSString>,
        ) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other preferredLocalizationsFromArray:forPreferences:)]
        pub unsafe fn preferredLocalizationsFromArray_forPreferences(
            localizations_array: &NSArray<NSString>,
            preferences_array: Option<&NSArray<NSString>>,
        ) -> Retained<NSArray<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSValue"))]
        #[method_id(@__retain_semantics Other executableArchitectures)]
        pub unsafe fn executableArchitectures(&self) -> Option<Retained<NSArray<NSNumber>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSBundle {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSBundleExtensionMethods
    #[cfg(feature = "NSString")]
    unsafe impl NSString {
        #[method_id(@__retain_semantics Other variantFittingPresentationWidth:)]
        pub unsafe fn variantFittingPresentationWidth(
            &self,
            width: NSInteger,
        ) -> Retained<NSString>;
    }
);

extern "C" {
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSBundleDidLoadNotification: &'static NSNotificationName;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSLoadedClasses: &'static NSString;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSBundleResourceRequest;

    unsafe impl ClassType for NSBundleResourceRequest {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for NSBundleResourceRequest {}

#[cfg(feature = "NSProgress")]
unsafe impl NSProgressReporting for NSBundleResourceRequest {}

extern_methods!(
    unsafe impl NSBundleResourceRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(feature = "NSSet", feature = "NSString"))]
        #[method_id(@__retain_semantics Init initWithTags:)]
        pub unsafe fn initWithTags(this: Allocated<Self>, tags: &NSSet<NSString>)
            -> Retained<Self>;

        #[cfg(all(feature = "NSSet", feature = "NSString"))]
        #[method_id(@__retain_semantics Init initWithTags:bundle:)]
        pub unsafe fn initWithTags_bundle(
            this: Allocated<Self>,
            tags: &NSSet<NSString>,
            bundle: &NSBundle,
        ) -> Retained<Self>;

        #[method(loadingPriority)]
        pub unsafe fn loadingPriority(&self) -> c_double;

        #[method(setLoadingPriority:)]
        pub unsafe fn setLoadingPriority(&self, loading_priority: c_double);

        #[cfg(all(feature = "NSSet", feature = "NSString"))]
        #[method_id(@__retain_semantics Other tags)]
        pub unsafe fn tags(&self) -> Retained<NSSet<NSString>>;

        #[method_id(@__retain_semantics Other bundle)]
        pub unsafe fn bundle(&self) -> Retained<NSBundle>;

        #[cfg(all(feature = "NSError", feature = "block2"))]
        #[method(beginAccessingResourcesWithCompletionHandler:)]
        pub unsafe fn beginAccessingResourcesWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(conditionallyBeginAccessingResourcesWithCompletionHandler:)]
        pub unsafe fn conditionallyBeginAccessingResourcesWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(Bool)>,
        );

        #[method(endAccessingResources)]
        pub unsafe fn endAccessingResources(&self);

        #[cfg(feature = "NSProgress")]
        #[method_id(@__retain_semantics Other progress)]
        pub unsafe fn progress(&self) -> Retained<NSProgress>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSBundleResourceRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSBundleResourceRequestAdditions
    unsafe impl NSBundle {
        #[cfg(all(feature = "NSSet", feature = "NSString"))]
        #[method(setPreservationPriority:forTags:)]
        pub unsafe fn setPreservationPriority_forTags(
            &self,
            priority: c_double,
            tags: &NSSet<NSString>,
        );

        #[cfg(feature = "NSString")]
        #[method(preservationPriorityForTag:)]
        pub unsafe fn preservationPriorityForTag(&self, tag: &NSString) -> c_double;
    }
);

extern "C" {
    #[cfg(all(feature = "NSNotification", feature = "NSString"))]
    pub static NSBundleResourceRequestLowDiskSpaceNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSBundleResourceRequestLoadingPriorityUrgent: c_double;
}

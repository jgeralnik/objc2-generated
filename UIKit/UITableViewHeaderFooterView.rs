//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitableviewheaderfooterviewconfigurationupdatehandler?language=objc)
#[cfg(all(
    feature = "UIResponder",
    feature = "UIView",
    feature = "UIViewConfigurationState",
    feature = "block2"
))]
pub type UITableViewHeaderFooterViewConfigurationUpdateHandler = *mut block2::Block<
    dyn Fn(NonNull<UITableViewHeaderFooterView>, NonNull<UIViewConfigurationState>),
>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitableviewheaderfooterview?language=objc)
    #[unsafe(super(UIView, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    pub struct UITableViewHeaderFooterView;
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
unsafe impl CALayerDelegate for UITableViewHeaderFooterView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSCoding for UITableViewHeaderFooterView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSObjectProtocol for UITableViewHeaderFooterView {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearance for UITableViewHeaderFooterView {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearanceContainer for UITableViewHeaderFooterView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UICoordinateSpace for UITableViewHeaderFooterView {}

#[cfg(all(
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UITableViewHeaderFooterView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusEnvironment for UITableViewHeaderFooterView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItem for UITableViewHeaderFooterView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItemContainer for UITableViewHeaderFooterView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UIResponderStandardEditActions for UITableViewHeaderFooterView {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UITableViewHeaderFooterView {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UITableViewHeaderFooterView {
        #[method_id(@__retain_semantics Init initWithReuseIdentifier:)]
        pub unsafe fn initWithReuseIdentifier(
            this: Allocated<Self>,
            reuse_identifier: Option<&NSString>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "UIViewConfigurationState")]
        #[method_id(@__retain_semantics Other configurationState)]
        pub unsafe fn configurationState(&self) -> Retained<UIViewConfigurationState>;

        #[method(setNeedsUpdateConfiguration)]
        pub unsafe fn setNeedsUpdateConfiguration(&self);

        #[cfg(feature = "UIViewConfigurationState")]
        #[method(updateConfigurationUsingState:)]
        pub unsafe fn updateConfigurationUsingState(&self, state: &UIViewConfigurationState);

        #[cfg(all(feature = "UIViewConfigurationState", feature = "block2"))]
        #[method(configurationUpdateHandler)]
        pub unsafe fn configurationUpdateHandler(
            &self,
        ) -> UITableViewHeaderFooterViewConfigurationUpdateHandler;

        #[cfg(all(feature = "UIViewConfigurationState", feature = "block2"))]
        #[method(setConfigurationUpdateHandler:)]
        pub unsafe fn setConfigurationUpdateHandler(
            &self,
            configuration_update_handler: UITableViewHeaderFooterViewConfigurationUpdateHandler,
        );

        #[cfg(feature = "UIListContentConfiguration")]
        #[method_id(@__retain_semantics Other defaultContentConfiguration)]
        pub unsafe fn defaultContentConfiguration(&self) -> Retained<UIListContentConfiguration>;

        #[cfg(feature = "UIContentConfiguration")]
        #[method_id(@__retain_semantics Other contentConfiguration)]
        pub unsafe fn contentConfiguration(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIContentConfiguration>>>;

        #[cfg(feature = "UIContentConfiguration")]
        #[method(setContentConfiguration:)]
        pub unsafe fn setContentConfiguration(
            &self,
            content_configuration: Option<&ProtocolObject<dyn UIContentConfiguration>>,
        );

        #[method(automaticallyUpdatesContentConfiguration)]
        pub unsafe fn automaticallyUpdatesContentConfiguration(&self) -> bool;

        #[method(setAutomaticallyUpdatesContentConfiguration:)]
        pub unsafe fn setAutomaticallyUpdatesContentConfiguration(
            &self,
            automatically_updates_content_configuration: bool,
        );

        #[method_id(@__retain_semantics Other contentView)]
        pub unsafe fn contentView(&self) -> Retained<UIView>;

        #[cfg(feature = "UILabel")]
        #[deprecated = "Use UIListContentConfiguration instead, this property will be deprecated in a future release."]
        #[method_id(@__retain_semantics Other textLabel)]
        pub unsafe fn textLabel(&self) -> Option<Retained<UILabel>>;

        #[cfg(feature = "UILabel")]
        #[deprecated = "Use UIListContentConfiguration instead, this property will be deprecated in a future release."]
        #[method_id(@__retain_semantics Other detailTextLabel)]
        pub unsafe fn detailTextLabel(&self) -> Option<Retained<UILabel>>;

        #[cfg(feature = "UIBackgroundConfiguration")]
        #[method_id(@__retain_semantics Other defaultBackgroundConfiguration)]
        pub unsafe fn defaultBackgroundConfiguration(&self) -> Retained<UIBackgroundConfiguration>;

        #[cfg(feature = "UIBackgroundConfiguration")]
        #[method_id(@__retain_semantics Other backgroundConfiguration)]
        pub unsafe fn backgroundConfiguration(&self)
            -> Option<Retained<UIBackgroundConfiguration>>;

        #[cfg(feature = "UIBackgroundConfiguration")]
        #[method(setBackgroundConfiguration:)]
        pub unsafe fn setBackgroundConfiguration(
            &self,
            background_configuration: Option<&UIBackgroundConfiguration>,
        );

        #[method(automaticallyUpdatesBackgroundConfiguration)]
        pub unsafe fn automaticallyUpdatesBackgroundConfiguration(&self) -> bool;

        #[method(setAutomaticallyUpdatesBackgroundConfiguration:)]
        pub unsafe fn setAutomaticallyUpdatesBackgroundConfiguration(
            &self,
            automatically_updates_background_configuration: bool,
        );

        #[method_id(@__retain_semantics Other backgroundView)]
        pub unsafe fn backgroundView(&self) -> Option<Retained<UIView>>;

        #[method(setBackgroundView:)]
        pub unsafe fn setBackgroundView(&self, background_view: Option<&UIView>);

        #[method_id(@__retain_semantics Other reuseIdentifier)]
        pub unsafe fn reuseIdentifier(&self) -> Option<Retained<NSString>>;

        #[method(prepareForReuse)]
        pub unsafe fn prepareForReuse(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIView`
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UITableViewHeaderFooterView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UITableViewHeaderFooterView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

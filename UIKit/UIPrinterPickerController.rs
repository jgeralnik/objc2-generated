//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[cfg(feature = "block2")]
pub type UIPrinterPickerCompletionHandler =
    *mut block2::Block<dyn Fn(NonNull<UIPrinterPickerController>, Bool, *mut NSError)>;

extern_protocol!(
    pub unsafe trait UIPrinterPickerControllerDelegate:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method_id(@__retain_semantics Other printerPickerControllerParentViewController:)]
        unsafe fn printerPickerControllerParentViewController(
            &self,
            printer_picker_controller: &UIPrinterPickerController,
        ) -> Option<Retained<UIViewController>>;

        #[cfg(feature = "UIPrinter")]
        #[optional]
        #[method(printerPickerController:shouldShowPrinter:)]
        unsafe fn printerPickerController_shouldShowPrinter(
            &self,
            printer_picker_controller: &UIPrinterPickerController,
            printer: &UIPrinter,
        ) -> bool;

        #[optional]
        #[method(printerPickerControllerWillPresent:)]
        unsafe fn printerPickerControllerWillPresent(
            &self,
            printer_picker_controller: &UIPrinterPickerController,
        );

        #[optional]
        #[method(printerPickerControllerDidPresent:)]
        unsafe fn printerPickerControllerDidPresent(
            &self,
            printer_picker_controller: &UIPrinterPickerController,
        );

        #[optional]
        #[method(printerPickerControllerWillDismiss:)]
        unsafe fn printerPickerControllerWillDismiss(
            &self,
            printer_picker_controller: &UIPrinterPickerController,
        );

        #[optional]
        #[method(printerPickerControllerDidDismiss:)]
        unsafe fn printerPickerControllerDidDismiss(
            &self,
            printer_picker_controller: &UIPrinterPickerController,
        );

        #[optional]
        #[method(printerPickerControllerDidSelectPrinter:)]
        unsafe fn printerPickerControllerDidSelectPrinter(
            &self,
            printer_picker_controller: &UIPrinterPickerController,
        );
    }

    unsafe impl ProtocolType for dyn UIPrinterPickerControllerDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPrinterPickerController;

    unsafe impl ClassType for UIPrinterPickerController {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UIPrinterPickerController {}

extern_methods!(
    unsafe impl UIPrinterPickerController {
        #[cfg(feature = "UIPrinter")]
        #[method_id(@__retain_semantics Other printerPickerControllerWithInitiallySelectedPrinter:)]
        pub unsafe fn printerPickerControllerWithInitiallySelectedPrinter(
            printer: Option<&UIPrinter>,
            mtm: MainThreadMarker,
        ) -> Retained<UIPrinterPickerController>;

        #[cfg(feature = "UIPrinter")]
        #[method_id(@__retain_semantics Other selectedPrinter)]
        pub unsafe fn selectedPrinter(&self) -> Option<Retained<UIPrinter>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIPrinterPickerControllerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UIPrinterPickerControllerDelegate>>,
        );

        #[cfg(feature = "block2")]
        #[method(presentAnimated:completionHandler:)]
        pub unsafe fn presentAnimated_completionHandler(
            &self,
            animated: bool,
            completion: UIPrinterPickerCompletionHandler,
        ) -> bool;

        #[cfg(all(feature = "UIResponder", feature = "UIView", feature = "block2"))]
        #[method(presentFromRect:inView:animated:completionHandler:)]
        pub unsafe fn presentFromRect_inView_animated_completionHandler(
            &self,
            rect: CGRect,
            view: &UIView,
            animated: bool,
            completion: UIPrinterPickerCompletionHandler,
        ) -> bool;

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem", feature = "block2"))]
        #[method(presentFromBarButtonItem:animated:completionHandler:)]
        pub unsafe fn presentFromBarButtonItem_animated_completionHandler(
            &self,
            item: &UIBarButtonItem,
            animated: bool,
            completion: UIPrinterPickerCompletionHandler,
        ) -> bool;

        #[method(dismissAnimated:)]
        pub unsafe fn dismissAnimated(&self, animated: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIPrinterPickerController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

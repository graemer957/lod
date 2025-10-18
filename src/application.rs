use std::{ffi::c_void, sync::mpsc::Receiver};

use objc2::{
    ffi::{objc_autoreleasePoolPop, objc_autoreleasePoolPush},
    rc::Retained,
};
use objc2_app_kit::{NSApplication, NSEvent, NSEventMask};
use objc2_foundation::{MainThreadMarker, NSDate, NSString};

use crate::app_state::StateChangeMessage;

struct AutoReleasePoolContext(*mut c_void);
unsafe impl Send for AutoReleasePoolContext {}

pub struct Application;

impl Application {
    /// # Panics
    ///
    /// If not called from main thread.
    pub fn run(
        receiver: &Receiver<StateChangeMessage>,
        mut callback: impl FnMut(StateChangeMessage),
    ) {
        // This code is mostly copy/pasted from system_status_bar_macos,
        // see https://github.com/amachang/system_status_bar_macos/blob/1add60da873f9ac8e22be211ef84d72513d9459a/src/lib.rs#L581
        //
        // I plan to upstream this change, but:
        // 1. The author seems to be busy right now and not responding to my other raised PRs
        // 2. Tests do not work out of the box, so would have to fix those first
        unsafe {
            let mtm = MainThreadMarker::new()
                .expect("Application::run should be invoked from main thread!");
            let run_mode = NSString::from_str("kCFRunLoopDefaultMode");
            {
                let app = NSApplication::sharedApplication(mtm);
                app.finishLaunching();
            }

            'event_loop: loop {
                let pool_ctx = AutoReleasePoolContext(objc_autoreleasePoolPush());
                let app = NSApplication::sharedApplication(mtm);

                let event: Option<Retained<NSEvent>> = app
                    .nextEventMatchingMask_untilDate_inMode_dequeue(
                        NSEventMask::Any,
                        Some(&NSDate::distantFuture()),
                        &run_mode,
                        true,
                    );
                if let Some(event) = event {
                    app.sendEvent(&event);
                }
                app.updateWindows();

                while let Ok(message) = receiver.try_recv() {
                    match message {
                        StateChangeMessage::Quit => break 'event_loop,
                        _ => callback(message),
                    }
                }

                objc_autoreleasePoolPop(pool_ctx.0);
            }
        };
    }
}

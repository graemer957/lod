use std::ffi::c_void;

use icrate::{
    objc2::{
        ffi::{objc_autoreleasePoolPop, objc_autoreleasePoolPush},
        rc::Id,
    },
    AppKit::{NSApplication, NSEvent, NSEventMaskAny},
    Foundation::{NSDate, NSString},
};

struct AutoReleasePoolContext(*mut c_void);
unsafe impl Send for AutoReleasePoolContext {}

pub struct Application;

impl Application {
    pub fn run() {
        unsafe {
            let run_mode = NSString::from_str("kCFRunLoopDefaultMode");
            {
                let app = NSApplication::sharedApplication();
                app.finishLaunching();
            }
            /* 'event_loop: */
            loop {
                let pool_ctx = AutoReleasePoolContext(objc_autoreleasePoolPush());
                let app = NSApplication::sharedApplication();
                // if $terminatee.should_terminate() {
                //    break 'event_loop;
                //}

                let event: Option<Id<NSEvent>> = app
                    .nextEventMatchingMask_untilDate_inMode_dequeue(
                        NSEventMaskAny,
                        Some(&NSDate::distantFuture()),
                        &run_mode,
                        true,
                    );
                if let Some(event) = event {
                    app.sendEvent(&event);
                };
                app.updateWindows();

                //$receiver_callback;

                objc_autoreleasePoolPop(pool_ctx.0);
            }
        };
    }
}

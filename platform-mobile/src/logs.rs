use std::panic;
#[cfg(target_os = "android")]
use tracing::{Event, Subscriber};
#[cfg(target_os = "android")]
use tracing_subscriber::{Layer, registry::Registry};
#[cfg(target_os = "android")]
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
#[cfg(target_os = "android")]
use tracing::field::Field;
#[cfg(target_os = "android")]
extern "C" {
    fn __android_log_write(prio: i32, tag: *const libc::c_char, text: *const libc::c_char);
}

#[cfg(target_os = "android")]
pub fn android_log_message(tag: &str, message: &str) {
    unsafe {
        __android_log_write(
            3, // Log priority: LogInfo
            tag.as_ptr() as *const libc::c_char,
            message.as_ptr() as *const libc::c_char
        );
    }
}

#[cfg(target_os = "android")]
struct VisitMessage<'a>(&'a mut String);

#[cfg(target_os = "android")]
impl<'a> tracing::field::Visit for VisitMessage<'a> {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.0.push_str(&format!("{:?}", value));
        }
    }
}

#[cfg(target_os = "android")]
struct AndroidLogLayer;
#[cfg(target_os = "android")]
impl<S> Layer<S> for AndroidLogLayer
    where
        S: Subscriber,
{
    fn on_event(&self, event: &Event, _context: tracing_subscriber::layer::Context<S>) {
        let metadata = event.metadata();

        // Extract the message from the event fields
        let mut message = String::new();
        let mut visitor = VisitMessage(&mut message);
        event.record(&mut visitor);

        let simplified_message = format!(
            "[{}] {} - {}: {}",
            metadata.level(),
            metadata.target(),
            metadata.name(),
            visitor.0
        );

        crate::logs::android_log_message("platform-mobile", &message);
    }
}

#[cfg(target_os = "android")]
use tracing::Level;
#[cfg(target_os = "android")]
use tracing_logcat::{LogcatMakeWriter, LogcatTag};
#[cfg(target_os = "android")]
use tracing_subscriber::fmt::format::Format;

static mut LOGS_SETUP: bool = false;
#[cfg(target_os = "android")]
pub fn setup_logs() {
    let tag = LogcatTag::Fixed(env!("CARGO_PKG_NAME").to_owned());
    let writer = LogcatMakeWriter::new(tag)
        .expect("Failed to initialize logcat writer");
    
    tracing_subscriber::fmt()
        .event_format(Format::default().with_level(false).without_time())
        .with_writer(writer)
        .with_ansi(false)
        .with_max_level(Level::INFO) //TRACE
        .init();

    // unsafe {
    //     if (LOGS_SETUP) {
    //         return
    //     }
    //     LOGS_SETUP = true;
    // }

    // let env_filter = match tracing_subscriber::EnvFilter::try_from_default_env() {
    //     Ok(filter) => filter,
    //     Err(e) => {
    //         let filter = tracing_subscriber::EnvFilter::new("info,dash_sdk=trace,h2=info");
    //         android_log_message("platform-mobile", &format!("Error parsing env filter: {}. Using default.", e));
    //         filter
    //     }
    // };
    // let subscriber = tracing_subscriber::registry()
    //     .with(AndroidLogLayer)
    //     .with(
    //         tracing_subscriber::fmt::layer()
    //             .with_ansi(false)
    //             .with_writer(std::io::stderr)
    //     )
    //     .with(env_filter);

    // //tracing::subscriber::set_global_default(subscriber)
    // //    .expect("Unable to set global default subscriber");

    // if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
    //     crate::logs::android_log_message("platform-mobile", &*format!("Unable to set global default subscriber: {}", e));
    // }

    setup_panic_hook();
}

#[cfg(not(target_os = "android"))]
pub fn setup_logs() {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            "info,dash_sdk=trace,h2=info",
        ))
        .pretty()
        .with_ansi(false)
        .with_writer(std::io::stdout)
        .try_init()
        .ok();

    setup_panic_hook();
}

fn setup_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        tracing::warn!("Panic occurred: {:?}", panic_info);

        if let Some(panic_msg) = panic_info.payload().downcast_ref::<&str>() {
            tracing::info!("Thread panicked with message: {}", panic_msg);
        } else if let Some(panic_msg) = panic_info.payload().downcast_ref::<String>() {
            tracing::info!("Thread panicked with message: {}", panic_msg);
        } else {
            tracing::info!("Thread panicked with unknown type.");
        }
    }));
}
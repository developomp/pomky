use std::time::Duration;

use gdk::{glib::Object, prelude::IsA};
use gtk::prelude::BuilderExtManual;

use crate::could_not_get;

/// number of kibibytes in a gigabyte
const KIB_IN_GB: f64 = 1024_f64 * 1000_f64;

/// number of bytes in a gigabyte
const B_IN_GB: f64 = 1000_f64 * 1000_f64 * 1000_f64;

/// Converts kilobytes to gigabytes
pub fn kib_2_gb(kb: u64) -> f64 {
    return kb as f64 / KIB_IN_GB;
}

/// Converts bytes to gigabytes
pub fn b_2_gb(bytes: u64) -> f64 {
    return bytes as f64 / B_IN_GB;
}

pub fn get_widget<T: IsA<Object>>(name: &str, builder: &gtk::Builder) -> T {
    return builder.object(name).expect(could_not_get!(name));
}

/// converts millisecond to Duration
pub fn msec_2_duration(millisecond: u32) -> Duration {
    return Duration::new(0, millisecond * 1_000_000);
}

/// convert bytes to a more sensible unit
pub fn pretty_bytes(bytes: u64) -> String {
    let bytes = bytes as f64;

    const BYTES_IN_GIGABYTE: f64 = 1_000_000_000.;
    if bytes >= BYTES_IN_GIGABYTE {
        let kilobytes = bytes / BYTES_IN_GIGABYTE;
        return format!("{:.1} GB", kilobytes);
    }

    const BYTES_IN_MEGABYTE: f64 = 1_000_000.;
    if bytes >= BYTES_IN_MEGABYTE {
        let megabytes = bytes / BYTES_IN_MEGABYTE;
        return format!("{:.1} MB", megabytes);
    }

    const BYTES_IN_KILOBYTES: f64 = 1000.;
    if bytes >= BYTES_IN_KILOBYTES {
        let kilobytes = bytes / BYTES_IN_KILOBYTES;
        return format!("{:.1} KB", kilobytes);
    }

    return format!("{:.1} B", bytes);
}

#[macro_export]
macro_rules! could_not_get {
    ($name:expr) => {
        &format!("Couldn't get {}", $name)
    };
}

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

#[macro_export]
macro_rules! could_not_get {
    ($name:expr) => {
        format!("Couldn't get {}", $name).as_str()
    };
}

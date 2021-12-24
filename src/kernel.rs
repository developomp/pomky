use gtk::prelude::{BuilderExtManual, LabelExt};
use gtk::{Builder, Label};
use sysinfo::{RefreshKind, System, SystemExt};

use crate::could_not_get;

pub fn setup(builder: &Builder) {
    let sys = System::new_with_specifics(RefreshKind::new());

    let label_kernel_version: Label = builder
        .object("label_kernel_version")
        .expect(could_not_get!("label_kernel_version"));

    let kernel_version = sys.kernel_version();
    let kernel_version = match kernel_version {
        None => "not available",
        Some(ref value) => value.as_str(),
    };
    label_kernel_version.set_text(kernel_version);
}

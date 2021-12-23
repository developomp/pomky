use gtk::prelude::{BuilderExtManual, LabelExt};
use gtk::{Builder, Label};
use sysinfo::{RefreshKind, System, SystemExt};

pub fn setup(builder: &Builder) {
    let sys = System::new_with_specifics(RefreshKind::new());

    let label_kernel_version: Label = builder
        .object("label_kernel_version")
        .expect("Couldn't get Kernel version label");

    let kernel_version = sys.kernel_version();
    let kernel_version = match kernel_version {
        None => "not available",
        Some(ref value) => value.as_str(),
    };
    label_kernel_version.set_text(kernel_version);
}

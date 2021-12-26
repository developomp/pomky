use gtk::{prelude::LabelExt, Builder};

use sysinfo::{RefreshKind, System, SystemExt};

use crate::util::get_widget;

pub fn setup(builder: &Builder) {
    let sys = System::new_with_specifics(RefreshKind::new());

    let label_kernel_version = get_widget::<gtk::Label>("label_kernel_version", &builder);

    let kernel_version = sys.kernel_version();
    let kernel_version = match kernel_version {
        None => "not available",
        Some(ref value) => value.as_str(),
    };
    label_kernel_version.set_text(kernel_version);
}

use gtk::prelude::{BuilderExtManual, LabelExt};
use gtk::{Builder, Label};
use sysinfo::{RefreshKind, System, SystemExt};

pub fn setup(builder: &Builder) {
    let sys = System::new_with_specifics(RefreshKind::new());

    let label_cpu_cores: Label = builder
        .object("label_physical_core_count")
        .expect("Couldn't get Kernel version label");

    let core_count = match sys.physical_core_count() {
        Some(count) => count,
        None => 0,
    };

    label_cpu_cores.set_text(format!("physical cores: {}", core_count).as_str());
}

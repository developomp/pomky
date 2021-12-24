use gtk::prelude::{BuilderExtManual, LabelExt};
use gtk::{Builder, Label};
use sysinfo::{RefreshKind, System, SystemExt};

use crate::could_not_get;

pub fn setup(builder: &Builder) {
    let sys = System::new_with_specifics(RefreshKind::new());

    let label_physical_core_count: Label = builder
        .object("label_physical_core_count")
        .expect(could_not_get!("label_physical_core_count"));

    let core_count = match sys.physical_core_count() {
        Some(count) => count,
        None => 0,
    };

    label_physical_core_count.set_text(format!("physical cores: {}", core_count).as_str());
}

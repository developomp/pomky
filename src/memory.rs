use gdk::glib;
use gtk::{
    prelude::{BuilderExtManual, LabelExt},
    Builder,
};
use sysinfo::{RefreshKind, System, SystemExt};

use crate::MEMORY_UPDATE_INTERVAL;
use crate::{could_not_get, util::kib_2_gb};

pub fn setup(builder: &Builder) {
    let mut sys = System::new_with_specifics(RefreshKind::new().with_memory());
    sys.refresh_memory();

    let label_memory_usage: gtk::Label = builder
        .object("label_memory_usage")
        .expect(could_not_get!("label_memory_usage"));
    let label_memory_free: gtk::Label = builder
        .object("label_memory_free")
        .expect(could_not_get!("label_memory_free"));

    update_memory_usage(&label_memory_usage, &sys);
    update_memory_free(&label_memory_free, &sys);

    glib::timeout_add_seconds_local(MEMORY_UPDATE_INTERVAL, move || {
        sys.refresh_memory();

        update_memory_usage(&label_memory_usage, &sys);
        update_memory_free(&label_memory_free, &sys);

        return glib::Continue(true);
    });
}

fn update_memory_usage(label: &gtk::Label, sys: &System) {
    let mem_used = sys.used_memory();
    let mem_total = sys.total_memory();

    label.set_text(
        format!(
            "{:.1} GiB / {:.1} GiB ({:.1}%)",
            kib_2_gb(mem_used),
            kib_2_gb(mem_total),
            100 * mem_used / mem_total
        )
        .as_str(),
    );
}

fn update_memory_free(label: &gtk::Label, sys: &System) {
    label.set_text(format!("{:.1} GiB", kib_2_gb(sys.free_memory())).as_str());
}

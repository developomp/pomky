use gdk::glib;
use gtk::prelude::{BuilderExtManual, LabelExt};
use gtk::Builder;

use sysinfo::{RefreshKind, System, SystemExt};

use crate::{bar::build_bar, could_not_get, util::kib_2_gb, MEMORY_UPDATE_INTERVAL};

pub fn setup(builder: &Builder) {
    let label_memory_used: gtk::Label = builder
        .object("label_memory_used")
        .expect(could_not_get!("label_memory_used"));
    let label_memory_total: gtk::Label = builder
        .object("label_memory_total")
        .expect(could_not_get!("label_memory_total"));
    let label_memory_free: gtk::Label = builder
        .object("label_memory_free")
        .expect(could_not_get!("label_memory_free"));
    let label_memory_percent: gtk::Label = builder
        .object("label_memory_percent")
        .expect(could_not_get!("label_memory_percent"));

    let mut sys = System::new_with_specifics(RefreshKind::new().with_memory());
    sys.refresh_memory();
    update(
        &sys,
        &label_memory_used,
        &label_memory_total,
        &label_memory_free,
        &label_memory_percent,
    );

    build_bar(
        &builder,
        "memory_usage_bar",
        500,
        6,
        MEMORY_UPDATE_INTERVAL.into(),
        || {
            return System::new_with_specifics(RefreshKind::new().with_memory());
        },
        |sys| {
            sys.refresh_memory();

            let (_, _, ratio) = get_mem_stats(&sys);

            return ratio;
        },
    );

    glib::timeout_add_seconds_local(MEMORY_UPDATE_INTERVAL, move || {
        sys.refresh_memory();
        update(
            &sys,
            &label_memory_used,
            &label_memory_total,
            &label_memory_free,
            &label_memory_percent,
        );

        return glib::Continue(true);
    });
}

fn update(
    sys: &System,
    label_memory_used: &gtk::Label,
    label_memory_total: &gtk::Label,
    label_memory_free: &gtk::Label,
    label_memory_percent: &gtk::Label,
) {
    let (mem_used, mem_total, ratio) = get_mem_stats(&sys);

    label_memory_used.set_text(format!("{:.1} GB", kib_2_gb(mem_used)).as_str());
    label_memory_total.set_text(format!("{:.1} GB", kib_2_gb(mem_total)).as_str());
    label_memory_free.set_text(format!("{:.1} GB", kib_2_gb(sys.free_memory())).as_str());
    label_memory_percent.set_text(format!("{:.1}%", 100.0 * ratio).as_str());
}

fn get_mem_stats(sys: &System) -> (u64, u64, f64) {
    let mem_used = sys.used_memory();
    let mem_total = sys.total_memory();
    let ratio = mem_used as f64 / mem_total as f64;

    return (mem_used, mem_total, ratio);
}

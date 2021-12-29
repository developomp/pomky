use gdk::glib::{self, Sender};
use gtk::prelude::LabelExt;
use gtk::Builder;

use sysinfo::{RefreshKind, System, SystemExt};

use crate::custom_components::bar::build_bar;
use crate::util::{get_widget, kib_2_gb};

const MEMORY_UPDATE_INTERVAL: u32 = 1; // in seconds

pub fn setup(builder: &Builder) {
    let label_memory_used = get_widget("label_memory_used", &builder);
    let label_memory_total = get_widget("label_memory_total", &builder);
    let label_memory_free = get_widget("label_memory_free", &builder);
    let label_memory_percent = get_widget("label_memory_percent", &builder);

    let mut sys = System::new_with_specifics(RefreshKind::new().with_memory());

    let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    build_bar(get_widget("memory_usage_bar", &builder), 500, 6, rx);

    update(
        &mut sys,
        &label_memory_used,
        &label_memory_total,
        &label_memory_free,
        &label_memory_percent,
        &tx,
    );
    glib::timeout_add_seconds_local(MEMORY_UPDATE_INTERVAL, move || {
        update(
            &mut sys,
            &label_memory_used,
            &label_memory_total,
            &label_memory_free,
            &label_memory_percent,
            &tx,
        );

        return glib::Continue(true);
    });
}

fn update(
    sys: &mut System,
    label_memory_used: &gtk::Label,
    label_memory_total: &gtk::Label,
    label_memory_free: &gtk::Label,
    label_memory_percent: &gtk::Label,
    tx: &Sender<f64>,
) {
    sys.refresh_memory();

    let mem_used = sys.used_memory();
    let mem_total = sys.total_memory();
    let ratio = mem_used as f64 / mem_total as f64;

    label_memory_used.set_text(&format!("{:.1} GB", kib_2_gb(mem_used)));
    label_memory_total.set_text(&format!("{:.1} GB", kib_2_gb(mem_total)));
    label_memory_free.set_text(&format!("{:.1} GB", kib_2_gb(sys.free_memory())));
    label_memory_percent.set_text(&format!("{:.1}%", 100.0 * ratio));

    tx.send(ratio).unwrap();
}

use gdk::glib;
use gtk::prelude::*;
use sysinfo::{DiskExt, RefreshKind, System, SystemExt};

use crate::custom_components::bar::build_bar;
use crate::util::{b_2_gb, get_widget};

const UPDATE_INTERVAL: u32 = 10;

pub fn setup(builder: &gtk::Builder) {
    let label_disk_usage = get_widget("label_disk_usage", &builder);
    let label_disk_percent = get_widget("label_disk_percent", &builder);
    let (disk_tx, disk_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    build_bar(get_widget("disk_bar", &builder), 500, 6, disk_rx);

    update(&label_disk_usage, &label_disk_percent, &disk_tx);
    glib::timeout_add_seconds_local(UPDATE_INTERVAL, move || {
        update(&label_disk_usage, &label_disk_percent, &disk_tx);

        return glib::Continue(true);
    });
}

fn update(
    label_disk_usage: &gtk::Label,
    label_disk_percent: &gtk::Label,
    disk_tx: &glib::Sender<f64>,
) {
    let mut sys = System::new_with_specifics(RefreshKind::new().with_disks().with_disks_list());
    sys.refresh_disks();

    let mut total_bytes = 0;
    let mut available_bytes = 0;
    for partition in sys.disks() {
        total_bytes += partition.total_space();
        available_bytes += partition.available_space();
    }

    let used_bytes = total_bytes - available_bytes;
    let ratio = used_bytes as f64 / total_bytes as f64;

    label_disk_usage.set_text(&format!(
        "{:.1}  /  {:.1} GB",
        b_2_gb(used_bytes),
        b_2_gb(total_bytes)
    ));
    label_disk_percent.set_text(&format!("{:.1}%", 100.0 * ratio));
    disk_tx.send(ratio).unwrap();
}

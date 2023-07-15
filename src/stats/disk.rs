use gdk::glib;
use gtk::prelude::*;
use sysinfo::{DiskExt, RefreshKind, System, SystemExt};

use crate::custom_components::bar::build_bar;
use crate::util::{b_2_gb, get_widget};

const UPDATE_INTERVAL_DISK: u32 = 10;

pub fn setup(builder: &gtk::Builder) {
    // primary disk
    let label_disk_name = get_widget("label_disk_name", &builder);
    let label_disk_usage = get_widget("label_disk_usage", &builder);
    let label_disk_percent = get_widget("label_disk_percent", &builder);
    let (disk_tx, disk_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    build_bar(get_widget("disk_bar", &builder), 500, 6, disk_rx);

    update(
        &label_disk_name,
        &label_disk_usage,
        &label_disk_percent,
        &disk_tx,
    );
    glib::timeout_add_seconds_local(UPDATE_INTERVAL_DISK, move || {
        update(
            &label_disk_name,
            &label_disk_usage,
            &label_disk_percent,
            &disk_tx,
        );

        return glib::Continue(true);
    });
}

fn update(
    label_disk_name: &gtk::Label,
    label_disk_usage: &gtk::Label,
    label_disk_percent: &gtk::Label,
    disk_tx: &glib::Sender<f64>,
) {
    let mut sys = System::new_with_specifics(RefreshKind::new().with_disks().with_disks_list());
    sys.refresh_disks();

    for disk in sys.disks() {
        match disk.mount_point().to_str() {
            Some("/") => {
                let total = disk.total_space();
                let used = total - disk.available_space();
                let ratio = used as f64 / total as f64;

                label_disk_name
                    .set_text(disk.name().to_str().expect("Failed to process disk name"));
                label_disk_usage.set_text(&format!(
                    "{:.1}  /  {:.1} GB",
                    b_2_gb(used),
                    b_2_gb(total)
                ));
                label_disk_percent.set_text(&format!("{:.1}%", 100.0 * ratio));
                disk_tx.send(ratio).unwrap();
            }

            _ => {}
        }
    }
}

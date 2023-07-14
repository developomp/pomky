use gdk::glib;
use gtk::prelude::*;
use sysinfo::{Disk, DiskExt, RefreshKind, System, SystemExt};

use crate::config::CONFIG_LOCK;
use crate::custom_components::bar::build_bar;
use crate::util::{b_2_gb, get_widget};

pub fn setup(builder: &gtk::Builder) {
    // primary disk
    let label_disk_root = get_widget("label_disk_root", &builder);
    let label_disk_root_percent = get_widget("label_disk_root_percent", &builder);
    let (disk_root_tx, disk_root_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    // secondary disk
    let label_disk_data = get_widget("label_disk_data", &builder);
    let label_disk_data_percent = get_widget("label_disk_data_percent", &builder);
    let (disk_data_tx, disk_data_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    build_bar(get_widget("disk_root_bar", &builder), 500, 6, disk_root_rx);
    build_bar(get_widget("disk_data_bar", &builder), 500, 6, disk_data_rx);

    update(
        &label_disk_root,
        &label_disk_root_percent,
        &label_disk_data,
        &label_disk_data_percent,
        &disk_root_tx,
        &disk_data_tx,
    );
    glib::timeout_add_seconds_local(
        CONFIG_LOCK.read().unwrap().update_interval_disk,
        move || {
            update(
                &label_disk_root,
                &label_disk_root_percent,
                &label_disk_data,
                &label_disk_data_percent,
                &disk_root_tx,
                &disk_data_tx,
            );

            return glib::Continue(true);
        },
    );
}

fn update(
    label_disk_root: &gtk::Label,
    label_disk_root_percent: &gtk::Label,
    label_disk_data: &gtk::Label,
    label_disk_data_percent: &gtk::Label,
    disk_root_tx: &glib::Sender<f64>,
    disk_data_tx: &glib::Sender<f64>,
) {
    let mut sys = System::new_with_specifics(RefreshKind::new().with_disks().with_disks_list());
    sys.refresh_disks();

    for disk in sys.disks() {
        match disk.mount_point().to_str() {
            Some("/") => {
                update_disk(
                    disk,
                    &label_disk_root,
                    &label_disk_root_percent,
                    &disk_root_tx,
                );
            }

            Some("/media/pomp/data") => {
                update_disk(
                    disk,
                    &label_disk_data,
                    &label_disk_data_percent,
                    &disk_data_tx,
                );
            }

            Some(&_) => {}

            None => {}
        }
    }
}

fn update_disk(
    disk: &Disk,
    label: &gtk::Label,
    label_percent: &gtk::Label,
    tx: &glib::Sender<f64>,
) {
    let total = disk.total_space();
    let used = total - disk.available_space();
    let ratio = used as f64 / total as f64;

    label.set_text(&format!("{:.1}  /  {:.1} GB", b_2_gb(used), b_2_gb(total)));
    label_percent.set_text(&format!("{:.1}%", 100.0 * ratio));
    tx.send(ratio).unwrap();
}

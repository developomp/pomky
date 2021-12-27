use gdk::glib;
use gtk::prelude::LabelExt;

use sysinfo::{Disk, DiskExt, RefreshKind, System, SystemExt};

use crate::bar::build_bar;
use crate::util::{b_2_gb, get_widget};

const DISK_UPDATE_INTERVAL: u32 = 30_000;

pub fn setup(builder: &gtk::Builder) {
    let label_disk_root = get_widget("label_disk_root", &builder);
    let label_disk_root_percent = get_widget("label_disk_root_percent", &builder);

    let label_disk_data = get_widget("label_disk_data", &builder);
    let label_disk_data_percent = get_widget("label_disk_data_percent", &builder);

    update(
        &label_disk_root,
        &label_disk_root_percent,
        &label_disk_data,
        &label_disk_data_percent,
    );

    let disk_data_pairs = [
        ("disk_root_bar", "/"),
        ("disk_data_bar", "/media/pomp/data"),
    ];

    for (widget_name, path) in disk_data_pairs {
        build_bar(
            &builder,
            widget_name,
            500,
            6,
            DISK_UPDATE_INTERVAL as u64,
            || {
                return System::new_with_specifics(
                    RefreshKind::new().with_disks().with_disks_list(),
                );
            },
            |sys| {
                sys.refresh_disks();

                let mut result = 0.0;

                for disk in sys.disks() {
                    if disk.mount_point().to_str() == Some(path) {
                        let (_, _, ratio) = get_disk_stats(disk);
                        result = ratio;
                    }
                }

                return result;
            },
        );
    }

    glib::timeout_add_seconds_local(DISK_UPDATE_INTERVAL / 1000, move || {
        update(
            &label_disk_root,
            &label_disk_root_percent,
            &label_disk_data,
            &label_disk_data_percent,
        );

        return glib::Continue(true);
    });
}

fn update(
    label_disk_root: &gtk::Label,
    label_disk_root_percent: &gtk::Label,
    label_disk_data: &gtk::Label,
    label_disk_data_percent: &gtk::Label,
) {
    let mut sys = System::new_with_specifics(RefreshKind::new().with_disks().with_disks_list());
    sys.refresh_disks();

    for disk in sys.disks() {
        match disk.mount_point().to_str() {
            Some("/") => {
                update_disk(disk, &label_disk_root, &label_disk_root_percent);
            }

            Some("/media/pomp/data") => {
                update_disk(disk, &label_disk_data, &label_disk_data_percent);
            }

            Some(&_) => {}

            None => {}
        }
    }
}

fn update_disk(disk: &Disk, label: &gtk::Label, label_percent: &gtk::Label) {
    let (total, used, ratio) = get_disk_stats(disk);

    label.set_text(format!("{:.1}  /  {:.1} GB", b_2_gb(used), b_2_gb(total)).as_str());
    label_percent.set_text(format!("{:.1}%", 100.0 * ratio).as_str());
}

fn get_disk_stats(disk: &Disk) -> (u64, u64, f64) {
    let total = disk.total_space();
    let used = total - disk.available_space();
    let ratio = used as f64 / total as f64;

    return (total, used, ratio);
}

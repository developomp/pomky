use gdk::glib;
use gtk::prelude::{BuilderExtManual, LabelExt};

use sysinfo::{Disk, DiskExt, RefreshKind, System, SystemExt};

use crate::bar::build_bar;
use crate::util::b_2_gb;
use crate::{could_not_get, DISK_UPDATE_INTERVAL};

pub fn setup(builder: &gtk::Builder) {
    let label_disk_root: gtk::Label = builder
        .object("label_disk_root")
        .expect(could_not_get!("label_disk_root"));
    let label_disk_root_percent: gtk::Label = builder
        .object("label_disk_root_percent")
        .expect(could_not_get!("label_disk_root_percent"));

    let label_disk_data: gtk::Label = builder
        .object("label_disk_data")
        .expect(could_not_get!("label_disk_data"));
    let label_disk_data_percent: gtk::Label = builder
        .object("label_disk_data_percent")
        .expect(could_not_get!("label_disk_data_percent"));

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

    glib::timeout_add_seconds_local(DISK_UPDATE_INTERVAL, move || {
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

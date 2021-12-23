use gdk::glib;
use gtk::{
    prelude::{BuilderExtManual, LabelExt},
    Builder,
};
use sysinfo::{Disk, DiskExt, RefreshKind, System, SystemExt};

use crate::{util::b_2_gb, DISK_UPDATE_INTERVAL};

pub fn setup(builder: &Builder) {
    let label_disk_root: gtk::Label = builder
        .object("label_disk_root")
        .expect("Couldn't get root disk label");
    let label_disk_data: gtk::Label = builder
        .object("label_disk_data")
        .expect("Couldn't get data disk label");
    let label_disk_root_percent: gtk::Label = builder
        .object("label_disk_root_percent")
        .expect("Couldn't get root disk percent label");
    let label_disk_data_percent: gtk::Label = builder
        .object("label_disk_data_percent")
        .expect("Couldn't get data disk percent label");

    let mut sys = System::new_with_specifics(RefreshKind::new().with_disks().with_disks_list());
    sys.refresh_disks();
    update_disks(
        &sys,
        &label_disk_root,
        &label_disk_data,
        &label_disk_root_percent,
        &label_disk_data_percent,
    );

    let mut sys = System::new_with_specifics(RefreshKind::new().with_disks().with_disks_list());

    glib::timeout_add_seconds_local(DISK_UPDATE_INTERVAL, move || {
        sys.refresh_disks();
        update_disks(
            &sys,
            &label_disk_root,
            &label_disk_data,
            &label_disk_root_percent,
            &label_disk_data_percent,
        );

        return glib::Continue(true);
    });
}

fn update_disks(
    sys: &System,
    label_disk_root: &gtk::Label,
    label_disk_data: &gtk::Label,
    label_disk_root_percent: &gtk::Label,
    label_disk_data_percent: &gtk::Label,
) {
    for disk in sys.disks() {
        match disk.mount_point().to_str() {
            Some("/") => {
                update_disk(&label_disk_root, &label_disk_root_percent, disk);
            }

            Some("/media/pomp/data") => {
                update_disk(&label_disk_data, &label_disk_data_percent, disk);
            }

            Some(&_) => {}

            None => {}
        }
    }
}

fn update_disk(label: &gtk::Label, label_percent: &gtk::Label, disk: &Disk) {
    let total = disk.total_space();
    let used = total - disk.available_space();

    label.set_text(format!("{:.1}  /  {:.1}  GB", b_2_gb(used), b_2_gb(total)).as_str());
    label_percent.set_text(format!("{:.1}%", 100_f64 * used as f64 / total as f64).as_str());
}

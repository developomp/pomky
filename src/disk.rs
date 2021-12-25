use gdk::glib;
use gtk::prelude::{BuilderExtManual, LabelExt};
use gtk::{Builder, DrawingArea};

use sysinfo::{Disk, DiskExt, RefreshKind, System, SystemExt};

use crate::bar::draw_bar;
use crate::util::b_2_gb;
use crate::{could_not_get, DISK_UPDATE_INTERVAL};

pub fn setup(builder: &Builder) {
    let label_disk_root: gtk::Label = builder
        .object("label_disk_root")
        .expect(could_not_get!("label_disk_root"));
    let label_disk_root_percent: gtk::Label = builder
        .object("label_disk_root_percent")
        .expect(could_not_get!("label_disk_root_percent"));
    let disk_root_bar: gtk::DrawingArea = builder
        .object("disk_root_bar")
        .expect(could_not_get!("disk_root_bar"));

    let label_disk_data: gtk::Label = builder
        .object("label_disk_data")
        .expect(could_not_get!("label_disk_data"));
    let label_disk_data_percent: gtk::Label = builder
        .object("label_disk_data_percent")
        .expect(could_not_get!("label_disk_data_percent"));
    let disk_data_bar: gtk::DrawingArea = builder
        .object("disk_data_bar")
        .expect(could_not_get!("disk_data_bar"));

    update(
        &label_disk_root,
        &label_disk_root_percent,
        &disk_root_bar,
        &label_disk_data,
        &label_disk_data_percent,
        &disk_data_bar,
    );

    glib::timeout_add_seconds_local(DISK_UPDATE_INTERVAL, move || {
        update(
            &label_disk_root,
            &label_disk_root_percent,
            &disk_root_bar,
            &label_disk_data,
            &label_disk_data_percent,
            &disk_data_bar,
        );

        return glib::Continue(true);
    });
}

fn update(
    label_disk_root: &gtk::Label,
    label_disk_root_percent: &gtk::Label,
    disk_root_bar: &gtk::DrawingArea,
    label_disk_data: &gtk::Label,
    label_disk_data_percent: &gtk::Label,
    disk_data_bar: &gtk::DrawingArea,
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
                    &disk_root_bar,
                );
            }

            Some("/media/pomp/data") => {
                update_disk(
                    disk,
                    &label_disk_data,
                    &label_disk_data_percent,
                    &disk_data_bar,
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
    drawing_area: &DrawingArea,
) {
    let total = disk.total_space();
    let used = total - disk.available_space();
    let ratio = used as f64 / total as f64;

    label.set_text(format!("{:.1}  /  {:.1} GB", b_2_gb(used), b_2_gb(total)).as_str());
    label_percent.set_text(format!("{:.1}%", 100.0 * ratio).as_str());

    draw_bar(&drawing_area, 500, 6, ratio);
}

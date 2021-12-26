use gtk::cairo::{Context, ImageSurface};

/// Render the image surface into the context at the given position
pub fn draw_image_if_dirty(cr: &Context, image: &ImageSurface, dimensions: (i32, i32)) {
    let x = 0.0;
    let y = 0.0;
    let w = dimensions.0 as f64;
    let h = dimensions.1 as f64;
    let (clip_x1, clip_y1, clip_x2, clip_y2) =
        cr.clip_extents().expect("Can't get cairo clip extents");
    if clip_x1 >= x + w || clip_y1 >= y + h || clip_x2 <= x || clip_y2 <= y {
        return;
    }
    cr.set_source_surface(image, x, y)
        .expect("The surface has an invalid state");
    cr.paint().expect("Invalid cairo surface state");

    // Release the reference to the surface again
    cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
}

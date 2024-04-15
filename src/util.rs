use sdl2::rect::Rect;

pub fn is_true_intersection(origin: &Rect, target: &Rect) -> bool {
    let origin_x = origin.x();
    let origin_y = origin.y();
    let origin_w = origin.width() as i32;
    let origin_h = origin.height() as i32;

    let target_x = target.x();
    let target_y = target.y();
    let target_w = target.width() as i32;
    let target_h = target.height() as i32;

    let origin_right = origin_x + origin_w;
    let origin_bottom = origin_y + origin_h;
    let target_right = target_x + target_w;
    let target_bottom = target_y + target_h;

    println!(
        "origin_x: {}, origin_y: {}, origin_w: {}, origin_h: {}",
        origin_x, origin_y, origin_w, origin_h
    );
    println!(
        "target_x: {}, target_y: {}, target_w: {}, target_h: {}",
        target_x, target_y, target_w, target_h
    );

    if origin_x < target_right
        && origin_right > target_x
        && origin_y < target_bottom
        && origin_bottom > target_y
    {
        return true;
    }
    false
}

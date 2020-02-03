use std::cmp;

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

const WHITE: u32 = 0xFF_FF_FF_FF;
const SKY_BLUE: u32 = 0xFF_EB_CE_87;

#[no_mangle]
static mut SCREEN: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

#[no_mangle]
pub extern fn width() -> usize {
    WIDTH
}

#[no_mangle]
pub extern fn height() -> usize {
    HEIGHT
}

#[no_mangle]
pub unsafe extern fn render() {
    render_safe(&mut SCREEN)
}

#[no_mangle]
pub unsafe extern fn update(x: u32, y: u32, radius: u32) {
    update_safe(&mut SCREEN, x as usize, y as usize, radius as usize)
}

fn p(x: usize, y: usize) -> usize
{
    y * WIDTH + x
}

fn clamp(v: i64, upper: usize) -> usize {
    cmp::max(0, cmp::min(v, upper as i64)) as usize
}

fn clamp_y(v: i64) -> usize {
    clamp(v, HEIGHT)
}

fn clamp_x(v: i64) -> usize {
    clamp(v, WIDTH)
}

fn horizontal_line(screen: &mut [u32; WIDTH * HEIGHT], start_x: usize, end_x: usize, y: usize, color: u32) {
    for x in start_x..=end_x {
        screen[p(x, y)] = color;
    }
}


fn update_safe(screen: &mut [u32; WIDTH * HEIGHT], click_x: usize, click_y: usize, radius: usize) {
    let click_x = click_x as i64;
    let click_y = click_y as i64;
    let radius = radius as i64;

    let mut f = 1 - radius;
    let mut ddf_x = 1;
    let mut ddf_y = 2 * radius;
    let mut x = 0;
    let mut y = radius;


    screen[p(click_x as usize, clamp_y(click_y - radius))] = WHITE;
    screen[p(click_x as usize, clamp_y(click_y + radius))] = WHITE;
    horizontal_line(screen, clamp_x(click_x - radius), clamp_x(click_x + radius), click_y as usize, WHITE);

    while x < y {
        if f >= 0 {
            y -= 1;
            ddf_y += 2;
            f += ddf_y;
        }
        x += 1;
        ddf_x += 2;
        f += ddf_x;

        let start_xx = clamp_x(click_x - x);
        let end_xx = clamp_x(click_x + x);
        horizontal_line(screen, start_xx, end_xx, clamp_y(click_y + y), WHITE);
        horizontal_line(screen, start_xx, end_xx, clamp_y(click_y - y), WHITE);

        let start_xy = clamp_x(click_x - y);
        let end_xy = clamp_x(click_x + y);
        horizontal_line(screen, start_xy, end_xy, clamp_y(click_y + x), WHITE);
        horizontal_line(screen, start_xy, end_xy, clamp_y(click_y - x), WHITE);
    }
}

fn render_safe(screen: &mut [u32; WIDTH * HEIGHT]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            screen[p(x, y)] = SKY_BLUE;
        }
    }
}

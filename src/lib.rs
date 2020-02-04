mod common;
mod diffuse;
use common::{Color, Screen, HEIGHT, WIDTH};
use std::cmp;

const COBOLT_BLUE: Color = Color {
    r: 0,
    g: 71,
    b: 171,
    a: 255,
};

#[no_mangle]
static mut SCREEN: Screen = [0; WIDTH * HEIGHT];

// unfortunately exposed, so let's hope no_mangle keeps people away...
static mut DIFFUSION: diffuse::Diffusion = diffuse::Diffusion::new();

#[no_mangle]
pub extern "C" fn width() -> usize {
    WIDTH
}

#[no_mangle]
pub extern "C" fn height() -> usize {
    HEIGHT
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    for pixel in SCREEN.iter_mut() {
        *pixel = COBOLT_BLUE.into();
    }
    DIFFUSION.update(WIDTH / 2, HEIGHT / 2);
}

#[no_mangle]
pub unsafe extern "C" fn render() -> u32 {
    render_safe(&mut DIFFUSION, &mut SCREEN)
}

#[no_mangle]
pub unsafe extern "C" fn update(x: u32, y: u32, radius: u32) {
    update_safe(&mut DIFFUSION, x as usize, y as usize, radius as usize)
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

fn update_safe(diffusion: &mut diffuse::Diffusion, click_x: usize, click_y: usize, radius: usize) {
    let click_x = click_x as i64;
    let click_y = click_y as i64;
    let radius = radius as i64;

    let mut f = 1 - radius;
    let mut ddf_x = 1;
    let mut ddf_y = 2 * radius;
    let mut x = 0;
    let mut y = radius;

    diffusion.update(click_x as usize, clamp_y(click_y - radius));
    diffusion.update(click_x as usize, clamp_y(click_y + radius));
    {
        let start_x = clamp_x(click_x - radius);
        let end_x = clamp_x(click_x + radius);
        for x in start_x..end_x {
            diffusion.update(x, click_y as usize);
        }
    }

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
        for x in start_xx..end_xx {
            diffusion.update(x, clamp_y(click_y + y));
            diffusion.update(x, clamp_y(click_y - y));
        }

        let start_xy = clamp_x(click_x - y);
        let end_xy = clamp_x(click_x + y);
        for x in start_xy..end_xy {
            diffusion.update(x, clamp_y(click_y + y));
            diffusion.update(x, clamp_y(click_y - y));
        }
    }
}

fn render_safe(diffusion: &mut diffuse::Diffusion, screen: &mut Screen) -> u32 {
    diffusion.draw(screen, COBOLT_BLUE)
}

use crate::common::{p, Color, Screen, N};
use std::cmp;
use std::mem;

const SIZE: usize = (N + 2) * (N * 2);
type Buffer = [f64; SIZE];

fn ix(i: usize, j: usize) -> usize {
    i + (N + 2) * j
}

fn add_source(x: &mut Buffer, s: &Buffer, dt: f64) {
    for i in 0..SIZE {
        x[i] += dt * s[i];
    }
}

enum Axis {
    None,
    Y,
    X,
}

fn diffuse(b: Axis, x: &mut Buffer, x0: &Buffer, diff: f64, dt: f64) {
    let a = dt * diff * ((N * N) as f64);
    for _ in 0..20 {
        for i in 1..N + 1 {
            for j in 1..N + 1 {
                x[ix(i, j)] = (x0[ix(i, j)]
                    + a * (x[ix(i - 1, j)] + x[ix(i + 1, j)] + x[ix(i, j - 1)] + x[ix(i, j + 1)]))
                    / (1.0 + 4.0 * a)
            }
        }
    }
    set_bnd(b, x)
}

fn advect(b: Axis, d: &mut Buffer, d0: &Buffer, u: &Buffer, v: &Buffer, dt: f64) {
    let dt0 = dt * N as f64;
    for i in 1..N + 1 {
        for j in 1..N + 1 {
            let x = i as f64 - dt0 * u[ix(i, j)];
            let y = j as f64 - dt0 * v[ix(i, j)];
            let x = x.max(0.5).min(N as f64 + 0.5);
            let i0 = x as usize;
            let i1 = i0 + 1;
            let y = y.max(0.5).min(N as f64 + 0.5);
            let j0 = y as usize;
            let j1 = j0 + 1;
            let s1 = x - i0 as f64;
            let s0 = 1.0 - s1;
            let t1 = y - j0 as f64;
            let t0 = 1.0 - t1;
            d[ix(i, j)] = s0 * (t0 * d0[ix(i0, j0)] + t1 * d0[ix(i0, j1)])
                + s1 * (t0 * d0[ix(i1, j0)] + t1 * d0[ix(i1, j1)]);
        }
    }
    set_bnd(b, d);
}

fn dens_step(x: &mut Buffer, x0: &mut Buffer, u: &Buffer, v: &Buffer, diff: f64, dt: f64) {
    add_source(x, x0, dt);
    mem::swap(x0, x);
    diffuse(Axis::None, x, x0, diff, dt);
    mem::swap(x0, x);
    advect(Axis::None, x, x0, u, v, dt);
}

fn vel_step(u: &mut Buffer, v: &mut Buffer, u0: &mut Buffer, v0: &mut Buffer, visc: f64, dt: f64) {
    add_source(u, u0, dt);
    add_source(v, v0, dt);
    mem::swap(u0, u);
    diffuse(Axis::Y, u, u0, visc, dt);
    mem::swap(v0, v);
    diffuse(Axis::X, v, v0, visc, dt);
    project(u, v, u0, v0);
    mem::swap(u0, u);
    mem::swap(v0, v);
    advect(Axis::Y, u, u0, u0, v0, dt);
    advect(Axis::X, v, v0, u0, v0, dt);
    project(u, v, u0, v0);
}

fn project(u: &mut Buffer, v: &mut Buffer, proj: &mut Buffer, div: &mut Buffer) {
    let h = 1.0 / N as f64;
    for i in 1..N + 1 {
        for j in 1..N + 1 {
            div[ix(i, j)] =
                -0.5 * h * (u[ix(i + 1, j)] - u[ix(i - 1, j)] + v[ix(i, j + 1)] - v[ix(i, j - 1)]);
            proj[ix(i, j)] = 0.0;
        }
    }
    set_bnd(Axis::None, div);
    set_bnd(Axis::None, proj);

    for _ in 0..20 {
        for i in 1..N + 1 {
            for j in 1..N + 1 {
                proj[ix(i, j)] = (div[ix(i, j)]
                    + proj[ix(i - 1, j)]
                    + proj[ix(i + 1, j)]
                    + proj[ix(i, j - 1)]
                    + proj[ix(i, j + 1)])
                    / 4.0;
            }
        }
        set_bnd(Axis::None, proj);
    }

    for i in 1..N + 1 {
        for j in 1..N + 1 {
            u[ix(i, j)] -= 0.5 * (proj[ix(i + 1, j)] - proj[ix(i - 1, j - 1)]) / h;
            v[ix(i, j)] -= 0.5 * (proj[ix(i, j + 1)] - proj[ix(i, j - 1)]) / h;
        }
    }
    set_bnd(Axis::Y, u);
    set_bnd(Axis::X, v);
}

fn set_bnd(b: Axis, x: &mut Buffer) {
    for i in 0..N + 1 {
        x[ix(0, i)] = match b {
            Axis::Y => -x[ix(1, i)],
            _ => x[ix(1, i)],
        };
        x[ix(N + 1, i)] = match b {
            Axis::Y => -x[ix(N, i)],
            _ => x[ix(N, i)],
        };
        x[ix(i, 0)] = match b {
            Axis::X => -x[ix(i, 1)],
            _ => x[ix(i, 1)],
        };
        x[ix(i, N + 1)] = match b {
            Axis::X => -x[ix(i, N)],
            _ => x[ix(i, N)],
        };
    }
    x[ix(0, 0)] = 0.5 * (x[ix(1, 0)] + x[ix(0, 1)]);
    x[ix(0, N + 1)] = 0.5 * (x[ix(1, N + 1)] + x[ix(0, N)]);
    x[ix(N + 1, 0)] = 0.5 * (x[ix(N, 0)] + x[ix(N + 1, 1)]);
    x[ix(N + 1, N + 1)] = 0.5 * (x[ix(N, N + 1)] + x[ix(N + 1, N)]);
}

pub struct Diffusion {
    u: Buffer,
    v: Buffer,

    u_prev: Buffer,
    v_prev: Buffer,

    dens: Buffer,
    dens_prev: Buffer,

    prev_x: usize,
    prev_y: usize,
}

impl Diffusion {
    pub const fn new() -> Diffusion {
        Diffusion {
            u: [0.0; SIZE],
            v: [0.0; SIZE],
            u_prev: [0.0; SIZE],
            v_prev: [0.0; SIZE],
            dens: [0.0; SIZE],
            dens_prev: [0.0; SIZE],

            prev_x: 0,
            prev_y: 0,
        }
    }

    pub fn update(&mut self, x: usize, y: usize) {
        let dx = x - self.prev_x;
        let dy = y - self.prev_y;
        self.u[ix(x, y)] = dx as f64;
        self.v[ix(x, y)] = dy as f64;
        self.prev_x = x;
        self.prev_y = y;
        self.dens_prev[ix(x, y)] = 200.0;
    }

    pub fn draw(&mut self, screen: &mut Screen, color: Color) -> u32 {
        vel_step(
            &mut self.u,
            &mut self.v,
            &mut self.u_prev,
            &mut self.v_prev,
            0.01,
            1.0,
        );
        dens_step(
            &mut self.dens,
            &mut self.dens_prev,
            &self.u,
            &self.v,
            1.0,
            1.0,
        );
        for i in 0..N {
            for j in 0..N {
                let mut color = color.clone();
                let density = self.dens[ix(i, j)];
                color.a = 255 - cmp::min(255, (255.0 * density).ceil() as u8);
                screen[p(i, j)] = color.into();
            }
        }
        color.into()
    }
}

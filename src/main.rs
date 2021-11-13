use core::fmt::Debug;
use num::{integer::sqrt, Num, Saturating};

mod endpoint;
use endpoint::*;

#[cfg(test)]
mod test {
    use crate::{interpolate, linspace};

    #[test]
    fn test_linspace() {
        assert_eq!(
            linspace::<usize, 3>(1, 3).collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn test_interpolate() {
        assert!(f32::abs(interpolate(0f32, -1f32, 2f32) - 1f32 / 3f32) < f32::EPSILON)
    }
}

fn linspace<N: Copy, const COUNT: usize>(a: N, b: N) -> impl ExactSizeIterator<Item = N>
where
    N: Num,
    usize: Into<N>,
{
    let step = (a + b) / COUNT.into();
    (0..COUNT).into_iter().map(move |idx| idx.into() * step + a)
}

fn sample_points<
    N: Num + Copy,
    Endpoint: Copy + Debug,
    F: Fn(N, N) -> N,
    const W: usize,
    const H: usize,
>(
    f: F,
    (xmin, xmax): (Endpoint, Endpoint),
    (ymin, ymax): (Endpoint, Endpoint),
    result: &mut [[N; W]; H],
) where
    Endpoint: Num,
    Endpoint: From<usize>,
    Endpoint: Into<N>,
{
    for (j, y) in linspace::<Endpoint, H>(ymin, ymax).enumerate() {
        for (i, x) in linspace::<Endpoint, W>(xmin, xmax).enumerate() {
            result[j][i] = f(x.into(), y.into());
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Line<N> {
    a: (N, N),
    b: (N, N),
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Contour<N> {
    None,
    One(Line<N>),
    Two(Line<N>, Line<N>),
}

struct ContourIterator<N>(Contour<N>);

impl<N> Iterator for ContourIterator<N>
where
    N: Copy,
{
    type Item = Line<N>;

    fn next(&mut self) -> Option<Self::Item> {
        let (line, next) = match self.0 {
            Contour::None => (None, Contour::None),
            Contour::One(a) => (Some(a), Contour::None),
            Contour::Two(a, b) => (Some(a), Contour::One(b)),
        };
        self.0 = next;
        line
    }
}

impl<N> Contour<N>
where
    N: Copy,
{
    pub fn lines(&self) -> impl IntoIterator<Item = Line<N>> {
        ContourIterator(*self)
    }
}

fn interpolate<N>(target: N, a: N, b: N) -> N
where
    N: Num + Copy,
{
    let da = target - a;
    let db = b - target;
    da / (da + db)
}

fn find_contour<N>(contour: N, tl: N, tr: N, bl: N, br: N) -> Contour<N>
where
    N: Num + PartialOrd + Copy,
{
    match (tl > contour, tr > contour, bl > contour, br > contour) {
        (false, false, false, false) | (true, true, true, true) => Contour::None,
        (false, false, false, true) | (true, true, true, false) => Contour::One(Line {
            a: (interpolate(contour, bl, br), N::one()),
            b: (N::one(), interpolate(contour, tr, br)),
        }),
        (true, true, false, true) | (false, false, true, false) => Contour::One(Line {
            a: (interpolate(contour, bl, br), N::one()),
            b: (N::zero(), interpolate(contour, tl, bl)),
        }),
        (false, false, true, true) | (true, true, false, false) => Contour::One(Line {
            a: (N::zero(), interpolate(contour, tl, bl)),
            b: (N::one(), interpolate(contour, tr, br)),
        }),
        (false, true, false, true) | (true, false, true, false) => Contour::One(Line {
            a: (interpolate(contour, tl, tr), N::zero()),
            b: (interpolate(contour, bl, br), N::one()),
        }),
        (true, false, true, true) | (false, true, false, false) => Contour::One(Line {
            a: (N::one(), interpolate(contour, tr, br)),
            b: (interpolate(contour, tl, tr), N::zero()),
        }),
        (true, false, false, false) | (false, true, true, true) => Contour::One(Line {
            a: (interpolate(contour, tl, tr), N::zero()),
            b: (N::zero(), interpolate(contour, tl, bl)),
        }),
        (true, false, false, true) => Contour::Two(
            Line {
                a: (interpolate(contour, tl, tr), N::zero()),
                b: (N::one(), interpolate(contour, tr, br)),
            },
            Line {
                a: (N::zero(), interpolate(contour, tl, bl)),
                b: (interpolate(contour, bl, br), N::one()),
            },
        ),
        (false, true, true, false) => Contour::Two(
            Line {
                a: (N::zero(), interpolate(contour, tl, bl)),
                b: (interpolate(contour, tl, tr), N::zero()),
            },
            Line {
                a: (interpolate(contour, bl, br), N::one()),
                b: (N::one(), interpolate(contour, tr, br)),
            },
        ),
    }
}

fn draw_line<const W: usize, const H: usize>(
    color: (u8, u8, u8),
    (x1, y1): (i32, i32),
    (x2, y2): (i32, i32),
    result: &mut [[(u8, u8, u8); W]; H],
) {
    let len = sqrt((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2));

    if len == 0 {
        result[y1 as usize][x1 as usize] = color;
    } else {
        for t in 0..=len {
            let x = x1 + (x2 - x1) * t / len;
            let y = y1 + (y2 - y1) * t / len;
            result[y as usize][x as usize] = color;
        }
    }
}

fn find_all_lines<
    'a,
    N: Num + Copy + PartialOrd,
    Cvt: Fn(usize) -> N + 'a + Copy,
    const W: usize,
    const H: usize,
>(
    threshold: N,
    scale: N,
    cvt: Cvt,
    result: &'a [[N; W]; H],
) -> impl Iterator<Item = Line<N>> + 'a {
    (0..(H - 1)).into_iter().flat_map(move |y| {
        (0..(W - 1)).into_iter().flat_map(move |x| {
            find_contour(
                threshold,
                result[y][x],
                result[y][x + 1],
                result[y + 1][x],
                result[y + 1][x + 1],
            )
            .lines()
            .into_iter()
            .map(
                move |Line {
                          a: (x1, y1),
                          b: (x2, y2),
                      }| Line {
                    a: ((x1 + cvt(x)) * scale, (y1 + cvt(y)) * scale),
                    b: ((x2 + cvt(x)) * scale, (y2 + cvt(y)) * scale),
                },
            )
        })
    })
}

struct Blob {
    center: (f32, f32),
    velocity: (f32, f32),
    size: f32,
}

fn step<'a>((width, height): (f32, f32), blobs: impl Iterator<Item = &'a mut Blob>) {
    for blob in blobs {
        blob.center.0 += blob.velocity.0 / 10.0;
        blob.center.1 += blob.velocity.1 / 10.0;

        let mut extra_step = false;
        if blob.center.0 + blob.size > width || blob.center.0 - blob.size < 0.0 {
            blob.velocity.0 = -blob.velocity.0;
            extra_step = true;
        }
        if blob.center.1 + blob.size > height || blob.center.1 - blob.size < 0.0 {
            blob.velocity.1 = -blob.velocity.1;
            extra_step = true;
        }
        if extra_step {
            blob.center.0 += blob.velocity.0 / 10.0;
            blob.center.1 += blob.velocity.1 / 10.0;
        }
    }
}

fn energy<'a>((x, y): (f32, f32), blobs: impl Iterator<Item = &'a Blob>) -> f32 {
    blobs
        .map(
            |Blob {
                 center: (cx, cy),
                 velocity: _,
                 size,
             }| { size / f32::sqrt((x - cx) * (x - cx) + (y - cy) * (y - cy)) },
        )
        .sum()
}

fn main() {
    const WIDTH: usize = 50;
    const HEIGHT: usize = 50;
    const SCALE: usize = 10;
    const NUM_FRAMES: usize = 1000;

    // For reading and opening files
    use std::fs::File;
    use std::io::BufWriter;
    use std::path::Path;

    let path = Path::new("result.png");
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, (WIDTH * SCALE) as u32, (HEIGHT * SCALE) as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_animated(NUM_FRAMES as u32, 0).unwrap();

    let mut writer = encoder.write_header().unwrap();

    let mut img = [[(0xff, 0xff, 0xff); WIDTH * SCALE]; HEIGHT * SCALE];

    let mut blobs = vec![
        Blob {
            center: (1.0, 1.0),
            velocity: (1.0, 0.7),
            size: f32::sqrt(0.6),
        },
        Blob {
            center: (4.0, 6.0),
            velocity: (-2.0, 1.0),
            size: f32::sqrt(0.3),
        },
        Blob {
            center: (6.0, 2.0),
            velocity: (-0.7, -0.2),
            size: f32::sqrt(0.4),
        },
        Blob {
            center: (8.0, 4.0),
            velocity: (0.4, -1.4),
            size: f32::sqrt(0.1),
        },
    ];

    for _ in 0..NUM_FRAMES {
        img.iter_mut().flatten().for_each(|(r, g, b)| {
            *r = r.saturating_add(40);
            *g = g.saturating_add(40);
            *b = b.saturating_add(40);
        });

        let mut result = [[0f32; WIDTH]; HEIGHT];
        let mut img_data = [[0f32; WIDTH * SCALE]; HEIGHT * SCALE];

        let f = |x, y| energy((x, y), blobs.iter());
        let x_rng = (F32(0f32), F32(10f32));
        let y_rng = (F32(0f32), F32(10f32));

        sample_points(f, x_rng, y_rng, &mut img_data);

        sample_points(f, x_rng, y_rng, &mut result);

        for line in find_all_lines(1.0, SCALE as f32, |x| x as f32, &result) {
            let x1 = line.a.0 as i32;
            let y1 = line.a.1 as i32;
            let x2 = line.b.0 as i32;
            let y2 = line.b.1 as i32;

            draw_line((0, 0, 0xff), (x1, y1), (x2, y2), &mut img);
        }

        step((10.0, 10.0), blobs.iter_mut());

        let ptr: *const u8 = unsafe { core::mem::transmute(img.as_ptr()) };

        writer
            .write_image_data(unsafe {
                std::slice::from_raw_parts(ptr, WIDTH * HEIGHT * SCALE * SCALE * 3)
            })
            .unwrap(); // Save
    }
}

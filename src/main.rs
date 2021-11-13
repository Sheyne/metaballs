use core::fmt::Debug;
use num::{integer::sqrt, Num};

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

fn colorize<Num: Copy, F: Fn(Num) -> (u8, u8, u8), const W: usize, const H: usize>(
    f: F,
    src: &[[Num; W]; H],
    result: &mut [[(u8, u8, u8); W]; H],
) {
    for j in 0..H {
        for i in 0..W {
            result[j][i] = f(src[j][i]);
        }
    }
}

fn save_png<const W: usize, const H: usize>(path: &str, img: &[[(u8, u8, u8); W]; H]) {
    // For reading and opening files
    use std::fs::File;
    use std::io::BufWriter;
    use std::path::Path;

    let path = Path::new(path);
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, W as u32, H as u32); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    let res: Vec<u8> = img
        .iter()
        .flat_map(|r| r.iter().flat_map(|(r, g, b)| [*r, *g, *b]))
        .collect();

    writer.write_image_data(&res).unwrap(); // Save
}

fn main() {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;
    const SCALE: usize = 25;

    let mut result = [[0f32; WIDTH]; HEIGHT];
    let mut big_result = [[0f32; WIDTH * SCALE]; HEIGHT * SCALE];
    let mut img = [[(0xff, 0xff, 0xff); WIDTH * SCALE]; HEIGHT * SCALE];

    let f = |x, y| ((x - 5f32) * (x - 5f32) + (y - 5f32) * (y - 5f32)) / 10f32 * 10f32;
    let x_rng = (F32(0f32), F32(10f32));
    let y_rng = (F32(0f32), F32(10f32));

    sample_points(f, x_rng, y_rng, &mut big_result);

    colorize(|x| (0, 0xbb, (x * 10f32) as u8), &big_result, &mut img);

    sample_points(f, x_rng, y_rng, &mut result);

    for y in 0..(HEIGHT - 1) {
        for x in 0..(WIDTH - 1) {
            let contour = find_contour(
                10f32,
                result[y][x],
                result[y][x + 1],
                result[y + 1][x],
                result[y + 1][x + 1],
            );
            for Line {
                a: (x1, y1),
                b: (x2, y2),
            } in contour.lines().into_iter().map(
                |Line {
                     a: (x1, y1),
                     b: (x2, y2),
                 }| Line {
                    a: (
                        ((x1 + x as f32) * SCALE as f32) as usize,
                        ((y1 + y as f32) * SCALE as f32) as usize,
                    ),
                    b: (
                        ((x2 + x as f32) * SCALE as f32) as usize,
                        ((y2 + y as f32) * SCALE as f32) as usize,
                    ),
                },
            ) {
                draw_line(
                    (0xff, 0, 0),
                    (x1 as i32, y1 as i32),
                    (x2 as i32, y2 as i32),
                    &mut img,
                );
                save_png("result.png", &img);
            }
        }
    }
}

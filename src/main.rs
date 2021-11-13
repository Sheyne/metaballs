use core::{
    fmt::Debug,
    ops::{Add, Div, Mul},
};

mod num;
use num::*;

#[cfg(test)]
mod test {
    use crate::linspace;

    #[test]
    fn test_linspace() {
        assert_eq!(
            linspace::<usize, 3>(1, 3).collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
    }
}

fn linspace<Num: Copy, const COUNT: usize>(a: Num, b: Num) -> impl ExactSizeIterator<Item = Num>
where
    Num: Add<Num, Output = Num>,
    Num: Mul<Num, Output = Num>,
    Num: Div<Num, Output = Num>,
    usize: Into<Num>,
{
    let step = (a + b) / COUNT.into();
    (0..COUNT).into_iter().map(move |idx| idx.into() * step + a)
}

fn sample_points<Num: Copy + Debug, F: Fn(Num, Num) -> Num, const W: usize, const H: usize>(
    f: F,
    (xmin, xmax): (Num, Num),
    (ymin, ymax): (Num, Num),
    result: &mut [[Num; W]; H],
) where
    Num: Add<Num, Output = Num>,
    Num: Mul<Num, Output = Num>,
    Num: Div<Num, Output = Num>,
    usize: Into<Num>,
{
    for (j, y) in linspace::<Num, H>(ymin, ymax).enumerate() {
        for (i, x) in linspace::<Num, W>(xmin, xmax).enumerate() {
            result[j][i] = f(x, y);
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
    const WIDTH: usize = 100;
    const HEIGHT: usize = 100;

    let mut result = [[F32(0f32); WIDTH]; HEIGHT];
    let mut img = [[(0, 0, 0); WIDTH]; HEIGHT];

    sample_points(
        |x, y| x * x / F32(10f32) + y,
        (F32(0f32), F32(5f32)),
        (F32(0f32), F32(10f32)),
        &mut result,
    );

    colorize(
        |x| {
            (
                (x.0 * 30f32) as u8,
                (x.0 * 30f32) as u8,
                (x.0 * 30f32) as u8,
            )
        },
        &result,
        &mut img,
    );

    save_png("result.png", &img);
}

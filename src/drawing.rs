use std::mem::swap;

// https://en.wikipedia.org/wiki/Xiaolin_Wu's_line_algorithm

// integer part of x
fn ipart(x: f32) -> f32 {
    f32::floor(x)
}

// fractional part of x
fn fpart(x: f32) -> f32 {
    x - f32::floor(x)
}

fn rfpart(x: f32) -> f32 {
    1.0 - fpart(x)
}

pub fn draw_line<const W: usize, const H: usize>(
    (r, g, b): (u8, u8, u8),
    (mut x0, mut y0): (f32, f32),
    (mut x1, mut y1): (f32, f32),
    result: &mut [[(u8, u8, u8); W]; H],
) {
    let mut plot = |x: f32, y: f32, c: f32| {
        let (er, eg, eb) = result[y as usize][x as usize];
        result[y as usize][x as usize].0 = (er as f32 + (r as f32 - er as f32) * c) as u8;
        result[y as usize][x as usize].1 = (eg as f32 + (g as f32 - eg as f32) * c) as u8;
        result[y as usize][x as usize].2 = (eb as f32 + (b as f32 - eb as f32) * c) as u8;
    };

    let steep = f32::abs(y1 - y0) > f32::abs(x1 - x0);

    if steep {
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
    }
    if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let gradient = if dx == 0.0 { 1.0 } else { dy / dx };

    // handle first endpoint
    let xend = f32::round(x0);
    let yend = y0 + gradient * (xend - x0);
    let xgap = rfpart(x0 + 0.5);
    let xpxl1 = xend; // this will be used in the main loop
    let ypxl1 = ipart(yend);
    if steep {
        plot(ypxl1, xpxl1, rfpart(yend) * xgap);
        plot(ypxl1 + 1.0, xpxl1, fpart(yend) * xgap);
    } else {
        plot(xpxl1, ypxl1, rfpart(yend) * xgap);
        plot(xpxl1, ypxl1 + 1.0, fpart(yend) * xgap);
    }
    let mut intery = yend + gradient; // first y-intersection for the main loop

    // handle second endpoint
    let xend = f32::round(x1);
    let yend = y1 + gradient * (xend - x1);
    let xgap = fpart(x1 + 0.5);
    let xpxl2 = xend; //this will be used in the main loop
    let ypxl2 = ipart(yend);
    if steep {
        plot(ypxl2, xpxl2, rfpart(yend) * xgap);
        plot(ypxl2 + 1.0, xpxl2, fpart(yend) * xgap);
    } else {
        plot(xpxl2, ypxl2, rfpart(yend) * xgap);
        plot(xpxl2, ypxl2 + 1.0, fpart(yend) * xgap);
    }

    // main loop
    if steep {
        for x in ((xpxl1 + 1.0) as usize)..(xpxl2 as usize) {
            plot(ipart(intery), x as f32, rfpart(intery));
            plot(ipart(intery) + 1.0, x as f32, fpart(intery));
            intery += gradient;
        }
    } else {
        for x in ((xpxl1 + 1.0) as usize)..(xpxl2 as usize) {
            plot(x as f32, ipart(intery), rfpart(intery));
            plot(x as f32, ipart(intery) + 1.0, fpart(intery));
            intery += gradient;
        }
    }
}

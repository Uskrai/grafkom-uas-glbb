pub fn calculate_distance(
    velocity: f64,
    acceleration: f64,
    time: f64,
) -> f64 {
    velocity * time + (0.5) * acceleration * (time * time)
}

pub fn calculate_velocity(
    velocity: f64,
    acceleration: f64,
    time: f64,
) -> f64 {
    velocity + acceleration * time
}

pub fn mid_point(
    x_center: usize,
    y_center: usize,
    radius: usize,
) -> Vec<(usize, usize)> {
    let mut x = radius;
    let mut y = 0;
    let mut points = vec![];

    let x_y = |x: usize, y: usize| {
        let x_add = x_center + x;
        let y_add = y_center + y;
        let x_min = x_center.checked_sub(x).unwrap();
        let y_min = y_center.checked_sub(y).unwrap();
        [
            (x_add, y_add),
            (x_min, y_add),
            (x_add, y_min),
            (x_min, y_min),
        ]
    };

    if radius > 0 {
        points.extend(
            x_y(x, y)
                .iter()
                .take(2)
                .chain(x_y(y, x).iter().take(2)),
        );
    } else {
        points.push(x_y(x, y)[0]);
    }

    let mut p = 1 - radius as isize;
    while x > y {
        y += 1;

        if p <= 0 {
            p = p + 2 * y as isize + 1;
        } else {
            x -= 1;
            p = p + (2 * y as isize) - (2 * x as isize) + 1;
        }

        // semua point sudah di tambahin
        if x < y {
            break;
        }

        points.extend(x_y(x, y));

        // kalo x sama dengan y maka titiknya udah ditambahkan
        if x != y {
            points.extend(x_y(y, x));
        }
    }

    points
}

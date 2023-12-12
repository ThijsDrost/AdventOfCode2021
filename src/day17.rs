pub(crate) fn day17() {
    // let target = ((20, 30), (-10, -5));
    let target = ((217, 240), (-126, -69));
    let mut total = 0;
    let mut max = 0;
    for vy_init in target.1.0..(-target.1.0) {
        for vx_init in 0..=target.0.1 {
            let mut x = 0;
            let mut y = 0;
            let mut vx = vx_init;
            let mut vy = vy_init;

            while y > target.1.0 {
                x += vx;
                y += vy;
                if vx < 0 { vx += 1;}
                else if vx > 0 { vx -= 1;}
                vy -= 1;
                if (target.0.0 <= x) && (x <= target.0.1) && (target.1.0 <= y) && (y <= target.1.1) {
                    if vy_init * (vy_init + 1) / 2 > max {
                        max = vy_init * (vy_init + 1) / 2;
                    }
                    total += 1;
                    break;
                }
            }
        }
    }
    println!("Day 17, part 1: {}", max);
    println!("Day 17, part 2: {}", total);
}
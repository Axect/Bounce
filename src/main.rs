use peroxide::fuga::*;
use rayon::prelude::*;
use std::cmp::Ordering;

fn main() {
    let u = Uniform(0f64, 1f64);
    let b = Bernoulli(0.5);
    let mut vs = vec![vec![0f64; 100]; 10000];
    let phi = linspace(0, 1, 100);

    // Parallel generation via rayon
    vs.par_iter_mut().for_each(|v| loop {
        let mut ps = u.sample(4);
        ps.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let p = b.sample(1)[0];

        let (phi_1p, mut phi_0, mut phi_2, phi_1n) = (ps[0], ps[1], ps[2], ps[3]);

        if p < 0.5 {
            std::mem::swap(&mut phi_0, &mut phi_2);
        }

        let f = c1_potential(phi_0, phi_1n, phi_1p, phi_2);
        let v_cand = phi.fmap(f);
        let v_max = v_cand.max();
        if v_max < 0.01 || v_max > 10f64.powf(-0.5) {
            continue;
        } else {
            let (count_max, count_min) = count_local_extrema(&v_cand);
            if count_max > 1 || count_min > 2 {
                continue;
            }
            *v = v_cand;
            break;
        }
    });

    let mut df = DataFrame::new(vec![]);
    for (i, v) in vs.into_iter().enumerate() {
        df.push(&format!("v{}", i), Series::new(v));
    }
    df.write_parquet("c1.parquet", CompressionOptions::Uncompressed)
        .unwrap();

    println!("done");
}

fn count_local_extrema(ys: &[f64]) -> (usize, usize) {
    let mut count_maxima = 0;
    let mut count_minima = 0;

    ys.iter().enumerate().for_each(|(i, y)| {
        if i == 0 {
            if y > &ys[1] {
                count_maxima += 1;
            } else if y < &ys[1] {
                count_minima += 1;
            }
        } else if i == ys.len() - 1 {
            if y > &ys[i - 1] {
                count_maxima += 1;
            } else if y < &ys[i - 1] {
                count_minima += 1;
            }
        } else {
            match y.partial_cmp(&ys[i - 1]).unwrap() {
                Ordering::Greater => {
                    if y > &ys[i + 1] {
                        count_maxima += 1;
                    }
                }
                Ordering::Less => {
                    if y < &ys[i + 1] {
                        count_minima += 1;
                    }
                }
                _ => {} // This case covers equal elements, where neither condition is met.
            }
        }
    });

    (count_maxima, count_minima)
}

fn c1_potential(phi_0: f64, phi_1n: f64, phi_1p: f64, phi_2: f64) -> Box<dyn Fn(f64) -> f64> {
    Box::new(move |phi: f64| {
        phi.powi(2)
            * (phi.powi(5)
                * (-720f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p.powi(2)
                    + 1440f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p * phi_2
                    - 720f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_2
                    + 240f64 * phi_0.powi(3) * phi_1n.powi(2)
                    + 1440f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(2) * phi_2
                    - 2160f64 * phi_0.powi(3) * phi_1n * phi_1p * phi_2.powi(2)
                    - 720f64 * phi_0.powi(3) * phi_1n * phi_1p * phi_2
                    + 240f64 * phi_0.powi(3) * phi_1n * phi_1p
                    + 1080f64 * phi_0.powi(3) * phi_1n * phi_2.powi(2)
                    - 180f64 * phi_0.powi(3) * phi_1n
                    - 720f64 * phi_0.powi(3) * phi_1p.powi(2) * phi_2
                    + 240f64 * phi_0.powi(3) * phi_1p.powi(2)
                    + 1080f64 * phi_0.powi(3) * phi_1p * phi_2.powi(2)
                    - 180f64 * phi_0.powi(3) * phi_1p
                    - 720f64 * phi_0.powi(3) * phi_2.powi(2)
                    + 360f64 * phi_0.powi(3) * phi_2
                    + 900f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(2)
                    - 1800f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p * phi_2
                    + 900f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_2
                    - 300f64 * phi_0.powi(2) * phi_1n.powi(3)
                    + 900f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(3)
                    - 1800f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(2) * phi_2
                    - 720f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(2)
                    + 2340f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p * phi_2
                    - 300f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p
                    - 720f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_2
                    + 240f64 * phi_0.powi(2) * phi_1n.powi(2)
                    - 1800f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(3) * phi_2
                    + 2340f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(2) * phi_2
                    - 300f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(2)
                    + 3600f64 * phi_0.powi(2) * phi_1n * phi_1p * phi_2.powi(3)
                    - 2160f64 * phi_0.powi(2) * phi_1n * phi_1p * phi_2.powi(2)
                    - 720f64 * phi_0.powi(2) * phi_1n * phi_1p * phi_2
                    + 240f64 * phi_0.powi(2) * phi_1n * phi_1p
                    - 1800f64 * phi_0.powi(2) * phi_1n * phi_2.powi(3)
                    + 1080f64 * phi_0.powi(2) * phi_1n * phi_2.powi(2)
                    + 900f64 * phi_0.powi(2) * phi_1p.powi(3) * phi_2
                    - 300f64 * phi_0.powi(2) * phi_1p.powi(3)
                    - 720f64 * phi_0.powi(2) * phi_1p.powi(2) * phi_2
                    + 240f64 * phi_0.powi(2) * phi_1p.powi(2)
                    - 1800f64 * phi_0.powi(2) * phi_1p * phi_2.powi(3)
                    + 1080f64 * phi_0.powi(2) * phi_1p * phi_2.powi(2)
                    + 1200f64 * phi_0.powi(2) * phi_2.powi(3)
                    - 720f64 * phi_0.powi(2) * phi_2.powi(2)
                    - 1200f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(3)
                    + 900f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(2)
                    + 3600f64 * phi_0 * phi_1n.powi(3) * phi_1p * phi_2.powi(2)
                    - 1800f64 * phi_0 * phi_1n.powi(3) * phi_1p * phi_2
                    - 1800f64 * phi_0 * phi_1n.powi(3) * phi_2.powi(2)
                    + 900f64 * phi_0 * phi_1n.powi(3) * phi_2
                    + 900f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(3)
                    + 3600f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(2)
                    - 1800f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2
                    - 720f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(2)
                    - 4800f64 * phi_0 * phi_1n.powi(2) * phi_1p * phi_2.powi(3)
                    - 1800f64 * phi_0 * phi_1n.powi(2) * phi_1p * phi_2.powi(2)
                    + 2340f64 * phi_0 * phi_1n.powi(2) * phi_1p * phi_2
                    + 2400f64 * phi_0 * phi_1n.powi(2) * phi_2.powi(3)
                    - 720f64 * phi_0 * phi_1n.powi(2) * phi_2
                    + 3600f64 * phi_0 * phi_1n * phi_1p.powi(3) * phi_2.powi(2)
                    - 1800f64 * phi_0 * phi_1n * phi_1p.powi(3) * phi_2
                    - 4800f64 * phi_0 * phi_1n * phi_1p.powi(2) * phi_2.powi(3)
                    - 1800f64 * phi_0 * phi_1n * phi_1p.powi(2) * phi_2.powi(2)
                    + 2340f64 * phi_0 * phi_1n * phi_1p.powi(2) * phi_2
                    + 6000f64 * phi_0 * phi_1n * phi_1p * phi_2.powi(3)
                    - 2160f64 * phi_0 * phi_1n * phi_1p * phi_2.powi(2)
                    - 720f64 * phi_0 * phi_1n * phi_1p * phi_2
                    - 1800f64 * phi_0 * phi_1n * phi_2.powi(3)
                    + 1080f64 * phi_0 * phi_1n * phi_2.powi(2)
                    - 1800f64 * phi_0 * phi_1p.powi(3) * phi_2.powi(2)
                    + 900f64 * phi_0 * phi_1p.powi(3) * phi_2
                    + 2400f64 * phi_0 * phi_1p.powi(2) * phi_2.powi(3)
                    - 720f64 * phi_0 * phi_1p.powi(2) * phi_2
                    - 1800f64 * phi_0 * phi_1p * phi_2.powi(3)
                    + 1080f64 * phi_0 * phi_1p * phi_2.powi(2)
                    + 3600f64 * phi_1n.powi(3) * phi_1p.powi(3) * phi_2
                    - 1200f64 * phi_1n.powi(3) * phi_1p.powi(3)
                    - 5400f64 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2.powi(2)
                    + 900f64 * phi_1n.powi(3) * phi_1p.powi(2)
                    + 3600f64 * phi_1n.powi(3) * phi_1p * phi_2.powi(2)
                    - 1800f64 * phi_1n.powi(3) * phi_1p * phi_2
                    - 5400f64 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2.powi(2)
                    + 900f64 * phi_1n.powi(2) * phi_1p.powi(3)
                    + 7200f64 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(3)
                    + 3600f64 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(2)
                    - 1800f64 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2
                    - 720f64 * phi_1n.powi(2) * phi_1p.powi(2)
                    - 4800f64 * phi_1n.powi(2) * phi_1p * phi_2.powi(3)
                    + 1440f64 * phi_1n.powi(2) * phi_1p * phi_2
                    + 3600f64 * phi_1n * phi_1p.powi(3) * phi_2.powi(2)
                    - 1800f64 * phi_1n * phi_1p.powi(3) * phi_2
                    - 4800f64 * phi_1n * phi_1p.powi(2) * phi_2.powi(3)
                    + 1440f64 * phi_1n * phi_1p.powi(2) * phi_2
                    + 3600f64 * phi_1n * phi_1p * phi_2.powi(3)
                    - 2160f64 * phi_1n * phi_1p * phi_2.powi(2))
                + phi.powi(4)
                    * (840f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_1p.powi(2)
                        - 1680f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_1p * phi_2
                        + 840f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_2
                        - 280f64 * phi_0.powi(4) * phi_1n.powi(2)
                        - 1680f64 * phi_0.powi(4) * phi_1n * phi_1p.powi(2) * phi_2
                        + 2520f64 * phi_0.powi(4) * phi_1n * phi_1p * phi_2.powi(2)
                        + 840f64 * phi_0.powi(4) * phi_1n * phi_1p * phi_2
                        - 280f64 * phi_0.powi(4) * phi_1n * phi_1p
                        - 1260f64 * phi_0.powi(4) * phi_1n * phi_2.powi(2)
                        + 210f64 * phi_0.powi(4) * phi_1n
                        + 840f64 * phi_0.powi(4) * phi_1p.powi(2) * phi_2
                        - 280f64 * phi_0.powi(4) * phi_1p.powi(2)
                        - 1260f64 * phi_0.powi(4) * phi_1p * phi_2.powi(2)
                        + 210f64 * phi_0.powi(4) * phi_1p
                        + 840f64 * phi_0.powi(4) * phi_2.powi(2)
                        - 420f64 * phi_0.powi(4) * phi_2
                        + 840f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p.powi(2)
                        - 1680f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p * phi_2
                        + 840f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_2
                        - 280f64 * phi_0.powi(3) * phi_1n.powi(2)
                        - 1680f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(2) * phi_2
                        + 2520f64 * phi_0.powi(3) * phi_1n * phi_1p * phi_2.powi(2)
                        + 840f64 * phi_0.powi(3) * phi_1n * phi_1p * phi_2
                        - 280f64 * phi_0.powi(3) * phi_1n * phi_1p
                        - 1260f64 * phi_0.powi(3) * phi_1n * phi_2.powi(2)
                        + 210f64 * phi_0.powi(3) * phi_1n
                        + 840f64 * phi_0.powi(3) * phi_1p.powi(2) * phi_2
                        - 280f64 * phi_0.powi(3) * phi_1p.powi(2)
                        - 1260f64 * phi_0.powi(3) * phi_1p * phi_2.powi(2)
                        + 210f64 * phi_0.powi(3) * phi_1p
                        + 840f64 * phi_0.powi(3) * phi_2.powi(2)
                        - 420f64 * phi_0.powi(3) * phi_2
                        - 1260f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_1p.powi(2)
                        + 2520f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_1p * phi_2
                        - 1260f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_2
                        + 420f64 * phi_0.powi(2) * phi_1n.powi(4)
                        - 1260f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(3)
                        + 2520f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(2) * phi_2
                        - 1260f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p * phi_2
                        + 420f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p
                        - 1260f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(4)
                        + 2520f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(3) * phi_2
                        - 1260f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(2) * phi_2
                        + 1260f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(2)
                        - 1680f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p * phi_2
                        + 840f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_2
                        - 280f64 * phi_0.powi(2) * phi_1n.powi(2)
                        + 2520f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(4) * phi_2
                        - 1260f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(3) * phi_2
                        + 420f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(3)
                        - 1680f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(2) * phi_2
                        - 6300f64 * phi_0.powi(2) * phi_1n * phi_1p * phi_2.powi(4)
                        + 2520f64 * phi_0.powi(2) * phi_1n * phi_1p * phi_2.powi(2)
                        + 840f64 * phi_0.powi(2) * phi_1n * phi_1p * phi_2
                        - 280f64 * phi_0.powi(2) * phi_1n * phi_1p
                        + 3150f64 * phi_0.powi(2) * phi_1n * phi_2.powi(4)
                        - 1260f64 * phi_0.powi(2) * phi_1n * phi_2.powi(2)
                        - 1260f64 * phi_0.powi(2) * phi_1p.powi(4) * phi_2
                        + 420f64 * phi_0.powi(2) * phi_1p.powi(4)
                        + 840f64 * phi_0.powi(2) * phi_1p.powi(2) * phi_2
                        - 280f64 * phi_0.powi(2) * phi_1p.powi(2)
                        + 3150f64 * phi_0.powi(2) * phi_1p * phi_2.powi(4)
                        - 1260f64 * phi_0.powi(2) * phi_1p * phi_2.powi(2)
                        - 2100f64 * phi_0.powi(2) * phi_2.powi(4)
                        + 840f64 * phi_0.powi(2) * phi_2.powi(2)
                        + 1680f64 * phi_0 * phi_1n.powi(4) * phi_1p.powi(3)
                        - 1260f64 * phi_0 * phi_1n.powi(4) * phi_1p.powi(2)
                        - 5040f64 * phi_0 * phi_1n.powi(4) * phi_1p * phi_2.powi(2)
                        + 2520f64 * phi_0 * phi_1n.powi(4) * phi_1p * phi_2
                        + 2520f64 * phi_0 * phi_1n.powi(4) * phi_2.powi(2)
                        - 1260f64 * phi_0 * phi_1n.powi(4) * phi_2
                        + 1680f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(4)
                        - 1260f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(3)
                        - 5040f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2.powi(2)
                        + 2520f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2
                        + 2520f64 * phi_0 * phi_1n.powi(3) * phi_1p * phi_2.powi(2)
                        - 1260f64 * phi_0 * phi_1n.powi(3) * phi_1p * phi_2
                        - 1260f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(4)
                        - 5040f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2.powi(2)
                        + 2520f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2
                        + 2520f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(2)
                        - 1260f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2
                        + 840f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(2)
                        + 8400f64 * phi_0 * phi_1n.powi(2) * phi_1p * phi_2.powi(4)
                        - 1680f64 * phi_0 * phi_1n.powi(2) * phi_1p * phi_2
                        - 4200f64 * phi_0 * phi_1n.powi(2) * phi_2.powi(4)
                        + 840f64 * phi_0 * phi_1n.powi(2) * phi_2
                        - 5040f64 * phi_0 * phi_1n * phi_1p.powi(4) * phi_2.powi(2)
                        + 2520f64 * phi_0 * phi_1n * phi_1p.powi(4) * phi_2
                        + 2520f64 * phi_0 * phi_1n * phi_1p.powi(3) * phi_2.powi(2)
                        - 1260f64 * phi_0 * phi_1n * phi_1p.powi(3) * phi_2
                        + 8400f64 * phi_0 * phi_1n * phi_1p.powi(2) * phi_2.powi(4)
                        - 1680f64 * phi_0 * phi_1n * phi_1p.powi(2) * phi_2
                        - 10500f64 * phi_0 * phi_1n * phi_1p * phi_2.powi(4)
                        + 2520f64 * phi_0 * phi_1n * phi_1p * phi_2.powi(2)
                        + 840f64 * phi_0 * phi_1n * phi_1p * phi_2
                        + 3150f64 * phi_0 * phi_1n * phi_2.powi(4)
                        - 1260f64 * phi_0 * phi_1n * phi_2.powi(2)
                        + 2520f64 * phi_0 * phi_1p.powi(4) * phi_2.powi(2)
                        - 1260f64 * phi_0 * phi_1p.powi(4) * phi_2
                        - 4200f64 * phi_0 * phi_1p.powi(2) * phi_2.powi(4)
                        + 840f64 * phi_0 * phi_1p.powi(2) * phi_2
                        + 3150f64 * phi_0 * phi_1p * phi_2.powi(4)
                        - 1260f64 * phi_0 * phi_1p * phi_2.powi(2)
                        - 5040f64 * phi_1n.powi(4) * phi_1p.powi(3) * phi_2
                        + 1680f64 * phi_1n.powi(4) * phi_1p.powi(3)
                        + 7560f64 * phi_1n.powi(4) * phi_1p.powi(2) * phi_2.powi(2)
                        - 1260f64 * phi_1n.powi(4) * phi_1p.powi(2)
                        - 5040f64 * phi_1n.powi(4) * phi_1p * phi_2.powi(2)
                        + 2520f64 * phi_1n.powi(4) * phi_1p * phi_2
                        - 5040f64 * phi_1n.powi(3) * phi_1p.powi(4) * phi_2
                        + 1680f64 * phi_1n.powi(3) * phi_1p.powi(4)
                        + 7560f64 * phi_1n.powi(3) * phi_1p.powi(3) * phi_2.powi(2)
                        - 1260f64 * phi_1n.powi(3) * phi_1p.powi(3)
                        - 5040f64 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2.powi(2)
                        + 2520f64 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2
                        + 7560f64 * phi_1n.powi(2) * phi_1p.powi(4) * phi_2.powi(2)
                        - 1260f64 * phi_1n.powi(2) * phi_1p.powi(4)
                        - 5040f64 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2.powi(2)
                        + 2520f64 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2
                        - 12600f64 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(4)
                        + 840f64 * phi_1n.powi(2) * phi_1p.powi(2)
                        + 8400f64 * phi_1n.powi(2) * phi_1p * phi_2.powi(4)
                        - 1680f64 * phi_1n.powi(2) * phi_1p * phi_2
                        - 5040f64 * phi_1n * phi_1p.powi(4) * phi_2.powi(2)
                        + 2520f64 * phi_1n * phi_1p.powi(4) * phi_2
                        + 8400f64 * phi_1n * phi_1p.powi(2) * phi_2.powi(4)
                        - 1680f64 * phi_1n * phi_1p.powi(2) * phi_2
                        - 6300f64 * phi_1n * phi_1p * phi_2.powi(4)
                        + 2520f64 * phi_1n * phi_1p * phi_2.powi(2))
                + phi.powi(3)
                    * (-1260f64 * phi_0.powi(4) * phi_1n.powi(3) * phi_1p.powi(2)
                        + 2520f64 * phi_0.powi(4) * phi_1n.powi(3) * phi_1p * phi_2
                        - 1260f64 * phi_0.powi(4) * phi_1n.powi(3) * phi_2
                        + 420f64 * phi_0.powi(4) * phi_1n.powi(3)
                        - 1260f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_1p.powi(3)
                        + 2520f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_1p.powi(2) * phi_2
                        - 1260f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_1p * phi_2
                        + 420f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_1p
                        + 2520f64 * phi_0.powi(4) * phi_1n * phi_1p.powi(3) * phi_2
                        - 1260f64 * phi_0.powi(4) * phi_1n * phi_1p.powi(2) * phi_2
                        + 420f64 * phi_0.powi(4) * phi_1n * phi_1p.powi(2)
                        - 5040f64 * phi_0.powi(4) * phi_1n * phi_1p * phi_2.powi(3)
                        + 2520f64 * phi_0.powi(4) * phi_1n * phi_2.powi(3)
                        - 252f64 * phi_0.powi(4) * phi_1n
                        - 1260f64 * phi_0.powi(4) * phi_1p.powi(3) * phi_2
                        + 420f64 * phi_0.powi(4) * phi_1p.powi(3)
                        + 2520f64 * phi_0.powi(4) * phi_1p * phi_2.powi(3)
                        - 252f64 * phi_0.powi(4) * phi_1p
                        - 1680f64 * phi_0.powi(4) * phi_2.powi(3)
                        + 504f64 * phi_0.powi(4) * phi_2
                        + 1512f64 * phi_0.powi(3) * phi_1n.powi(4) * phi_1p.powi(2)
                        - 3024f64 * phi_0.powi(3) * phi_1n.powi(4) * phi_1p * phi_2
                        + 1512f64 * phi_0.powi(3) * phi_1n.powi(4) * phi_2
                        - 504f64 * phi_0.powi(3) * phi_1n.powi(4)
                        + 1512f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_1p.powi(3)
                        - 3024f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_1p.powi(2) * phi_2
                        - 1260f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_1p.powi(2)
                        + 4032f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_1p * phi_2
                        - 504f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_1p
                        - 1260f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_2
                        + 420f64 * phi_0.powi(3) * phi_1n.powi(3)
                        + 1512f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p.powi(4)
                        - 3024f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p.powi(3) * phi_2
                        - 1260f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p.powi(3)
                        + 4032f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p.powi(2) * phi_2
                        - 504f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p.powi(2)
                        - 1260f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p * phi_2
                        + 420f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p
                        - 3024f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(4) * phi_2
                        + 4032f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(3) * phi_2
                        - 504f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(3)
                        - 1260f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(2) * phi_2
                        + 420f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(2)
                        + 7560f64 * phi_0.powi(3) * phi_1n * phi_1p * phi_2.powi(4)
                        - 5040f64 * phi_0.powi(3) * phi_1n * phi_1p * phi_2.powi(3)
                        - 3780f64 * phi_0.powi(3) * phi_1n * phi_2.powi(4)
                        + 2520f64 * phi_0.powi(3) * phi_1n * phi_2.powi(3)
                        + 1512f64 * phi_0.powi(3) * phi_1p.powi(4) * phi_2
                        - 504f64 * phi_0.powi(3) * phi_1p.powi(4)
                        - 1260f64 * phi_0.powi(3) * phi_1p.powi(3) * phi_2
                        + 420f64 * phi_0.powi(3) * phi_1p.powi(3)
                        - 3780f64 * phi_0.powi(3) * phi_1p * phi_2.powi(4)
                        + 2520f64 * phi_0.powi(3) * phi_1p * phi_2.powi(3)
                        + 2520f64 * phi_0.powi(3) * phi_2.powi(4)
                        - 1680f64 * phi_0.powi(3) * phi_2.powi(3)
                        + 1512f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_1p.powi(2)
                        - 3024f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_1p * phi_2
                        + 1512f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_2
                        - 504f64 * phi_0.powi(2) * phi_1n.powi(4)
                        + 1512f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(3)
                        - 3024f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(2) * phi_2
                        - 1260f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(2)
                        + 4032f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p * phi_2
                        - 504f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p
                        - 1260f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_2
                        + 420f64 * phi_0.powi(2) * phi_1n.powi(3)
                        + 1512f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(4)
                        - 3024f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(3) * phi_2
                        - 1260f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(3)
                        + 4032f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(2) * phi_2
                        - 504f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(2)
                        - 1260f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p * phi_2
                        + 420f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p
                        - 3024f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(4) * phi_2
                        + 4032f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(3) * phi_2
                        - 504f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(3)
                        - 1260f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(2) * phi_2
                        + 420f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(2)
                        + 7560f64 * phi_0.powi(2) * phi_1n * phi_1p * phi_2.powi(4)
                        - 5040f64 * phi_0.powi(2) * phi_1n * phi_1p * phi_2.powi(3)
                        - 3780f64 * phi_0.powi(2) * phi_1n * phi_2.powi(4)
                        + 2520f64 * phi_0.powi(2) * phi_1n * phi_2.powi(3)
                        + 1512f64 * phi_0.powi(2) * phi_1p.powi(4) * phi_2
                        - 504f64 * phi_0.powi(2) * phi_1p.powi(4)
                        - 1260f64 * phi_0.powi(2) * phi_1p.powi(3) * phi_2
                        + 420f64 * phi_0.powi(2) * phi_1p.powi(3)
                        - 3780f64 * phi_0.powi(2) * phi_1p * phi_2.powi(4)
                        + 2520f64 * phi_0.powi(2) * phi_1p * phi_2.powi(3)
                        + 2520f64 * phi_0.powi(2) * phi_2.powi(4)
                        - 1680f64 * phi_0.powi(2) * phi_2.powi(3)
                        - 2520f64 * phi_0 * phi_1n.powi(4) * phi_1p.powi(4)
                        + 1512f64 * phi_0 * phi_1n.powi(4) * phi_1p.powi(2)
                        + 10080f64 * phi_0 * phi_1n.powi(4) * phi_1p * phi_2.powi(3)
                        - 3024f64 * phi_0 * phi_1n.powi(4) * phi_1p * phi_2
                        - 5040f64 * phi_0 * phi_1n.powi(4) * phi_2.powi(3)
                        + 1512f64 * phi_0 * phi_1n.powi(4) * phi_2
                        + 1512f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(3)
                        + 10080f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2.powi(3)
                        - 3024f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2
                        - 1260f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(2)
                        - 12600f64 * phi_0 * phi_1n.powi(3) * phi_1p * phi_2.powi(4)
                        - 5040f64 * phi_0 * phi_1n.powi(3) * phi_1p * phi_2.powi(3)
                        + 4032f64 * phi_0 * phi_1n.powi(3) * phi_1p * phi_2
                        + 6300f64 * phi_0 * phi_1n.powi(3) * phi_2.powi(4)
                        - 1260f64 * phi_0 * phi_1n.powi(3) * phi_2
                        + 1512f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(4)
                        + 10080f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2.powi(3)
                        - 3024f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2
                        - 1260f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(3)
                        - 12600f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(4)
                        - 5040f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(3)
                        + 4032f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2
                        + 6300f64 * phi_0 * phi_1n.powi(2) * phi_1p * phi_2.powi(4)
                        - 1260f64 * phi_0 * phi_1n.powi(2) * phi_1p * phi_2
                        + 10080f64 * phi_0 * phi_1n * phi_1p.powi(4) * phi_2.powi(3)
                        - 3024f64 * phi_0 * phi_1n * phi_1p.powi(4) * phi_2
                        - 12600f64 * phi_0 * phi_1n * phi_1p.powi(3) * phi_2.powi(4)
                        - 5040f64 * phi_0 * phi_1n * phi_1p.powi(3) * phi_2.powi(3)
                        + 4032f64 * phi_0 * phi_1n * phi_1p.powi(3) * phi_2
                        + 6300f64 * phi_0 * phi_1n * phi_1p.powi(2) * phi_2.powi(4)
                        - 1260f64 * phi_0 * phi_1n * phi_1p.powi(2) * phi_2
                        + 7560f64 * phi_0 * phi_1n * phi_1p * phi_2.powi(4)
                        - 5040f64 * phi_0 * phi_1n * phi_1p * phi_2.powi(3)
                        - 3780f64 * phi_0 * phi_1n * phi_2.powi(4)
                        + 2520f64 * phi_0 * phi_1n * phi_2.powi(3)
                        - 5040f64 * phi_0 * phi_1p.powi(4) * phi_2.powi(3)
                        + 1512f64 * phi_0 * phi_1p.powi(4) * phi_2
                        + 6300f64 * phi_0 * phi_1p.powi(3) * phi_2.powi(4)
                        - 1260f64 * phi_0 * phi_1p.powi(3) * phi_2
                        - 3780f64 * phi_0 * phi_1p * phi_2.powi(4)
                        + 2520f64 * phi_0 * phi_1p * phi_2.powi(3)
                        + 7560f64 * phi_1n.powi(4) * phi_1p.powi(4) * phi_2
                        - 2520f64 * phi_1n.powi(4) * phi_1p.powi(4)
                        - 15120f64 * phi_1n.powi(4) * phi_1p.powi(2) * phi_2.powi(3)
                        + 1512f64 * phi_1n.powi(4) * phi_1p.powi(2)
                        + 10080f64 * phi_1n.powi(4) * phi_1p * phi_2.powi(3)
                        - 3024f64 * phi_1n.powi(4) * phi_1p * phi_2
                        - 15120f64 * phi_1n.powi(3) * phi_1p.powi(3) * phi_2.powi(3)
                        + 1512f64 * phi_1n.powi(3) * phi_1p.powi(3)
                        + 18900f64 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2.powi(4)
                        + 10080f64 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2.powi(3)
                        - 3024f64 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2
                        - 1260f64 * phi_1n.powi(3) * phi_1p.powi(2)
                        - 12600f64 * phi_1n.powi(3) * phi_1p * phi_2.powi(4)
                        + 2520f64 * phi_1n.powi(3) * phi_1p * phi_2
                        - 15120f64 * phi_1n.powi(2) * phi_1p.powi(4) * phi_2.powi(3)
                        + 1512f64 * phi_1n.powi(2) * phi_1p.powi(4)
                        + 18900f64 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2.powi(4)
                        + 10080f64 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2.powi(3)
                        - 3024f64 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2
                        - 1260f64 * phi_1n.powi(2) * phi_1p.powi(3)
                        - 12600f64 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(4)
                        + 2520f64 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2
                        + 10080f64 * phi_1n * phi_1p.powi(4) * phi_2.powi(3)
                        - 3024f64 * phi_1n * phi_1p.powi(4) * phi_2
                        - 12600f64 * phi_1n * phi_1p.powi(3) * phi_2.powi(4)
                        + 2520f64 * phi_1n * phi_1p.powi(3) * phi_2
                        + 7560f64 * phi_1n * phi_1p * phi_2.powi(4)
                        - 5040f64 * phi_1n * phi_1p * phi_2.powi(3))
                + phi.powi(2)
                    * (2100f64 * phi_0.powi(4) * phi_1n.powi(3) * phi_1p.powi(3)
                        - 6300f64 * phi_0.powi(4) * phi_1n.powi(3) * phi_1p * phi_2.powi(2)
                        + 3150f64 * phi_0.powi(4) * phi_1n.powi(3) * phi_2.powi(2)
                        - 525f64 * phi_0.powi(4) * phi_1n.powi(3)
                        - 6300f64
                            * phi_0.powi(4)
                            * phi_1n.powi(2)
                            * phi_1p.powi(2)
                            * phi_2.powi(2)
                        + 8400f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_1p * phi_2.powi(3)
                        + 3150f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_1p * phi_2.powi(2)
                        - 525f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_1p
                        - 4200f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_2.powi(3)
                        + 420f64 * phi_0.powi(4) * phi_1n.powi(2)
                        - 6300f64 * phi_0.powi(4) * phi_1n * phi_1p.powi(3) * phi_2.powi(2)
                        + 8400f64 * phi_0.powi(4) * phi_1n * phi_1p.powi(2) * phi_2.powi(3)
                        + 3150f64 * phi_0.powi(4) * phi_1n * phi_1p.powi(2) * phi_2.powi(2)
                        - 525f64 * phi_0.powi(4) * phi_1n * phi_1p.powi(2)
                        - 4200f64 * phi_0.powi(4) * phi_1n * phi_1p * phi_2.powi(3)
                        + 420f64 * phi_0.powi(4) * phi_1n * phi_1p
                        + 3150f64 * phi_0.powi(4) * phi_1p.powi(3) * phi_2.powi(2)
                        - 525f64 * phi_0.powi(4) * phi_1p.powi(3)
                        - 4200f64 * phi_0.powi(4) * phi_1p.powi(2) * phi_2.powi(3)
                        + 420f64 * phi_0.powi(4) * phi_1p.powi(2)
                        + 2100f64 * phi_0.powi(4) * phi_2.powi(3)
                        - 1260f64 * phi_0.powi(4) * phi_2.powi(2)
                        - 2520f64 * phi_0.powi(3) * phi_1n.powi(4) * phi_1p.powi(3)
                        + 7560f64 * phi_0.powi(3) * phi_1n.powi(4) * phi_1p * phi_2.powi(2)
                        - 3780f64 * phi_0.powi(3) * phi_1n.powi(4) * phi_2.powi(2)
                        + 630f64 * phi_0.powi(3) * phi_1n.powi(4)
                        - 2520f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_1p.powi(4)
                        + 2100f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_1p.powi(3)
                        + 7560f64
                            * phi_0.powi(3)
                            * phi_1n.powi(3)
                            * phi_1p.powi(2)
                            * phi_2.powi(2)
                        - 10080f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_1p * phi_2.powi(2)
                        + 630f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_1p
                        + 3150f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_2.powi(2)
                        - 525f64 * phi_0.powi(3) * phi_1n.powi(3)
                        + 7560f64
                            * phi_0.powi(3)
                            * phi_1n.powi(2)
                            * phi_1p.powi(3)
                            * phi_2.powi(2)
                        - 10080f64
                            * phi_0.powi(3)
                            * phi_1n.powi(2)
                            * phi_1p.powi(2)
                            * phi_2.powi(2)
                        + 630f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p.powi(2)
                        - 12600f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p * phi_2.powi(4)
                        + 8400f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p * phi_2.powi(3)
                        + 3150f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p * phi_2.powi(2)
                        - 525f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p
                        + 6300f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_2.powi(4)
                        - 4200f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_2.powi(3)
                        + 7560f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(4) * phi_2.powi(2)
                        - 10080f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(3) * phi_2.powi(2)
                        + 630f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(3)
                        - 12600f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(2) * phi_2.powi(4)
                        + 8400f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(2) * phi_2.powi(3)
                        + 3150f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(2) * phi_2.powi(2)
                        - 525f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(2)
                        + 6300f64 * phi_0.powi(3) * phi_1n * phi_1p * phi_2.powi(4)
                        - 4200f64 * phi_0.powi(3) * phi_1n * phi_1p * phi_2.powi(3)
                        - 3780f64 * phi_0.powi(3) * phi_1p.powi(4) * phi_2.powi(2)
                        + 630f64 * phi_0.powi(3) * phi_1p.powi(4)
                        + 3150f64 * phi_0.powi(3) * phi_1p.powi(3) * phi_2.powi(2)
                        - 525f64 * phi_0.powi(3) * phi_1p.powi(3)
                        + 6300f64 * phi_0.powi(3) * phi_1p.powi(2) * phi_2.powi(4)
                        - 4200f64 * phi_0.powi(3) * phi_1p.powi(2) * phi_2.powi(3)
                        - 3150f64 * phi_0.powi(3) * phi_2.powi(4)
                        + 2100f64 * phi_0.powi(3) * phi_2.powi(3)
                        + 3150f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_1p.powi(4)
                        - 2520f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_1p.powi(3)
                        - 12600f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_1p * phi_2.powi(3)
                        + 7560f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_1p * phi_2.powi(2)
                        + 6300f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_2.powi(3)
                        - 3780f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_2.powi(2)
                        - 2520f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(4)
                        + 2100f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(3)
                        - 12600f64
                            * phi_0.powi(2)
                            * phi_1n.powi(3)
                            * phi_1p.powi(2)
                            * phi_2.powi(3)
                        + 7560f64
                            * phi_0.powi(2)
                            * phi_1n.powi(3)
                            * phi_1p.powi(2)
                            * phi_2.powi(2)
                        + 15750f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p * phi_2.powi(4)
                        + 6300f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p * phi_2.powi(3)
                        - 10080f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p * phi_2.powi(2)
                        - 7875f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_2.powi(4)
                        + 3150f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_2.powi(2)
                        - 12600f64
                            * phi_0.powi(2)
                            * phi_1n.powi(2)
                            * phi_1p.powi(3)
                            * phi_2.powi(3)
                        + 7560f64
                            * phi_0.powi(2)
                            * phi_1n.powi(2)
                            * phi_1p.powi(3)
                            * phi_2.powi(2)
                        + 15750f64
                            * phi_0.powi(2)
                            * phi_1n.powi(2)
                            * phi_1p.powi(2)
                            * phi_2.powi(4)
                        + 6300f64
                            * phi_0.powi(2)
                            * phi_1n.powi(2)
                            * phi_1p.powi(2)
                            * phi_2.powi(3)
                        - 10080f64
                            * phi_0.powi(2)
                            * phi_1n.powi(2)
                            * phi_1p.powi(2)
                            * phi_2.powi(2)
                        - 20475f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p * phi_2.powi(4)
                        + 8400f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p * phi_2.powi(3)
                        + 3150f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p * phi_2.powi(2)
                        + 6300f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_2.powi(4)
                        - 4200f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_2.powi(3)
                        - 12600f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(4) * phi_2.powi(3)
                        + 7560f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(4) * phi_2.powi(2)
                        + 15750f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(3) * phi_2.powi(4)
                        + 6300f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(3) * phi_2.powi(3)
                        - 10080f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(3) * phi_2.powi(2)
                        - 20475f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(2) * phi_2.powi(4)
                        + 8400f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(2) * phi_2.powi(3)
                        + 3150f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(2) * phi_2.powi(2)
                        + 6300f64 * phi_0.powi(2) * phi_1n * phi_1p * phi_2.powi(4)
                        - 4200f64 * phi_0.powi(2) * phi_1n * phi_1p * phi_2.powi(3)
                        + 6300f64 * phi_0.powi(2) * phi_1p.powi(4) * phi_2.powi(3)
                        - 3780f64 * phi_0.powi(2) * phi_1p.powi(4) * phi_2.powi(2)
                        - 7875f64 * phi_0.powi(2) * phi_1p.powi(3) * phi_2.powi(4)
                        + 3150f64 * phi_0.powi(2) * phi_1p.powi(3) * phi_2.powi(2)
                        + 6300f64 * phi_0.powi(2) * phi_1p.powi(2) * phi_2.powi(4)
                        - 4200f64 * phi_0.powi(2) * phi_1p.powi(2) * phi_2.powi(3)
                        + 3150f64 * phi_0 * phi_1n.powi(4) * phi_1p.powi(4)
                        - 2520f64 * phi_0 * phi_1n.powi(4) * phi_1p.powi(3)
                        - 12600f64 * phi_0 * phi_1n.powi(4) * phi_1p * phi_2.powi(3)
                        + 7560f64 * phi_0 * phi_1n.powi(4) * phi_1p * phi_2.powi(2)
                        + 6300f64 * phi_0 * phi_1n.powi(4) * phi_2.powi(3)
                        - 3780f64 * phi_0 * phi_1n.powi(4) * phi_2.powi(2)
                        - 2520f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(4)
                        + 2100f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(3)
                        - 12600f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2.powi(3)
                        + 7560f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2.powi(2)
                        + 15750f64 * phi_0 * phi_1n.powi(3) * phi_1p * phi_2.powi(4)
                        + 6300f64 * phi_0 * phi_1n.powi(3) * phi_1p * phi_2.powi(3)
                        - 10080f64 * phi_0 * phi_1n.powi(3) * phi_1p * phi_2.powi(2)
                        - 7875f64 * phi_0 * phi_1n.powi(3) * phi_2.powi(4)
                        + 3150f64 * phi_0 * phi_1n.powi(3) * phi_2.powi(2)
                        - 12600f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2.powi(3)
                        + 7560f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2.powi(2)
                        + 15750f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(4)
                        + 6300f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(3)
                        - 10080f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(2)
                        - 20475f64 * phi_0 * phi_1n.powi(2) * phi_1p * phi_2.powi(4)
                        + 8400f64 * phi_0 * phi_1n.powi(2) * phi_1p * phi_2.powi(3)
                        + 3150f64 * phi_0 * phi_1n.powi(2) * phi_1p * phi_2.powi(2)
                        + 6300f64 * phi_0 * phi_1n.powi(2) * phi_2.powi(4)
                        - 4200f64 * phi_0 * phi_1n.powi(2) * phi_2.powi(3)
                        - 12600f64 * phi_0 * phi_1n * phi_1p.powi(4) * phi_2.powi(3)
                        + 7560f64 * phi_0 * phi_1n * phi_1p.powi(4) * phi_2.powi(2)
                        + 15750f64 * phi_0 * phi_1n * phi_1p.powi(3) * phi_2.powi(4)
                        + 6300f64 * phi_0 * phi_1n * phi_1p.powi(3) * phi_2.powi(3)
                        - 10080f64 * phi_0 * phi_1n * phi_1p.powi(3) * phi_2.powi(2)
                        - 20475f64 * phi_0 * phi_1n * phi_1p.powi(2) * phi_2.powi(4)
                        + 8400f64 * phi_0 * phi_1n * phi_1p.powi(2) * phi_2.powi(3)
                        + 3150f64 * phi_0 * phi_1n * phi_1p.powi(2) * phi_2.powi(2)
                        + 6300f64 * phi_0 * phi_1n * phi_1p * phi_2.powi(4)
                        - 4200f64 * phi_0 * phi_1n * phi_1p * phi_2.powi(3)
                        + 6300f64 * phi_0 * phi_1p.powi(4) * phi_2.powi(3)
                        - 3780f64 * phi_0 * phi_1p.powi(4) * phi_2.powi(2)
                        - 7875f64 * phi_0 * phi_1p.powi(3) * phi_2.powi(4)
                        + 3150f64 * phi_0 * phi_1p.powi(3) * phi_2.powi(2)
                        + 6300f64 * phi_0 * phi_1p.powi(2) * phi_2.powi(4)
                        - 4200f64 * phi_0 * phi_1p.powi(2) * phi_2.powi(3)
                        - 18900f64 * phi_1n.powi(4) * phi_1p.powi(4) * phi_2.powi(2)
                        + 3150f64 * phi_1n.powi(4) * phi_1p.powi(4)
                        + 25200f64 * phi_1n.powi(4) * phi_1p.powi(3) * phi_2.powi(3)
                        - 2520f64 * phi_1n.powi(4) * phi_1p.powi(3)
                        - 12600f64 * phi_1n.powi(4) * phi_1p * phi_2.powi(3)
                        + 7560f64 * phi_1n.powi(4) * phi_1p * phi_2.powi(2)
                        + 25200f64 * phi_1n.powi(3) * phi_1p.powi(4) * phi_2.powi(3)
                        - 2520f64 * phi_1n.powi(3) * phi_1p.powi(4)
                        - 31500f64 * phi_1n.powi(3) * phi_1p.powi(3) * phi_2.powi(4)
                        + 2100f64 * phi_1n.powi(3) * phi_1p.powi(3)
                        - 12600f64 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2.powi(3)
                        + 7560f64 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2.powi(2)
                        + 15750f64 * phi_1n.powi(3) * phi_1p * phi_2.powi(4)
                        - 6300f64 * phi_1n.powi(3) * phi_1p * phi_2.powi(2)
                        - 12600f64 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2.powi(3)
                        + 7560f64 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2.powi(2)
                        + 15750f64 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(4)
                        - 6300f64 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(2)
                        - 12600f64 * phi_1n.powi(2) * phi_1p * phi_2.powi(4)
                        + 8400f64 * phi_1n.powi(2) * phi_1p * phi_2.powi(3)
                        - 12600f64 * phi_1n * phi_1p.powi(4) * phi_2.powi(3)
                        + 7560f64 * phi_1n * phi_1p.powi(4) * phi_2.powi(2)
                        + 15750f64 * phi_1n * phi_1p.powi(3) * phi_2.powi(4)
                        - 6300f64 * phi_1n * phi_1p.powi(3) * phi_2.powi(2)
                        - 12600f64 * phi_1n * phi_1p.powi(2) * phi_2.powi(4)
                        + 8400f64 * phi_1n * phi_1p.powi(2) * phi_2.powi(3))
                + 140f64
                    * phi
                    * phi_2
                    * (-60f64 * phi_0.powi(4) * phi_1n.powi(3) * phi_1p.powi(3)
                        + 90f64 * phi_0.powi(4) * phi_1n.powi(3) * phi_1p.powi(2) * phi_2
                        - 30f64 * phi_0.powi(4) * phi_1n.powi(3) * phi_2
                        + 15f64 * phi_0.powi(4) * phi_1n.powi(3)
                        + 90f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_1p.powi(3) * phi_2
                        - 120f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(2)
                        - 30f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_1p * phi_2
                        + 15f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_1p
                        + 40f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_2.powi(2)
                        - 12f64 * phi_0.powi(4) * phi_1n.powi(2)
                        - 30f64 * phi_0.powi(4) * phi_1n * phi_1p.powi(2) * phi_2
                        + 15f64 * phi_0.powi(4) * phi_1n * phi_1p.powi(2)
                        + 40f64 * phi_0.powi(4) * phi_1n * phi_1p * phi_2.powi(2)
                        - 12f64 * phi_0.powi(4) * phi_1n * phi_1p
                        - 30f64 * phi_0.powi(4) * phi_1n * phi_2.powi(2)
                        + 18f64 * phi_0.powi(4) * phi_1n * phi_2
                        - 30f64 * phi_0.powi(4) * phi_1p.powi(3) * phi_2
                        + 15f64 * phi_0.powi(4) * phi_1p.powi(3)
                        + 40f64 * phi_0.powi(4) * phi_1p.powi(2) * phi_2.powi(2)
                        - 12f64 * phi_0.powi(4) * phi_1p.powi(2)
                        - 30f64 * phi_0.powi(4) * phi_1p * phi_2.powi(2)
                        + 18f64 * phi_0.powi(4) * phi_1p * phi_2
                        + 72f64 * phi_0.powi(3) * phi_1n.powi(4) * phi_1p.powi(3)
                        - 108f64 * phi_0.powi(3) * phi_1n.powi(4) * phi_1p.powi(2) * phi_2
                        + 36f64 * phi_0.powi(3) * phi_1n.powi(4) * phi_2
                        - 18f64 * phi_0.powi(3) * phi_1n.powi(4)
                        + 72f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_1p.powi(4)
                        - 108f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_1p.powi(3) * phi_2
                        - 60f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_1p.powi(3)
                        + 90f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_1p.powi(2) * phi_2
                        + 36f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_1p * phi_2
                        - 18f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_1p
                        - 30f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_2
                        + 15f64 * phi_0.powi(3) * phi_1n.powi(3)
                        - 108f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p.powi(4) * phi_2
                        + 90f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p.powi(3) * phi_2
                        + 180f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(3)
                        - 120f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(2)
                        + 36f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p.powi(2) * phi_2
                        - 18f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p.powi(2)
                        - 30f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p * phi_2
                        + 15f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p
                        - 60f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_2.powi(3)
                        + 40f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_2.powi(2)
                        + 36f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(3) * phi_2
                        - 18f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(3)
                        - 30f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(2) * phi_2
                        + 15f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(2)
                        - 60f64 * phi_0.powi(3) * phi_1n * phi_1p * phi_2.powi(3)
                        + 40f64 * phi_0.powi(3) * phi_1n * phi_1p * phi_2.powi(2)
                        + 45f64 * phi_0.powi(3) * phi_1n * phi_2.powi(3)
                        - 30f64 * phi_0.powi(3) * phi_1n * phi_2.powi(2)
                        + 36f64 * phi_0.powi(3) * phi_1p.powi(4) * phi_2
                        - 18f64 * phi_0.powi(3) * phi_1p.powi(4)
                        - 30f64 * phi_0.powi(3) * phi_1p.powi(3) * phi_2
                        + 15f64 * phi_0.powi(3) * phi_1p.powi(3)
                        - 60f64 * phi_0.powi(3) * phi_1p.powi(2) * phi_2.powi(3)
                        + 40f64 * phi_0.powi(3) * phi_1p.powi(2) * phi_2.powi(2)
                        + 45f64 * phi_0.powi(3) * phi_1p * phi_2.powi(3)
                        - 30f64 * phi_0.powi(3) * phi_1p * phi_2.powi(2)
                        - 90f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_1p.powi(4)
                        + 72f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_1p.powi(3)
                        + 180f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_1p.powi(2) * phi_2.powi(2)
                        - 108f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_1p.powi(2) * phi_2
                        - 60f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_2.powi(2)
                        + 36f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_2
                        + 72f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(4)
                        + 180f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(3) * phi_2.powi(2)
                        - 108f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(3) * phi_2
                        - 60f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(3)
                        - 225f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(2) * phi_2.powi(3)
                        + 90f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(2) * phi_2
                        - 60f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p * phi_2.powi(2)
                        + 36f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p * phi_2
                        + 75f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_2.powi(3)
                        - 30f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_2
                        + 180f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(4) * phi_2.powi(2)
                        - 108f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(4) * phi_2
                        - 225f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(3) * phi_2.powi(3)
                        + 90f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(3) * phi_2
                        + 180f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(3)
                        - 180f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(2)
                        + 36f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(2) * phi_2
                        + 75f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p * phi_2.powi(3)
                        - 30f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p * phi_2
                        - 60f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_2.powi(3)
                        + 40f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_2.powi(2)
                        - 60f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(3) * phi_2.powi(2)
                        + 36f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(3) * phi_2
                        + 75f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(2) * phi_2.powi(3)
                        - 30f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(2) * phi_2
                        - 60f64 * phi_0.powi(2) * phi_1n * phi_1p * phi_2.powi(3)
                        + 40f64 * phi_0.powi(2) * phi_1n * phi_1p * phi_2.powi(2)
                        - 60f64 * phi_0.powi(2) * phi_1p.powi(4) * phi_2.powi(2)
                        + 36f64 * phi_0.powi(2) * phi_1p.powi(4) * phi_2
                        + 75f64 * phi_0.powi(2) * phi_1p.powi(3) * phi_2.powi(3)
                        - 30f64 * phi_0.powi(2) * phi_1p.powi(3) * phi_2
                        - 60f64 * phi_0.powi(2) * phi_1p.powi(2) * phi_2.powi(3)
                        + 40f64 * phi_0.powi(2) * phi_1p.powi(2) * phi_2.powi(2)
                        + 180f64 * phi_0 * phi_1n.powi(4) * phi_1p.powi(4) * phi_2
                        - 90f64 * phi_0 * phi_1n.powi(4) * phi_1p.powi(4)
                        - 240f64 * phi_0 * phi_1n.powi(4) * phi_1p.powi(3) * phi_2.powi(2)
                        + 72f64 * phi_0 * phi_1n.powi(4) * phi_1p.powi(3)
                        + 180f64 * phi_0 * phi_1n.powi(4) * phi_1p.powi(2) * phi_2.powi(2)
                        - 108f64 * phi_0 * phi_1n.powi(4) * phi_1p.powi(2) * phi_2
                        - 240f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(4) * phi_2.powi(2)
                        + 72f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(4)
                        + 300f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(3) * phi_2.powi(3)
                        + 180f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(3) * phi_2.powi(2)
                        - 108f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(3) * phi_2
                        - 60f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(3)
                        - 225f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2.powi(3)
                        + 90f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2
                        + 180f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(4) * phi_2.powi(2)
                        - 108f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(4) * phi_2
                        - 225f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2.powi(3)
                        + 90f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2
                        + 180f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(3)
                        - 120f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(2)
                        + 180f64 * phi_1n.powi(4) * phi_1p.powi(4) * phi_2
                        - 90f64 * phi_1n.powi(4) * phi_1p.powi(4)
                        - 240f64 * phi_1n.powi(4) * phi_1p.powi(3) * phi_2.powi(2)
                        + 72f64 * phi_1n.powi(4) * phi_1p.powi(3)
                        + 180f64 * phi_1n.powi(4) * phi_1p.powi(2) * phi_2.powi(2)
                        - 108f64 * phi_1n.powi(4) * phi_1p.powi(2) * phi_2
                        - 240f64 * phi_1n.powi(3) * phi_1p.powi(4) * phi_2.powi(2)
                        + 72f64 * phi_1n.powi(3) * phi_1p.powi(4)
                        + 300f64 * phi_1n.powi(3) * phi_1p.powi(3) * phi_2.powi(3)
                        + 180f64 * phi_1n.powi(3) * phi_1p.powi(3) * phi_2.powi(2)
                        - 108f64 * phi_1n.powi(3) * phi_1p.powi(3) * phi_2
                        - 60f64 * phi_1n.powi(3) * phi_1p.powi(3)
                        - 225f64 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2.powi(3)
                        + 90f64 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2
                        + 180f64 * phi_1n.powi(2) * phi_1p.powi(4) * phi_2.powi(2)
                        - 108f64 * phi_1n.powi(2) * phi_1p.powi(4) * phi_2
                        - 225f64 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2.powi(3)
                        + 90f64 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2
                        + 180f64 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(3)
                        - 120f64 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(2))
                + 210f64
                    * phi_0
                    * phi_1n
                    * phi_1p
                    * (60f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p.powi(2) * phi_2
                        - 20f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p.powi(2)
                        - 90f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p * phi_2.powi(2)
                        + 15f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p
                        + 60f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_2.powi(2)
                        - 30f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_2
                        - 90f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(2) * phi_2.powi(2)
                        + 15f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(2)
                        + 120f64 * phi_0.powi(3) * phi_1n * phi_1p * phi_2.powi(3)
                        + 60f64 * phi_0.powi(3) * phi_1n * phi_1p * phi_2.powi(2)
                        - 30f64 * phi_0.powi(3) * phi_1n * phi_1p * phi_2
                        - 12f64 * phi_0.powi(3) * phi_1n * phi_1p
                        - 80f64 * phi_0.powi(3) * phi_1n * phi_2.powi(3)
                        + 24f64 * phi_0.powi(3) * phi_1n * phi_2
                        + 60f64 * phi_0.powi(3) * phi_1p.powi(2) * phi_2.powi(2)
                        - 30f64 * phi_0.powi(3) * phi_1p.powi(2) * phi_2
                        - 80f64 * phi_0.powi(3) * phi_1p * phi_2.powi(3)
                        + 24f64 * phi_0.powi(3) * phi_1p * phi_2
                        + 60f64 * phi_0.powi(3) * phi_2.powi(3)
                        - 36f64 * phi_0.powi(3) * phi_2.powi(2)
                        - 72f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(2) * phi_2
                        + 24f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(2)
                        + 108f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p * phi_2.powi(2)
                        - 18f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p
                        - 72f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_2.powi(2)
                        + 36f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_2
                        - 72f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(3) * phi_2
                        + 24f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(3)
                        + 108f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(2)
                        + 60f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(2) * phi_2
                        - 38f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(2)
                        - 162f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p * phi_2.powi(2)
                        + 36f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p * phi_2
                        + 15f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p
                        + 60f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_2.powi(2)
                        - 30f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_2
                        + 108f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(3) * phi_2.powi(2)
                        - 18f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(3)
                        - 162f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(2) * phi_2.powi(2)
                        + 36f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(2) * phi_2
                        + 15f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(2)
                        - 180f64 * phi_0.powi(2) * phi_1n * phi_1p * phi_2.powi(4)
                        + 120f64 * phi_0.powi(2) * phi_1n * phi_1p * phi_2.powi(3)
                        + 60f64 * phi_0.powi(2) * phi_1n * phi_1p * phi_2.powi(2)
                        - 30f64 * phi_0.powi(2) * phi_1n * phi_1p * phi_2
                        + 120f64 * phi_0.powi(2) * phi_1n * phi_2.powi(4)
                        - 80f64 * phi_0.powi(2) * phi_1n * phi_2.powi(3)
                        - 72f64 * phi_0.powi(2) * phi_1p.powi(3) * phi_2.powi(2)
                        + 36f64 * phi_0.powi(2) * phi_1p.powi(3) * phi_2
                        + 60f64 * phi_0.powi(2) * phi_1p.powi(2) * phi_2.powi(2)
                        - 30f64 * phi_0.powi(2) * phi_1p.powi(2) * phi_2
                        + 120f64 * phi_0.powi(2) * phi_1p * phi_2.powi(4)
                        - 80f64 * phi_0.powi(2) * phi_1p * phi_2.powi(3)
                        - 90f64 * phi_0.powi(2) * phi_2.powi(4)
                        + 60f64 * phi_0.powi(2) * phi_2.powi(3)
                        + 90f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(3) * phi_2
                        - 30f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(3)
                        - 72f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2
                        + 24f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(2)
                        - 180f64 * phi_0 * phi_1n.powi(3) * phi_1p * phi_2.powi(3)
                        + 108f64 * phi_0 * phi_1n.powi(3) * phi_1p * phi_2.powi(2)
                        + 120f64 * phi_0 * phi_1n.powi(3) * phi_2.powi(3)
                        - 72f64 * phi_0 * phi_1n.powi(3) * phi_2.powi(2)
                        - 72f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2
                        + 24f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(3)
                        - 180f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(3)
                        + 108f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(2)
                        + 60f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2
                        - 20f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(2)
                        + 225f64 * phi_0 * phi_1n.powi(2) * phi_1p * phi_2.powi(4)
                        + 120f64 * phi_0 * phi_1n.powi(2) * phi_1p * phi_2.powi(3)
                        - 162f64 * phi_0 * phi_1n.powi(2) * phi_1p * phi_2.powi(2)
                        - 150f64 * phi_0 * phi_1n.powi(2) * phi_2.powi(4)
                        + 60f64 * phi_0 * phi_1n.powi(2) * phi_2.powi(2)
                        - 180f64 * phi_0 * phi_1n * phi_1p.powi(3) * phi_2.powi(3)
                        + 108f64 * phi_0 * phi_1n * phi_1p.powi(3) * phi_2.powi(2)
                        + 225f64 * phi_0 * phi_1n * phi_1p.powi(2) * phi_2.powi(4)
                        + 120f64 * phi_0 * phi_1n * phi_1p.powi(2) * phi_2.powi(3)
                        - 162f64 * phi_0 * phi_1n * phi_1p.powi(2) * phi_2.powi(2)
                        - 330f64 * phi_0 * phi_1n * phi_1p * phi_2.powi(4)
                        + 120f64 * phi_0 * phi_1n * phi_1p * phi_2.powi(3)
                        + 60f64 * phi_0 * phi_1n * phi_1p * phi_2.powi(2)
                        + 120f64 * phi_0 * phi_1n * phi_2.powi(4)
                        - 80f64 * phi_0 * phi_1n * phi_2.powi(3)
                        + 120f64 * phi_0 * phi_1p.powi(3) * phi_2.powi(3)
                        - 72f64 * phi_0 * phi_1p.powi(3) * phi_2.powi(2)
                        - 150f64 * phi_0 * phi_1p.powi(2) * phi_2.powi(4)
                        + 60f64 * phi_0 * phi_1p.powi(2) * phi_2.powi(2)
                        + 120f64 * phi_0 * phi_1p * phi_2.powi(4)
                        - 80f64 * phi_0 * phi_1p * phi_2.powi(3)
                        - 180f64 * phi_1n.powi(3) * phi_1p.powi(3) * phi_2.powi(2)
                        + 90f64 * phi_1n.powi(3) * phi_1p.powi(3) * phi_2
                        + 240f64 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2.powi(3)
                        - 72f64 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2
                        - 180f64 * phi_1n.powi(3) * phi_1p * phi_2.powi(3)
                        + 108f64 * phi_1n.powi(3) * phi_1p * phi_2.powi(2)
                        + 240f64 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2.powi(3)
                        - 72f64 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2
                        - 300f64 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(4)
                        - 180f64 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(3)
                        + 108f64 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(2)
                        + 60f64 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2
                        + 225f64 * phi_1n.powi(2) * phi_1p * phi_2.powi(4)
                        - 90f64 * phi_1n.powi(2) * phi_1p * phi_2.powi(2)
                        - 180f64 * phi_1n * phi_1p.powi(3) * phi_2.powi(3)
                        + 108f64 * phi_1n * phi_1p.powi(3) * phi_2.powi(2)
                        + 225f64 * phi_1n * phi_1p.powi(2) * phi_2.powi(4)
                        - 90f64 * phi_1n * phi_1p.powi(2) * phi_2.powi(2)
                        - 180f64 * phi_1n * phi_1p * phi_2.powi(4)
                        + 120f64 * phi_1n * phi_1p * phi_2.powi(3)))
            / (-4200f64 * phi_0.powi(4) * phi_1n.powi(3) * phi_1p.powi(3) * phi_2
                + 2100f64 * phi_0.powi(4) * phi_1n.powi(3) * phi_1p.powi(3)
                + 6300f64 * phi_0.powi(4) * phi_1n.powi(3) * phi_1p.powi(2) * phi_2.powi(2)
                - 1890f64 * phi_0.powi(4) * phi_1n.powi(3) * phi_1p.powi(2)
                - 6300f64 * phi_0.powi(4) * phi_1n.powi(3) * phi_1p * phi_2.powi(2)
                + 3780f64 * phi_0.powi(4) * phi_1n.powi(3) * phi_1p * phi_2
                + 1050f64 * phi_0.powi(4) * phi_1n.powi(3) * phi_2.powi(2)
                - 840f64 * phi_0.powi(4) * phi_1n.powi(3) * phi_2
                + 105f64 * phi_0.powi(4) * phi_1n.powi(3)
                + 6300f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_1p.powi(3) * phi_2.powi(2)
                - 1890f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_1p.powi(3)
                - 8400f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(3)
                - 6300f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(2)
                + 3780f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_1p.powi(2) * phi_2
                + 1680f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_1p.powi(2)
                + 8400f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_1p * phi_2.powi(3)
                + 1050f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_1p * phi_2.powi(2)
                - 4200f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_1p * phi_2
                + 105f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_1p
                - 1400f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_2.powi(3)
                + 840f64 * phi_0.powi(4) * phi_1n.powi(2) * phi_2
                - 140f64 * phi_0.powi(4) * phi_1n.powi(2)
                - 6300f64 * phi_0.powi(4) * phi_1n * phi_1p.powi(3) * phi_2.powi(2)
                + 3780f64 * phi_0.powi(4) * phi_1n * phi_1p.powi(3) * phi_2
                + 8400f64 * phi_0.powi(4) * phi_1n * phi_1p.powi(2) * phi_2.powi(3)
                + 1050f64 * phi_0.powi(4) * phi_1n * phi_1p.powi(2) * phi_2.powi(2)
                - 4200f64 * phi_0.powi(4) * phi_1n * phi_1p.powi(2) * phi_2
                + 105f64 * phi_0.powi(4) * phi_1n * phi_1p.powi(2)
                - 8960f64 * phi_0.powi(4) * phi_1n * phi_1p * phi_2.powi(3)
                + 5040f64 * phi_0.powi(4) * phi_1n * phi_1p * phi_2.powi(2)
                + 840f64 * phi_0.powi(4) * phi_1n * phi_1p * phi_2
                - 140f64 * phi_0.powi(4) * phi_1n * phi_1p
                + 1680f64 * phi_0.powi(4) * phi_1n * phi_2.powi(3)
                - 1260f64 * phi_0.powi(4) * phi_1n * phi_2.powi(2)
                + 42f64 * phi_0.powi(4) * phi_1n
                + 1050f64 * phi_0.powi(4) * phi_1p.powi(3) * phi_2.powi(2)
                - 840f64 * phi_0.powi(4) * phi_1p.powi(3) * phi_2
                + 105f64 * phi_0.powi(4) * phi_1p.powi(3)
                - 1400f64 * phi_0.powi(4) * phi_1p.powi(2) * phi_2.powi(3)
                + 840f64 * phi_0.powi(4) * phi_1p.powi(2) * phi_2
                - 140f64 * phi_0.powi(4) * phi_1p.powi(2)
                + 1680f64 * phi_0.powi(4) * phi_1p * phi_2.powi(3)
                - 1260f64 * phi_0.powi(4) * phi_1p * phi_2.powi(2)
                + 42f64 * phi_0.powi(4) * phi_1p
                - 420f64 * phi_0.powi(4) * phi_2.powi(3)
                + 420f64 * phi_0.powi(4) * phi_2.powi(2)
                - 84f64 * phi_0.powi(4) * phi_2
                + 5040f64 * phi_0.powi(3) * phi_1n.powi(4) * phi_1p.powi(3) * phi_2
                - 2520f64 * phi_0.powi(3) * phi_1n.powi(4) * phi_1p.powi(3)
                - 7560f64 * phi_0.powi(3) * phi_1n.powi(4) * phi_1p.powi(2) * phi_2.powi(2)
                + 2268f64 * phi_0.powi(3) * phi_1n.powi(4) * phi_1p.powi(2)
                + 7560f64 * phi_0.powi(3) * phi_1n.powi(4) * phi_1p * phi_2.powi(2)
                - 4536f64 * phi_0.powi(3) * phi_1n.powi(4) * phi_1p * phi_2
                - 1260f64 * phi_0.powi(3) * phi_1n.powi(4) * phi_2.powi(2)
                + 1008f64 * phi_0.powi(3) * phi_1n.powi(4) * phi_2
                - 126f64 * phi_0.powi(3) * phi_1n.powi(4)
                + 5040f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_1p.powi(4) * phi_2
                - 2520f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_1p.powi(4)
                - 7560f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_1p.powi(3) * phi_2.powi(2)
                - 4200f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_1p.powi(3) * phi_2
                + 4368f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_1p.powi(3)
                + 13860f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_1p.powi(2) * phi_2.powi(2)
                - 4536f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_1p.powi(2) * phi_2
                - 1890f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_1p.powi(2)
                - 7560f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_1p * phi_2.powi(2)
                + 4788f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_1p * phi_2
                - 126f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_1p
                + 1050f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_2.powi(2)
                - 840f64 * phi_0.powi(3) * phi_1n.powi(3) * phi_2
                + 105f64 * phi_0.powi(3) * phi_1n.powi(3)
                - 7560f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p.powi(4) * phi_2.powi(2)
                + 2268f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p.powi(4)
                + 13860f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p.powi(3) * phi_2.powi(2)
                - 4536f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p.powi(3) * phi_2
                - 1890f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p.powi(3)
                + 12600f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(4)
                - 8400f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(3)
                - 7560f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(2)
                + 4788f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p.powi(2) * phi_2
                - 246f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p.powi(2)
                - 12600f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p * phi_2.powi(4)
                + 8400f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p * phi_2.powi(3)
                + 1050f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p * phi_2.powi(2)
                - 600f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p * phi_2
                + 105f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_1p
                + 2100f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_2.powi(4)
                - 1400f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_2.powi(3)
                - 120f64 * phi_0.powi(3) * phi_1n.powi(2) * phi_2
                + 40f64 * phi_0.powi(3) * phi_1n.powi(2)
                + 7560f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(4) * phi_2.powi(2)
                - 4536f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(4) * phi_2
                - 7560f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(3) * phi_2.powi(2)
                + 4788f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(3) * phi_2
                - 126f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(3)
                - 12600f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(2) * phi_2.powi(4)
                + 8400f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(2) * phi_2.powi(3)
                + 1050f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(2) * phi_2.powi(2)
                - 600f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(2) * phi_2
                + 105f64 * phi_0.powi(3) * phi_1n * phi_1p.powi(2)
                + 13440f64 * phi_0.powi(3) * phi_1n * phi_1p * phi_2.powi(4)
                - 8960f64 * phi_0.powi(3) * phi_1n * phi_1p * phi_2.powi(3)
                - 360f64 * phi_0.powi(3) * phi_1n * phi_1p * phi_2.powi(2)
                - 120f64 * phi_0.powi(3) * phi_1n * phi_1p * phi_2
                + 40f64 * phi_0.powi(3) * phi_1n * phi_1p
                - 2520f64 * phi_0.powi(3) * phi_1n * phi_2.powi(4)
                + 1680f64 * phi_0.powi(3) * phi_1n * phi_2.powi(3)
                + 180f64 * phi_0.powi(3) * phi_1n * phi_2.powi(2)
                - 30f64 * phi_0.powi(3) * phi_1n
                - 1260f64 * phi_0.powi(3) * phi_1p.powi(4) * phi_2.powi(2)
                + 1008f64 * phi_0.powi(3) * phi_1p.powi(4) * phi_2
                - 126f64 * phi_0.powi(3) * phi_1p.powi(4)
                + 1050f64 * phi_0.powi(3) * phi_1p.powi(3) * phi_2.powi(2)
                - 840f64 * phi_0.powi(3) * phi_1p.powi(3) * phi_2
                + 105f64 * phi_0.powi(3) * phi_1p.powi(3)
                + 2100f64 * phi_0.powi(3) * phi_1p.powi(2) * phi_2.powi(4)
                - 1400f64 * phi_0.powi(3) * phi_1p.powi(2) * phi_2.powi(3)
                - 120f64 * phi_0.powi(3) * phi_1p.powi(2) * phi_2
                + 40f64 * phi_0.powi(3) * phi_1p.powi(2)
                - 2520f64 * phi_0.powi(3) * phi_1p * phi_2.powi(4)
                + 1680f64 * phi_0.powi(3) * phi_1p * phi_2.powi(3)
                + 180f64 * phi_0.powi(3) * phi_1p * phi_2.powi(2)
                - 30f64 * phi_0.powi(3) * phi_1p
                + 630f64 * phi_0.powi(3) * phi_2.powi(4)
                - 420f64 * phi_0.powi(3) * phi_2.powi(3)
                - 120f64 * phi_0.powi(3) * phi_2.powi(2)
                + 60f64 * phi_0.powi(3) * phi_2
                - 6300f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_1p.powi(4) * phi_2
                + 3150f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_1p.powi(4)
                + 5040f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_1p.powi(3) * phi_2
                - 2520f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_1p.powi(3)
                + 12600f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_1p.powi(2) * phi_2.powi(3)
                - 7560f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_1p.powi(2) * phi_2.powi(2)
                - 252f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_1p.powi(2)
                - 12600f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_1p * phi_2.powi(3)
                + 7560f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_1p * phi_2.powi(2)
                + 504f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_1p * phi_2
                + 2100f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_2.powi(3)
                - 1260f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_2.powi(2)
                - 252f64 * phi_0.powi(2) * phi_1n.powi(4) * phi_2
                + 84f64 * phi_0.powi(2) * phi_1n.powi(4)
                + 5040f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(4) * phi_2
                - 2520f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(4)
                + 12600f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(3) * phi_2.powi(3)
                - 7560f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(3) * phi_2.powi(2)
                - 4200f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(3) * phi_2
                + 1848f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(3)
                - 15750f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(2) * phi_2.powi(4)
                - 12600f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(2) * phi_2.powi(3)
                + 13860f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(2) * phi_2.powi(2)
                + 504f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(2) * phi_2
                + 360f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p.powi(2)
                + 15750f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p * phi_2.powi(4)
                + 2100f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p * phi_2.powi(3)
                - 7560f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p * phi_2.powi(2)
                - 972f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p * phi_2
                + 84f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_1p
                - 2625f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_2.powi(4)
                + 1050f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_2.powi(2)
                + 360f64 * phi_0.powi(2) * phi_1n.powi(3) * phi_2
                - 120f64 * phi_0.powi(2) * phi_1n.powi(3)
                + 12600f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(4) * phi_2.powi(3)
                - 7560f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(4) * phi_2.powi(2)
                - 252f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(4)
                - 15750f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(3) * phi_2.powi(4)
                - 12600f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(3) * phi_2.powi(3)
                + 13860f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(3) * phi_2.powi(2)
                + 504f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(3) * phi_2
                + 360f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(3)
                + 28350f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(4)
                - 6300f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(3)
                - 7560f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(2)
                - 972f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(2) * phi_2
                - 36f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p.powi(2)
                - 15225f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p * phi_2.powi(4)
                + 8400f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p * phi_2.powi(3)
                + 1050f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p * phi_2.powi(2)
                + 600f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p * phi_2
                - 120f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_1p
                + 2100f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_2.powi(4)
                - 1400f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_2.powi(3)
                - 120f64 * phi_0.powi(2) * phi_1n.powi(2) * phi_2
                + 40f64 * phi_0.powi(2) * phi_1n.powi(2)
                - 12600f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(4) * phi_2.powi(3)
                + 7560f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(4) * phi_2.powi(2)
                + 504f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(4) * phi_2
                + 15750f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(3) * phi_2.powi(4)
                + 2100f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(3) * phi_2.powi(3)
                - 7560f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(3) * phi_2.powi(2)
                - 972f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(3) * phi_2
                + 84f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(3)
                - 15225f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(2) * phi_2.powi(4)
                + 8400f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(2) * phi_2.powi(3)
                + 1050f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(2) * phi_2.powi(2)
                + 600f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(2) * phi_2
                - 120f64 * phi_0.powi(2) * phi_1n * phi_1p.powi(2)
                + 840f64 * phi_0.powi(2) * phi_1n * phi_1p * phi_2.powi(4)
                + 40f64 * phi_0.powi(2) * phi_1n * phi_1p * phi_2.powi(3)
                - 360f64 * phi_0.powi(2) * phi_1n * phi_1p * phi_2.powi(2)
                - 120f64 * phi_0.powi(2) * phi_1n * phi_1p * phi_2
                + 40f64 * phi_0.powi(2) * phi_1n * phi_1p
                + 630f64 * phi_0.powi(2) * phi_1n * phi_2.powi(4)
                - 720f64 * phi_0.powi(2) * phi_1n * phi_2.powi(3)
                + 180f64 * phi_0.powi(2) * phi_1n * phi_2.powi(2)
                + 2100f64 * phi_0.powi(2) * phi_1p.powi(4) * phi_2.powi(3)
                - 1260f64 * phi_0.powi(2) * phi_1p.powi(4) * phi_2.powi(2)
                - 252f64 * phi_0.powi(2) * phi_1p.powi(4) * phi_2
                + 84f64 * phi_0.powi(2) * phi_1p.powi(4)
                - 2625f64 * phi_0.powi(2) * phi_1p.powi(3) * phi_2.powi(4)
                + 1050f64 * phi_0.powi(2) * phi_1p.powi(3) * phi_2.powi(2)
                + 360f64 * phi_0.powi(2) * phi_1p.powi(3) * phi_2
                - 120f64 * phi_0.powi(2) * phi_1p.powi(3)
                + 2100f64 * phi_0.powi(2) * phi_1p.powi(2) * phi_2.powi(4)
                - 1400f64 * phi_0.powi(2) * phi_1p.powi(2) * phi_2.powi(3)
                - 120f64 * phi_0.powi(2) * phi_1p.powi(2) * phi_2
                + 40f64 * phi_0.powi(2) * phi_1p.powi(2)
                + 630f64 * phi_0.powi(2) * phi_1p * phi_2.powi(4)
                - 720f64 * phi_0.powi(2) * phi_1p * phi_2.powi(3)
                + 180f64 * phi_0.powi(2) * phi_1p * phi_2.powi(2)
                - 420f64 * phi_0.powi(2) * phi_2.powi(4)
                + 480f64 * phi_0.powi(2) * phi_2.powi(3)
                - 120f64 * phi_0.powi(2) * phi_2.powi(2)
                + 12600f64 * phi_0 * phi_1n.powi(4) * phi_1p.powi(4) * phi_2.powi(2)
                - 6300f64 * phi_0 * phi_1n.powi(4) * phi_1p.powi(4) * phi_2
                - 630f64 * phi_0 * phi_1n.powi(4) * phi_1p.powi(4)
                - 16800f64 * phi_0 * phi_1n.powi(4) * phi_1p.powi(3) * phi_2.powi(3)
                + 5040f64 * phi_0 * phi_1n.powi(4) * phi_1p.powi(3) * phi_2
                + 840f64 * phi_0 * phi_1n.powi(4) * phi_1p.powi(3)
                + 12600f64 * phi_0 * phi_1n.powi(4) * phi_1p.powi(2) * phi_2.powi(3)
                - 7560f64 * phi_0 * phi_1n.powi(4) * phi_1p.powi(2) * phi_2.powi(2)
                - 252f64 * phi_0 * phi_1n.powi(4) * phi_1p.powi(2)
                + 2520f64 * phi_0 * phi_1n.powi(4) * phi_1p * phi_2.powi(3)
                - 2520f64 * phi_0 * phi_1n.powi(4) * phi_1p * phi_2.powi(2)
                + 504f64 * phi_0 * phi_1n.powi(4) * phi_1p * phi_2
                - 1260f64 * phi_0 * phi_1n.powi(4) * phi_2.powi(3)
                + 1260f64 * phi_0 * phi_1n.powi(4) * phi_2.powi(2)
                - 252f64 * phi_0 * phi_1n.powi(4) * phi_2
                - 16800f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(4) * phi_2.powi(3)
                + 5040f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(4) * phi_2
                + 840f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(4)
                + 21000f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(3) * phi_2.powi(4)
                + 12600f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(3) * phi_2.powi(3)
                - 7560f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(3) * phi_2.powi(2)
                - 4200f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(3) * phi_2
                - 1152f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(3)
                - 15750f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2.powi(4)
                + 2520f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2.powi(3)
                + 3780f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2.powi(2)
                + 504f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2
                + 360f64 * phi_0 * phi_1n.powi(3) * phi_1p.powi(2)
                - 3150f64 * phi_0 * phi_1n.powi(3) * phi_1p * phi_2.powi(4)
                - 1260f64 * phi_0 * phi_1n.powi(3) * phi_1p * phi_2.powi(3)
                + 3960f64 * phi_0 * phi_1n.powi(3) * phi_1p * phi_2.powi(2)
                - 972f64 * phi_0 * phi_1n.powi(3) * phi_1p * phi_2
                + 1575f64 * phi_0 * phi_1n.powi(3) * phi_2.powi(4)
                - 1350f64 * phi_0 * phi_1n.powi(3) * phi_2.powi(2)
                + 360f64 * phi_0 * phi_1n.powi(3) * phi_2
                + 12600f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(4) * phi_2.powi(3)
                - 7560f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(4) * phi_2.powi(2)
                - 252f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(4)
                - 15750f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2.powi(4)
                + 2520f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2.powi(3)
                + 3780f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2.powi(2)
                + 504f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2
                + 360f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(3)
                + 9450f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(4)
                - 9660f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(3)
                + 3960f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(2)
                - 972f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2
                - 120f64 * phi_0 * phi_1n.powi(2) * phi_1p.powi(2)
                + 5775f64 * phi_0 * phi_1n.powi(2) * phi_1p * phi_2.powi(4)
                - 3600f64 * phi_0 * phi_1n.powi(2) * phi_1p * phi_2.powi(3)
                - 1350f64 * phi_0 * phi_1n.powi(2) * phi_1p * phi_2.powi(2)
                + 600f64 * phi_0 * phi_1n.powi(2) * phi_1p * phi_2
                - 2100f64 * phi_0 * phi_1n.powi(2) * phi_2.powi(4)
                + 1800f64 * phi_0 * phi_1n.powi(2) * phi_2.powi(3)
                - 120f64 * phi_0 * phi_1n.powi(2) * phi_2
                + 2520f64 * phi_0 * phi_1n * phi_1p.powi(4) * phi_2.powi(3)
                - 2520f64 * phi_0 * phi_1n * phi_1p.powi(4) * phi_2.powi(2)
                + 504f64 * phi_0 * phi_1n * phi_1p.powi(4) * phi_2
                - 3150f64 * phi_0 * phi_1n * phi_1p.powi(3) * phi_2.powi(4)
                - 1260f64 * phi_0 * phi_1n * phi_1p.powi(3) * phi_2.powi(3)
                + 3960f64 * phi_0 * phi_1n * phi_1p.powi(3) * phi_2.powi(2)
                - 972f64 * phi_0 * phi_1n * phi_1p.powi(3) * phi_2
                + 5775f64 * phi_0 * phi_1n * phi_1p.powi(2) * phi_2.powi(4)
                - 3600f64 * phi_0 * phi_1n * phi_1p.powi(2) * phi_2.powi(3)
                - 1350f64 * phi_0 * phi_1n * phi_1p.powi(2) * phi_2.powi(2)
                + 600f64 * phi_0 * phi_1n * phi_1p.powi(2) * phi_2
                - 3360f64 * phi_0 * phi_1n * phi_1p * phi_2.powi(4)
                + 3240f64 * phi_0 * phi_1n * phi_1p * phi_2.powi(3)
                - 360f64 * phi_0 * phi_1n * phi_1p * phi_2.powi(2)
                - 120f64 * phi_0 * phi_1n * phi_1p * phi_2
                + 630f64 * phi_0 * phi_1n * phi_2.powi(4)
                - 720f64 * phi_0 * phi_1n * phi_2.powi(3)
                + 180f64 * phi_0 * phi_1n * phi_2.powi(2)
                - 1260f64 * phi_0 * phi_1p.powi(4) * phi_2.powi(3)
                + 1260f64 * phi_0 * phi_1p.powi(4) * phi_2.powi(2)
                - 252f64 * phi_0 * phi_1p.powi(4) * phi_2
                + 1575f64 * phi_0 * phi_1p.powi(3) * phi_2.powi(4)
                - 1350f64 * phi_0 * phi_1p.powi(3) * phi_2.powi(2)
                + 360f64 * phi_0 * phi_1p.powi(3) * phi_2
                - 2100f64 * phi_0 * phi_1p.powi(2) * phi_2.powi(4)
                + 1800f64 * phi_0 * phi_1p.powi(2) * phi_2.powi(3)
                - 120f64 * phi_0 * phi_1p.powi(2) * phi_2
                + 630f64 * phi_0 * phi_1p * phi_2.powi(4)
                - 720f64 * phi_0 * phi_1p * phi_2.powi(3)
                + 180f64 * phi_0 * phi_1p * phi_2.powi(2)
                - 6300f64 * phi_1n.powi(4) * phi_1p.powi(4) * phi_2.powi(2)
                + 5040f64 * phi_1n.powi(4) * phi_1p.powi(4) * phi_2
                - 630f64 * phi_1n.powi(4) * phi_1p.powi(4)
                + 8400f64 * phi_1n.powi(4) * phi_1p.powi(3) * phi_2.powi(3)
                - 5040f64 * phi_1n.powi(4) * phi_1p.powi(3) * phi_2
                + 840f64 * phi_1n.powi(4) * phi_1p.powi(3)
                - 10080f64 * phi_1n.powi(4) * phi_1p.powi(2) * phi_2.powi(3)
                + 7560f64 * phi_1n.powi(4) * phi_1p.powi(2) * phi_2.powi(2)
                - 252f64 * phi_1n.powi(4) * phi_1p.powi(2)
                + 2520f64 * phi_1n.powi(4) * phi_1p * phi_2.powi(3)
                - 2520f64 * phi_1n.powi(4) * phi_1p * phi_2.powi(2)
                + 504f64 * phi_1n.powi(4) * phi_1p * phi_2
                + 8400f64 * phi_1n.powi(3) * phi_1p.powi(4) * phi_2.powi(3)
                - 5040f64 * phi_1n.powi(3) * phi_1p.powi(4) * phi_2
                + 840f64 * phi_1n.powi(3) * phi_1p.powi(4)
                - 10500f64 * phi_1n.powi(3) * phi_1p.powi(3) * phi_2.powi(4)
                - 10080f64 * phi_1n.powi(3) * phi_1p.powi(3) * phi_2.powi(3)
                + 7560f64 * phi_1n.powi(3) * phi_1p.powi(3) * phi_2.powi(2)
                + 4800f64 * phi_1n.powi(3) * phi_1p.powi(3) * phi_2
                - 1152f64 * phi_1n.powi(3) * phi_1p.powi(3)
                + 12600f64 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2.powi(4)
                + 2520f64 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2.powi(3)
                - 9720f64 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2.powi(2)
                + 504f64 * phi_1n.powi(3) * phi_1p.powi(2) * phi_2
                + 360f64 * phi_1n.powi(3) * phi_1p.powi(2)
                - 3150f64 * phi_1n.powi(3) * phi_1p * phi_2.powi(4)
                + 2700f64 * phi_1n.powi(3) * phi_1p * phi_2.powi(2)
                - 720f64 * phi_1n.powi(3) * phi_1p * phi_2
                - 10080f64 * phi_1n.powi(2) * phi_1p.powi(4) * phi_2.powi(3)
                + 7560f64 * phi_1n.powi(2) * phi_1p.powi(4) * phi_2.powi(2)
                - 252f64 * phi_1n.powi(2) * phi_1p.powi(4)
                + 12600f64 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2.powi(4)
                + 2520f64 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2.powi(3)
                - 9720f64 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2.powi(2)
                + 504f64 * phi_1n.powi(2) * phi_1p.powi(3) * phi_2
                + 360f64 * phi_1n.powi(2) * phi_1p.powi(3)
                - 15750f64 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(4)
                + 9600f64 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(3)
                + 2700f64 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2.powi(2)
                - 720f64 * phi_1n.powi(2) * phi_1p.powi(2) * phi_2
                - 120f64 * phi_1n.powi(2) * phi_1p.powi(2)
                + 4200f64 * phi_1n.powi(2) * phi_1p * phi_2.powi(4)
                - 3600f64 * phi_1n.powi(2) * phi_1p * phi_2.powi(3)
                + 240f64 * phi_1n.powi(2) * phi_1p * phi_2
                + 2520f64 * phi_1n * phi_1p.powi(4) * phi_2.powi(3)
                - 2520f64 * phi_1n * phi_1p.powi(4) * phi_2.powi(2)
                + 504f64 * phi_1n * phi_1p.powi(4) * phi_2
                - 3150f64 * phi_1n * phi_1p.powi(3) * phi_2.powi(4)
                + 2700f64 * phi_1n * phi_1p.powi(3) * phi_2.powi(2)
                - 720f64 * phi_1n * phi_1p.powi(3) * phi_2
                + 4200f64 * phi_1n * phi_1p.powi(2) * phi_2.powi(4)
                - 3600f64 * phi_1n * phi_1p.powi(2) * phi_2.powi(3)
                + 240f64 * phi_1n * phi_1p.powi(2) * phi_2
                - 1260f64 * phi_1n * phi_1p * phi_2.powi(4)
                + 1440f64 * phi_1n * phi_1p * phi_2.powi(3)
                - 360f64 * phi_1n * phi_1p * phi_2.powi(2))
    })
}

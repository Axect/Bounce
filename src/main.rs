use bounce::c1::{c1_potential, c1_deriv};
use peroxide::fuga::*;
use rayon::prelude::*;
use std::cmp::Ordering;

fn main() {
    let u = Uniform(0f64, 1f64);
    let b = Bernoulli(0.5);
    let mut vs = vec![(vec![0f64; 100], vec![0f64; 100]); 10000];
    let phi = linspace(0, 1, 100);

    // Parallel generation via rayon
    vs.par_iter_mut().for_each(|(v, w)| loop {
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
            let g = c1_deriv(phi_0, phi_1n, phi_1p, phi_2);
            *w = phi.fmap(g);
            break;
        }
    });

    let mut df = DataFrame::new(vec![]);
    for (i, (v, w)) in vs.into_iter().enumerate() {
        let mut v = v;
        v.extend(w);
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

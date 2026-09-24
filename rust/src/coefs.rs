fn x2(xs: &[f64]) -> f64 {
    xs.iter().map(|&x| x.powi(2)).sum()
}

fn xy(xs: &[f64], ys: &[f64]) -> f64 {
    xs.iter().zip(ys.iter())
      .map(|(&x, &y)| x*y)
      .sum()
}

pub fn b1(xs: &[f64], ys: &[f64]) -> f64 {
    let xsum: f64 = xs.iter().sum();
    let ysum: f64 = ys.iter().sum();
    let n:    f64 = xs.len() as f64;
    
    let num: f64 = n * xy(xs, ys) - (xsum * ysum);
    let den: f64 = n * x2(xs) - xsum.powi(2);

    num / den
}

pub fn b0(xs: &[f64], ys: &[f64]) -> f64 {
    let xsum: f64 = xs.iter().sum();
    let ysum: f64 = ys.iter().sum();
    let n:    f64 = xs.len() as f64;

    (ysum - b1(xs, ys) * xsum) / n
}

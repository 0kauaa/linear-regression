fn x2(xs: &[f32]) -> f32 {
    xs.iter().map(|&x| x.powi(2)).sum()
}

fn xy(xs: &[f32], ys: &[f32]) -> f32 {
    xs.iter().zip(ys.iter())
      .map(|(&x, &y)| x*y)
      .sum()
}

pub fn b1(xs: &[f32], ys: &[f32]) -> f32 {
    let xsum: f32 = xs.iter().sum();
    let ysum: f32 = ys.iter().sum();
    let n:    f32 = xs.len() as f32;
    
    let num: f32 = n * xy(xs, ys) - (xsum * ysum);
    let den: f32 = n * x2(xs) - xsum.powi(2);

    num / den
}

pub fn b0(xs: &[f32], ys: &[f32]) -> f32 {
    let xsum: f32 = xs.iter().sum();
    let ysum: f32 = ys.iter().sum();
    let n:    f32 = xs.len() as f32;

    (ysum - b1(xs, ys) * xsum) / n
}

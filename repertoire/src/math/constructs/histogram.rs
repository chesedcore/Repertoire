pub struct RangedHistogram {
    bins: Vec<u64>,
    min: f64,
    max: f64,
    out_of_bounds: u64,
    //cache
    bin_width: f64,
    total: u64,
}

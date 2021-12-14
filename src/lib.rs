pub fn detect_increases(values: &[u32]) -> u64 {
    let mut previous = values.get(0).unwrap();
    let mut increases: u64 = 0;
    for v in values.iter().skip(1) {
        if previous < v {
            increases += 1;
        }
        previous = v;
    }
    increases
}

pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    if arr.len() <= 1 {
        return Vec::new();
    }
    let prod: usize = arr.iter().product();
    arr.iter().map(|num| prod / num).collect()
}

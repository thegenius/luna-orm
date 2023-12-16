pub trait ArrayStrEqual {
    fn equal(&self, arr: &[String]) -> bool;
}

impl<const N: usize> ArrayStrEqual for [&str; N] {
    fn equal(&self, arr: &[String]) -> bool {
        self.iter().eq(arr.iter())
    }
}

impl<const N: usize> ArrayStrEqual for &[&str; N] {
    fn equal(&self, arr: &[String]) -> bool {
        self.iter().eq(arr.iter())
    }
}

pub fn array_str_equal(arr_str: &[&str], arr_string: &[&str]) -> bool {
    arr_str.iter().all(|e| arr_string.contains(e))
}

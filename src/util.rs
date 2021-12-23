/// number of kibibytes in a gigabyte
const KIB_IN_GB: f64 = 1024_f64 * 1000_f64;

/// number of bytes in a gigabyte
const B_IN_GB: f64 = 1000_f64 * 1000_f64 * 1000_f64;

/// Converts kilobytes to gigabytes
pub fn kib_2_gb(kb: u64) -> f64 {
    return kb as f64 / KIB_IN_GB;
}

/// Converts bytes to gigabytes
pub fn b_2_gb(bytes: u64) -> f64 {
    return bytes as f64 / B_IN_GB;
}

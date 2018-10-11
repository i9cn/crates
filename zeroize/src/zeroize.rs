use super::secure_zero_memory;

/// Trait for securely erasing types from memory
pub trait Zeroize {
    /// Zero out this object from memory (using Rust or OS intrinsics which
    /// ensure the zeroization operation is not "optimized away")
    fn zeroize(&mut self);
}

// Zeroize implementations for the most common types we might want it for

impl<T> Zeroize for T
where
    T: AsMut<[u8]>,
{
    fn zeroize(&mut self) {
        secure_zero_memory(self.as_mut());
    }
}

#[cfg(test)]
mod tests {
    use super::Zeroize;

    #[test]
    fn zeroize_slice() {
        let mut arr = [42; 3];
        arr.zeroize();
        assert_eq!(arr, [0, 0, 0]);
    }

    #[test]
    fn zeroize_vec() {
        let mut arr = vec![42; 3];
        arr.zeroize();
        assert_eq!(arr.as_slice(), [0, 0, 0]);
    }
}
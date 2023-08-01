const DEFAULT_MEM_SIZE: usize = 65536;

// Macro for checking if index is in memory range
macro_rules! assert_index {
    ($index:expr, $mem_size:expr) => {
        // Index is usize, so only check for upper out of bounds
        assert!(
            $index < $mem_size,
            "[ERROR]: index {} went out of bounds!",
            $index
        );
    };
}

pub struct Memory {
    size: usize,
    array: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Self {
        Memory::new(DEFAULT_MEM_SIZE)
    }
}

impl Memory {
    fn new(size: usize) -> Self {
        Memory {
            size: size,
            array: vec![0; size],
        }
    }

    fn get_byte(&mut self, index: usize) -> u8 {
        assert_index!(index, self.size);
        self.array[index]
    }

    fn set_byte(&mut self, index: usize, value: u8) {
        assert_index!(index, self.size);
        self.array[index] = value;
    }

    fn set_bytes(&mut self, index: usize, values: &[u8]) {
        let (start, end) = (index, index + values.len());

        // Check if both are in bounds
        assert_index!(start, self.size);
        assert_index!(end, self.size);

        self.array[start..end].copy_from_slice(values);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_assert_get() {
        let mut memory = Memory::default();
        memory.get_byte(70000);
    }

    #[test]
    #[should_panic]
    fn test_assert_set() {
        let mut memory = Memory::default();
        memory.set_byte(70000, 2);
    }

    #[test]
    fn test_assert_set_bytes() {
        let mut memory = Memory::default();
        let array: Vec<u8> = vec![0; 10];
        memory.set_bytes(50, &array);
    }

    #[test]
    #[should_panic]
    fn test_assert_set_bytes_fail() {
        let mut memory = Memory::default();
        let array: Vec<u8> = vec![0; 10];
        memory.set_bytes(65530, &array);
    }
}

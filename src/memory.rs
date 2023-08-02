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
    array: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Self {
        Memory::new(DEFAULT_MEM_SIZE)
    }
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Memory {
            array: vec![0; size],
        }
    }

    pub fn read_byte(&mut self, index: u16) -> u8 {
        assert_index!(index as usize, self.array.len());
        self.array[index as usize]
    }

    pub fn set_byte(&mut self, index: u16, value: u8) {
        assert_index!(index as usize, self.array.len());
        self.array[index as usize] = value;
    }

    pub fn set_bytes(&mut self, index: u16, values: &[u8]) {
        let (start, end) = (index as usize, index as usize + values.len());

        // Check if both are in bounds
        assert_index!(start, self.array.len());
        assert_index!(end, self.array.len());

        self.array[start..end].copy_from_slice(values);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_assert_read_byte() {
        let mut memory = Memory::new(48000);
        memory.read_byte(50000);
    }

    #[test]
    #[should_panic]
    fn test_assert_set_byte() {
        let mut memory = Memory::new(48000);
        memory.set_byte(50000, 2);
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

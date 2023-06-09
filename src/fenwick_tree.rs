use std::{
    fmt::Debug,
    ops::{AddAssign, SubAssign},
};

#[derive(Debug, Clone, Copy)]
pub struct GenericError;

#[derive(Debug)]
pub struct FenwickTree<I>
where
    I: Default + Copy + AddAssign + SubAssign + Debug + PartialOrd,
{
    values: Vec<I>,
    tree: Vec<I>,
    final_sum: I,
}

impl<I> FenwickTree<I>
where
    I: Default + Copy + AddAssign + SubAssign + Debug + PartialOrd,
{
    // Constructs a new Fenwick tree
    pub fn with_len(len: usize) -> Self {
        Self {
            values: vec![I::default(); len + 1],
            tree: vec![I::default(); len + 1],
            final_sum: I::default(),
        }
    }

    // Length of the Fenwick tree
    pub fn len(&self) -> usize {
        self.tree.len()
    }

    // Add: update the value at `i` by `delta`
    pub fn set(&mut self, mut i: usize, value: I) -> Result<(), GenericError> {
        let size = self.len();
        i += 1;

        if i >= size {
            return Err(GenericError);
        }

        if value < I::default() {
            return Err(GenericError);
        }

        let old_value = self.values[i];
        self.values[i] = value;

        while i < size {
            self.tree[i] += value;
            self.tree[i] -= old_value;
            i = next(i);
        }

        self.final_sum += value;
        self.final_sum -= old_value;

        Ok(())
    }

    // Get value at `i`
    pub fn get_value(&self, mut i: usize) -> Result<I, GenericError> {
        let size = self.len();
        i += 1;

        if i >= size {
            return Err(GenericError);
        }

        Ok(self.values[i])
    }

    pub fn get_final_sum(&self) -> I {
        return self.final_sum;
    }

    // Get cumulative sum up to `i`
    pub fn get_sum(&self, mut i: usize) -> Result<I, GenericError> {
        let size = self.len();
        i += 1;

        if i >= size {
            return Err(GenericError);
        }

        if i + 1 == size {
            return Ok(self.final_sum);
        }

        let mut res = I::default();
        while i > 0 {
            res += self.tree[i];
            i = parent(i);
        }

        Ok(res)
    }

    // Get smallest index such that prefix sum is not less than the given value
    pub fn get_lower(&self, mut sum: I) -> Result<usize, GenericError> {
        let size = self.len();

        if self.final_sum < sum {
            return Err(GenericError);
        }

        let mut bits = size;
        let mut msb = 0;
        while bits > 1 {
            bits >>= 1;
            msb += 1;
        }
        let mut mask = 1 << msb;

        let mut index = 0;
        while mask > 0 {
            let candidate = index + mask;
            if (candidate < size) && self.tree[candidate] < sum {
                sum -= self.tree[candidate];
                index += mask;
            }
            mask >>= 1;
        }

        Ok(index)
    }

    // Get cumulative sums
    pub fn get_sums(&self) -> Result<Vec<I>, GenericError> {
        let size = self.len();
        let mut res = vec![I::default(); size - 1];
        for i in 0..size - 1 {
            res[i] = self.get_sum(i)?;
        }
        Ok(res)
    }
}

// Find next neighbor in the tree
const fn next(i: usize) -> usize {
    i + (i & (!i + 1))
}

// Find parent of current node
const fn parent(i: usize) -> usize {
    i - (i & (!i + 1))
}

use crate::structs::Cell;

// Function to perform quick sort - takes the vector to sort and the high and low index
pub fn quick_sort(cells_vec: &mut Vec<Cell>, low: i64, high: i64) {
  /* Checks that low isn't equal to high when called if the condition isn't matched then the sort
is complete. If the sort isn't complete, first the data is partitioned, this is done through the
partition function. This function takes in the vector and the low and high index, and first
selects the last value of the vector as the pivot iterates through the array and moves any values
lower than the pivot to the front of the vector. Once the vector has been iterated through the
pivot is moved to the index after where the last value lower than it was placed meaning that the
pivot is now sorted, the function then returns the position of the pivot, this is where the vector
is split - the index of the partition. This value is then used by the quick sort function, through
recursion, to sort the values of the vector above the pivot and then below the pivot, these will
recur until the list is sorted. */
  if low < high {
    let partition_index: i64 = partition(cells_vec, low, high);
    quick_sort(cells_vec, low, partition_index - 1);
    quick_sort(cells_vec, partition_index + 1, high);
  }
}

fn partition (vec_to_sort: &mut Vec<Cell>, low: i64, high: i64) -> i64 {
  // Defines pivot as the last value of the vector
  let pivot: i32 = vec_to_sort[high as usize].cell_number;
  // Defines the index where the current value lower than the pivot is
  let mut low_index: i64 = low-1;
  // Iterates through the given range pushing all values lower than the pivot to the next position low_index
  for i in low..high {
    if vec_to_sort[i as usize].cell_number < pivot {
      low_index += 1;
      vec_to_sort.swap(low_index as usize, i as usize);
    }
  }
  /* Once the vector is iterated through the pivot is moved to its sorted position - on index above
  the last low_index swapped element */
  vec_to_sort.swap((low_index +1) as usize, high as usize);
  // Returns where the vector is split - where the pivot is
  return low_index +1;
}
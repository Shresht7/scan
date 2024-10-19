pub enum Size {
    Fixed(usize),
    Flexible,
}

/// Distribute the given space amongst the elements
pub fn distribute(total: usize, sizes: &[Size]) -> Vec<usize> {
    // Calculate the space occupied by fixed sized elements
    let mut fixed_size = 0;
    // Determine the number of flexible elements
    let mut flexible_count = 0;
    // Track the index of the last flexible element
    let mut last_flexible_element: Option<usize> = None;

    // Iterate over sizes to determine the variables
    for (i, s) in sizes.iter().enumerate() {
        match s {
            Size::Fixed(x) => fixed_size += x,
            _ => {
                last_flexible_element = Some(i);
                flexible_count += 1
            }
        }
    }
    // Determine the space available for flexible elements
    let available_size = total.saturating_sub(fixed_size);

    // Calculate the size available to flexible elements
    let flexible_size = match available_size.checked_div(flexible_count) {
        Some(res) => res,
        None => 0,
    };

    // Calculate the remaining space, if any
    let remainder = match available_size.checked_rem(flexible_count) {
        Some(res) => res,
        None => 0,
    };

    // Map the calculated sizes
    sizes
        .iter()
        .enumerate()
        .map(|(i, s)| match s {
            Size::Fixed(x) => x.clone(),
            Size::Flexible => {
                if last_flexible_element.is_some_and(|idx| idx == i) {
                    flexible_size + remainder
                } else {
                    flexible_size
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_fixed_sizes_as_is() {
        let res = distribute(3, &vec![Size::Fixed(1), Size::Fixed(1), Size::Fixed(1)]);
        assert_eq!(1, res[0]);
        assert_eq!(1, res[1]);
        assert_eq!(1, res[2]);
    }

    #[test]
    fn should_distribute_space_evenly_if_all_are_flexible() {
        let res = distribute(9, &vec![Size::Flexible, Size::Flexible, Size::Flexible]);
        assert_eq!(3, res[0]);
        assert_eq!(3, res[1]);
        assert_eq!(3, res[2]);
    }

    #[test]
    fn should_distribute_remaining_space_evenly() {
        let res = distribute(12, &vec![Size::Flexible, Size::Fixed(2), Size::Flexible]);
        assert_eq!(5, res[0]);
        assert_eq!(2, res[1]);
        assert_eq!(5, res[2]);
    }

    #[test]
    fn should_add_the_remainder_to_the_last_flexible_element() {
        let mixed = distribute(13, &vec![Size::Flexible, Size::Fixed(2), Size::Flexible]);
        assert_eq!(5, mixed[0]);
        assert_eq!(2, mixed[1]);
        assert_eq!(6, mixed[2]);
        let all_flexible = distribute(17, &vec![Size::Flexible, Size::Flexible, Size::Flexible]);
        assert_eq!(5, all_flexible[0]);
        assert_eq!(5, all_flexible[1]);
        assert_eq!(7, all_flexible[2]);
        let all_fixed = distribute(21, &vec![Size::Fixed(3), Size::Fixed(5), Size::Fixed(7)]);
        assert_eq!(3, all_fixed[0]);
        assert_eq!(5, all_fixed[1]);
        assert_eq!(7, all_fixed[2]);
    }
}

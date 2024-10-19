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
    for s in sizes {
        match s {
            Size::Fixed(x) => fixed_size += x,
            _ => flexible_count += 1,
        }
    }
    // Determine the space available for flexible elements
    let available_size = total.saturating_sub(fixed_size);

    // Calculate the size available to flexible elements
    let flexible_size = match available_size.checked_div(flexible_count) {
        Some(res) => res,
        None => 0,
    };

    sizes
        .iter()
        .map(|s| match s {
            Size::Fixed(x) => x.clone(),
            Size::Flexible => flexible_size,
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
    #[ignore = "Not implemented yet"]
    fn should_add_the_remainder_to_the_last_flexible_element() {
        let res = distribute(13, &vec![Size::Flexible, Size::Fixed(2), Size::Flexible]);
        assert_eq!(5, res[0]);
        assert_eq!(2, res[1]);
        assert_eq!(6, res[0]);
    }
}

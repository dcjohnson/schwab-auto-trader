use crate::Error;
use std::{cmp::Ordering, collections::BinaryHeap};

pub fn two_decimals(f: f64) -> f64 {
    (f * 100.0).round() / 100.0
}

struct AllocationBHeapValue {
    value: f64,
    foreign_index: usize,
}

impl Ord for AllocationBHeapValue {
    fn cmp(&self, other: &Self) -> Ordering {
        f64::total_cmp(&self.value, &other.value)
    }
}

impl PartialOrd for AllocationBHeapValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for AllocationBHeapValue {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for AllocationBHeapValue {}

// l = new money to invest
// t = current value of account
// x vector of tuples where the first index is the target investment level and the second index is
// the desired investment level for a given collection.
// return: A vector of tuples, equal in length to x, of the investments per collection.
// Allocates money to the biggest deltas between target investment and current investment first.
pub fn calculate_investment_amount(
    mut l: f64,
    t: f64,
    x: Vec<(f64, f64)>,
) -> Result<Vec<f64>, Error> {
    if l < 0.0 || t < 0.0 {
        return Err(format!("invalid values: l: {}, t: {}", l, t).into());
    }

    let mut x_t = 0.0;
    let mut x_t_o = 0.0;
    for (x_n, x_n_o) in x.iter() {
        if !(1.0 >= *x_n && *x_n >= 0.0) || !(1.0 >= *x_n_o && *x_n_o >= 0.0) {
            return Err(format!("invalid values: x_n: {}, x_n_o: {}", x_n, x_n_o).into());
        }

        x_t += x_n;
        x_t_o += x_n_o;
    }

    //round off
    x_t = two_decimals(x_t);
    x_t_o = two_decimals(x_t_o);

    if x_t != 1.0 || x_t_o != 1.0 {
        return Err(format!("Invalid values: x_1: {}, x_t_o: {}", x_t, x_t_o).into());
    }

    let theta = l + t;

    let mut bheap = BinaryHeap::new();
    for i in 0..(x.len()) {
        let (x_n, x_n_o) = x[i];
        let delta = theta * x_n - t * x_n_o;

        let mut value = two_decimals(if delta <= 0.0 { 0.0 } else { delta });
        // if it is either NaN or infinite, just set it to zero.
        if !value.is_finite() {
            value = 0.0;
        }
        bheap.push(AllocationBHeapValue {
            value,
            foreign_index: i,
        });
    }

    let mut result = Vec::new();
    result.resize_with(x.len(), || 0.0);

    loop {
        let AllocationBHeapValue {
            value,
            foreign_index,
        } = match bheap.pop() {
            Some(a) => a,
            None => break,
        };
        if l == 0.0 || value <= 0.0 {
            result[foreign_index] = 0.0;
        } else if value >= l {
            result[foreign_index] = l;
            l = 0.0;
        } else if l > value {
            result[foreign_index] = value;
            l = two_decimals(l - value);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::{calculate_investment_amount, two_decimals};

    #[test]
    fn test_3_collections() {
        let t = 821.5;
        let l = 352.7;
        let x = Vec::from([(0.60, 0.7), (0.05, 0.1), (0.35, 0.2)]);

        let ao = calculate_investment_amount(l, t, x);
        assert!(ao.is_ok());

        let a = ao.unwrap();
        println!("{:#?}", a);
        assert_eq!(l, two_decimals(a.iter().fold(0.0, |l_o, a_n| l_o + a_n)));
    }

    #[test]
    fn test_2_collections_small_allocation() {
        let t = 821.5;
        let l = 2.0;
        let x = Vec::from([(0.65, 0.7), (0.35, 0.3)]);

        let ao = calculate_investment_amount(l, t, x);
        assert!(ao.is_ok());

        let a = ao.unwrap();
        println!("{:#?}", a);
        assert_eq!(l, two_decimals(a.iter().fold(0.0, |l_o, a_n| l_o + a_n)));
    }

    #[test]
    fn test_5_collections_big_allocation() {
        let t = 821.51;
        let l = 10025.23;
        let x = Vec::from([
            (0.4, 0.2),
            (0.2, 0.2),
            (0.15, 0.25),
            (0.15, 0.2),
            (0.1, 0.15),
        ]);

        let ao = calculate_investment_amount(l, t, x);
        if let Err(ref e) = ao {
            println!("error: {}", e);
        }
        assert!(ao.is_ok());

        let a = ao.unwrap();
        println!("{:#?}", a);
        assert_eq!(l, two_decimals(a.iter().fold(0.0, |l_o, a_n| l_o + a_n)));
    }
}

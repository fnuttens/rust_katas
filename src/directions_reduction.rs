#[derive(Clone, PartialEq, Debug)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

use Direction::*;

fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    let mut v = arr.to_vec();
    let mut i = 0;
    loop {
        if is_scan_complete(i, v.len()) {
            break;
        }

        let target = match v[i] {
            NORTH => SOUTH,
            SOUTH => NORTH,
            EAST => WEST,
            WEST => EAST,
        };

        if v[i + 1] == target {
            // Remove opposite directions
            v.remove(i + 1);
            v.remove(i);
            // Reset index
            i = 0;
        } else {
            i += 1;
        }
    }
    v
}

/// Scan is complete if :
///  - array is empty OR
///  - array contains 1 direction OR
///  - current index points to array's last element
fn is_scan_complete(i: usize, len: usize) -> bool {
    len == 0 || len == 1 || i == len - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        use Direction::*;
        let a = [NORTH, SOUTH, SOUTH, EAST, WEST, NORTH, WEST];
        assert_eq!(dir_reduc(&a), [WEST]);
        let a = [NORTH, WEST, SOUTH, EAST];
        assert_eq!(dir_reduc(&a), [NORTH, WEST, SOUTH, EAST]);
    }
}

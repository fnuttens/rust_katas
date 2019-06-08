use std::cmp::Ordering;

const UPPERCASE_OFFSET: i32 = 64;
const LOWERCASE_OFFSET: i32 = 96;

fn rank(st: &str, we: Vec<i32>, n: usize) -> &str {
    if st.is_empty() {
        return "No participants";
    }

    let names: Vec<&str> = st.split(',').collect();

    if n > names.len() {
        return "Not enough participants";
    }

    let mut rankings: Vec<(i32, &str)> = names
        .iter()
        .map(|name| get_som(name))
        .zip(we.iter())
        .map(|(som, weight)| som * weight)
        .zip(names.clone().into_iter())
        .collect();

    println!("Sorting...");
    rankings.sort_by(|(lscore, lname), (rscore, rname)| {
        if lscore.cmp(rscore) == Ordering::Equal {
            return lname.cmp(rname);
        }
        rscore.cmp(lscore)
    });

    rankings.get(n - 1).unwrap().1
}

fn get_som(name: &str) -> i32 {
    let sum = name.chars().fold(0, |acc, c| {
        let offset = if c.is_ascii_uppercase() {
            UPPERCASE_OFFSET
        } else {
            LOWERCASE_OFFSET
        };

        acc + (c as i32 - offset)
    });
    sum + name.len() as i32
}
#[cfg(test)]
mod tests {
    use super::*;

    fn testing(st: &str, we: Vec<i32>, n: usize, exp: &str) -> () {
        assert_eq!(rank(st, we, n), exp)
    }

    #[test]
    fn basics_rank() {
        testing(
            "Addison,Jayden,Sofia,Michael,Andrew,Lily,Benjamin",
            vec![4, 2, 1, 4, 3, 1, 2],
            4,
            "Benjamin",
        );
        testing(
            "Elijah,Chloe,Elizabeth,Matthew,Natalie,Jayden",
            vec![1, 3, 5, 5, 3, 6],
            2,
            "Matthew",
        );
        testing(
            "Aubrey,Olivai,Abigail,Chloe,Andrew,Elizabeth",
            vec![3, 1, 4, 4, 3, 2],
            4,
            "Abigail",
        );
        testing("Lagon,Lily", vec![1, 5], 2, "Lagon");
    }
}

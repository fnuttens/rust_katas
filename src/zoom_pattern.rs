const BLACK: char = '■';
const WHITE: char = '□';

#[derive(Clone, Copy)]
enum Color {
    Black,
    White,
}

fn zoom(n: i32) -> String {
    zoom_recur(Vec::with_capacity(n as usize), n as usize)
        .iter()
        .map(convert_colors)
        .collect::<Vec<Vec<char>>>()
        .join(&'\n')
        .iter()
        .collect()
}

fn zoom_recur(grid: Vec<Vec<Color>>, n: usize) -> Vec<Vec<Color>> {
    match n {
        1 => vec![vec![Color::Black]],
        _ => {
            let grid = zoom_recur(grid, n - 2);
            add_layer(grid, n)
        }
    }
}

fn add_layer(mut grid: Vec<Vec<Color>>, n: usize) -> Vec<Vec<Color>> {
    let color = match grid[0][0] {
        Color::Black => Color::White,
        Color::White => Color::Black,
    };

    let plain_column = vec![color; n];
    grid.insert(0, plain_column.clone());
    grid.push(plain_column);
    for i in 1..(n - 1) {
        let current_column = &mut grid[i];
        current_column.insert(0, color);
        current_column.push(color);
    }
    grid
}

fn convert_colors(row: &Vec<Color>) -> Vec<char> {
    row.iter()
        .map(|c| match c {
            Color::Black => BLACK,
            Color::White => WHITE,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_1() {
        assert_eq!(zoom(1), "■");
    }

    #[test]
    fn basic_test_2() {
        assert_eq!(
            zoom(3),
            "\
□□□
□■□
□□□"
        );
    }

    #[test]
    fn basic_test_3() {
        assert_eq!(
            zoom(5),
            "\
■■■■■
■□□□■
■□■□■
■□□□■
■■■■■"
        );
    }

    #[test]
    fn basic_test_4() {
        assert_eq!(
            zoom(7),
            "\
□□□□□□□
□■■■■■□
□■□□□■□
□■□■□■□
□■□□□■□
□■■■■■□
□□□□□□□"
        );
    }

    #[test]
    fn basic_test_5() {
        assert_eq!(
            zoom(9),
            "\
■■■■■■■■■
■□□□□□□□■
■□■■■■■□■
■□■□□□■□■
■□■□■□■□■
■□■□□□■□■
■□■■■■■□■
■□□□□□□□■
■■■■■■■■■"
        );
    }
}

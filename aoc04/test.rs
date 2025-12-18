use crate::matrix::Matrix;

mod matrix;

#[test]
fn test_parse_empty() {
    assert_eq!(Matrix::parse_line("", 5, '*'), vec![]);
    assert_eq!(Matrix::parse_line("\n", 5, '*'), vec![]);
    assert_eq!(Matrix::parse_line("****", 0, '*'), vec![]);
}

#[test]
fn test_parse_normal() {
    assert_eq!(Matrix::parse_line("****\n", 2, '*'), vec![true; 2]);
    assert_eq!(Matrix::parse_line("****\n", 4, '*'), vec![true; 4]);
    assert_eq!(Matrix::parse_line("****\n", 10, '*'), vec![true; 4]);
    assert_eq!(
        Matrix::parse_line("**##**\n", 4, '*'),
        [vec![true; 2], vec![false; 2]].concat()
    );
}

#[test]
fn test_parse_random_chars() {
    const TEST1LEN: usize = 13;

    assert_eq!(
        Matrix::parse_line("*JLF83m😀; \n\0*", TEST1LEN, '*'),
        [vec![true], vec![false; TEST1LEN - 2], vec![true]].concat()
    );
}

#[test]
fn test_parse_trim() {
    assert_eq!(
        Matrix::parse_line("   \n\n\0*\n\r\t\t    \n", 2, '*'),
        vec![false, true]
    );
}

#[test]
fn test_matrix_empty() {
    let mut mtx = Matrix::new(10);
    assert_eq!(mtx.width(), 10);
    assert_eq!(mtx.height(), 0);

    mtx.push_line("");
    assert_eq!(mtx.width(), 10);
    assert_eq!(mtx.height(), 0);
    assert_eq!(mtx.get((0, 0)), false);
    assert_eq!(mtx.get((usize::MAX, usize::MAX)), false);
    assert_eq!(mtx.get_coords((-1, -1)), false);
}

#[test]
fn test_matrix_index() {
    let mut mtx = Matrix::new(10);
    assert_eq!(mtx.width(), 10);
    assert_eq!(mtx.height(), 0);

    mtx.push_line("@");
    assert_eq!(mtx.width(), 10);
    assert_eq!(mtx.height(), 1);
    assert_eq!(mtx[(0, 0)], true);
    for i in 1..10 {
        assert_eq!(mtx[(0, i)], false);
    }

    mtx.push_line("@@@@@@@@@@@@@@@@@@@@");
    assert_eq!(mtx.width(), 10);
    assert_eq!(mtx.height(), 2);
    for i in 0..10 {
        assert_eq!(mtx[(1, i)], true);
    }
}

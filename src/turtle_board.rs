use std::collections::HashSet;
use std::vec::Vec;
use std::cmp;
use std::ops::Range;
use std::fmt::{self, Display, Formatter};

const DOT: &str = "+";
const DASH_VER: &str = "|";
const BLANK_VER: &str = " ";

const GAP: &str = "    ";
const DASH_HOR: &str = "----";
const BLANK_HOR: &str = "    ";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bounds {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

impl Bounds {
    pub fn min_bound(bounds: &[Bounds]) -> Bounds {
        Bounds {
            min_x: bounds.iter().map(|bound| bound.min_x).min()
                .expect("cannot find bounds for empty board"),
            min_y: bounds.iter().map(|bound| bound.min_y).min()
                .expect("cannot find bounds for empty board"),
            max_x: bounds.iter().map(|bound| bound.max_x).max()
                .expect("cannot find bounds for empty board"),
            max_y: bounds.iter().map(|bound| bound.max_y).max()
                .expect("cannot find bounds for empty board"),
        }
    }
    
    pub fn min_bound_2_bounds(a: &Bounds, b: &Bounds) -> Bounds {
        Bounds {
            min_x: cmp::min(a.min_x, b.min_x),
            min_y: cmp::min(a.min_y, b.min_y),
            max_x: cmp::max(a.max_x, b.max_x),
            max_y: cmp::max(a.max_y, b.max_y),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Edge {
    Vertical(i32, i32),
    Horizontal(i32, i32),
}

impl Edge {
    fn bounds(&self) -> Bounds {
        match self {
            Edge::Vertical(x, y) => Bounds {
                min_x: *x,
                min_y: *y,
                max_x: *x,
                max_y: *y + 1,
            },
            Edge::Horizontal(x, y) => Bounds {
                min_x: *x,
                min_y: *y,
                max_x: *x + 1,
                max_y: *y,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct TurtleBoard {
    edges: HashSet<Edge>,
    lazy_bounds: bool,
    bounds: Option<Bounds>,
}

impl TurtleBoard {
    pub fn new(lazy_bounds: bool) -> TurtleBoard {
        TurtleBoard {
            edges: HashSet::new(),
            lazy_bounds,
            bounds: None,
        }
    }
    
    pub fn new_lazy() -> TurtleBoard {
        TurtleBoard::new(true)
    }
    
    pub fn new_strict() -> TurtleBoard {
        TurtleBoard::new(false)
    }
    
    pub fn add_vertical_line(&mut self, x: i32, ys: Range<i32>) {
        self.expand_to_fit(Bounds {
            min_x: x,
            max_x: x,
            min_y: ys.start,
            max_y: ys.end,
        });
        for y in ys {
            self.edges.insert(Edge::Vertical(x, y));
        }
    }
    
    pub fn add_horizontal_line(&mut self, xs: Range<i32>, y: i32) {
        self.expand_to_fit(Bounds {
            min_y: y,
            max_y: y,
            min_x: xs.start,
            max_x: xs.end,
        });
        for x in xs {
            self.edges.insert(Edge::Horizontal(x, y));
        }
    }
    
    pub fn contains_vertical_line(&self, x: i32, ys: Range<i32>) -> bool {
        return ys
            .filter(|y| !self.edges.contains(&Edge::Vertical(x, *y)))
            .next()
            .is_none();
    }
    
    pub fn contains_horizontal_line(&self, xs: Range<i32>, y: i32) -> bool {
        return xs
            .filter(|x| !self.edges.contains(&Edge::Horizontal(*x, y)))
            .next()
            .is_none();
    }
    
    fn expand_to_fit(&mut self, bounds_to_fit: Bounds) {
        self.bounds = if self.lazy_bounds {
            None
        } else if self.edges.is_empty() {
            Some(bounds_to_fit)
        } else {
            Some(Bounds::min_bound_2_bounds(&bounds_to_fit, self.bounds()))
        }
    }
    
    fn compute_bounds(&self) -> Bounds {
        let bounds: Vec<_> = self.edges
            .iter()
            .map(|edge| edge.bounds())
            .collect();
        Bounds::min_bound(&bounds[..])
    }
    
    pub fn bounds_uncached(&self) -> Bounds {
        match &self.bounds {
            Some(bounds) => bounds.clone(),
            None => self.compute_bounds()
        }
    }
    
    pub fn bounds(&mut self) -> &Bounds {
        if self.bounds.is_none() {
            self.bounds = Some(self.compute_bounds());
        }
        self.bounds.as_ref().unwrap()
    }
}

impl PartialEq for TurtleBoard {
    fn eq(&self, other: &TurtleBoard) -> bool {
        self.edges == other.edges
    }
}

impl Eq for TurtleBoard {
}

impl Display for TurtleBoard {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        let bounds = self.bounds_uncached();
        
        let draw_line_hor = |formatter: &mut Formatter, y: i32| -> Result<(), fmt::Error> {
            write!(formatter, "{}", DOT)?;
            for x in bounds.min_x..bounds.max_x {
                write!(formatter, "{}",
                    if self.edges.contains(&Edge::Horizontal(x, y)) {
                        DASH_HOR
                    } else {
                        BLANK_HOR
                    }
                )?;
                write!(formatter, "{}", DOT)?;
            }
            Ok(())
        };
        
        let draw_line_ver = |formatter: &mut Formatter, y: i32| -> Result<(), fmt::Error> {
            let add_ver = |formatter: &mut Formatter, x: i32| -> Result<(), fmt::Error> {
                write!(formatter, "{}",
                    if self.edges.contains(&Edge::Vertical(x, y)) {
                        DASH_VER
                    } else {
                        BLANK_VER
                    }
                )?;
                Ok(())
            };
        
            for x in bounds.min_x..bounds.max_x {
                add_ver(formatter, x)?;
                write!(formatter, "{}", GAP)?;
            }
            add_ver(formatter, bounds.max_x)?;
            Ok(())
        };
        
        draw_line_hor(formatter, bounds.max_y)?;
        for y in (bounds.min_y..bounds.max_y).rev() {
            write!(formatter, "\n")?;
            draw_line_ver(formatter, y)?;
            write!(formatter, "\n")?;
            draw_line_hor(formatter, y)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_lines() {
        let mut board = TurtleBoard::new_lazy();
        board.add_horizontal_line(-3..5, 2);
        board.add_horizontal_line(-2..0, -1);
        board.add_vertical_line(3, -12..19);
        
        let mut edges: HashSet<Edge> = HashSet::new();
        for x in -3..5 {
            edges.insert(Edge::Horizontal(x, 2));
        }
        for x in -2..0 {
            edges.insert(Edge::Horizontal(x, -1));
        }
        for y in -12..19 {
            edges.insert(Edge::Vertical(3, y));
        }
        
        assert_eq!(edges, board.edges);
    }
    
    #[test]
    fn contains_lines() {
        let mut board = TurtleBoard::new_lazy();
        board.add_horizontal_line(-3..5, 2);
        board.add_horizontal_line(-2..0, -1);
        board.add_vertical_line(3, -12..19);
        board.add_vertical_line(4, -1..2);
        
        assert!(board.contains_horizontal_line(-1..2, 2));
        assert!(board.contains_horizontal_line(-2..0, -1));
        assert!(board.contains_vertical_line(3, -7..19));
        assert!(board.contains_vertical_line(92, -56..-56));
        
        assert!(!board.contains_horizontal_line(-4..5, 2));
        assert!(!board.contains_horizontal_line(-3..6, 2));
        assert!(!board.contains_horizontal_line(3..4, 0));
        assert!(!board.contains_vertical_line(2, -12..19));
    }
    
    fn test_bounds(is_lazy: bool) {
        let mut board = TurtleBoard::new(is_lazy);
        
        board.add_horizontal_line(-1..2, 3);
        let expected_bounds = Bounds {min_x: -1, max_x: 2, min_y: 3, max_y: 3};
        assert_eq!(&expected_bounds, board.bounds());
        
        board.add_horizontal_line(3..5, -2);
        board.add_horizontal_line(1..2, 0);
        let expected_bounds = Bounds {min_x: -1, max_x: 5, min_y: -2, max_y: 3};
        assert_eq!(&expected_bounds, board.bounds());
        
        let mut board = TurtleBoard::new(is_lazy);
        board.add_vertical_line(-3, 4..7);
        let expected_bounds = Bounds {min_x: -3, max_x: -3, min_y: 4, max_y: 7};
        assert_eq!(&expected_bounds, board.bounds());
        
        board.add_horizontal_line(-12..-6, -20);
        board.add_vertical_line(72, -3..6);
        let expected_bounds = Bounds {min_x: -12, max_x: 72, min_y: -20, max_y: 7};
        assert_eq!(&expected_bounds, board.bounds());
        
        let mut board = TurtleBoard::new(is_lazy);
        board.add_horizontal_line(5..15, 2);
        board.add_vertical_line(8, 17..19);
        let expected_bounds = Bounds {min_x: 5, max_x: 15, min_y: 2, max_y: 19};
        assert_eq!(&expected_bounds, board.bounds());
    }
    
    #[test]
    fn lazy_bounds() {
        test_bounds(true);
    }
    
    #[test]
    fn strict_bounds() {
        test_bounds(false);
    }
    
    fn test_switch_bounds(initial_lazy: bool) {
        let mut board = TurtleBoard::new(initial_lazy);
        board.add_horizontal_line(-3..5, 2);
        board.add_horizontal_line(-2..0, -1);
        board.add_vertical_line(3, -12..19);
        board.add_vertical_line(4, -1..2);
        board.lazy_bounds = !initial_lazy;
        let expected_bounds = Bounds {min_x: -3, max_x: 5, min_y: -12, max_y:19};
        assert_eq!(&expected_bounds, board.bounds());
        board.add_horizontal_line(-17..12, 2);
        let expected_bounds = Bounds {min_x: -17, max_x: 12, min_y: -12, max_y:19};
        assert_eq!(&expected_bounds, board.bounds());
    }
    
    #[test]
    fn lazy_to_strict_bounds() {
        test_switch_bounds(true);
    }
    
    #[test]
    fn strict_to_lazy_bounds() {
        test_switch_bounds(false);
    }
    
    #[test]
    fn display() {        
        let mut board = TurtleBoard::new_strict();
        board.add_horizontal_line(-2..6, 2);
        board.add_vertical_line(3, 4..6);
        let display = format!("{}", board);
        let expected_display =
"\
*'*'*'*'*'*'*'*'*
. . . . . | . . .
*'*'*'*'*'*'*'*'*
. . . . . | . . .
*'*'*'*'*'*'*'*'*
. . . . . . . . .
*'*'*'*'*'*'*'*'*
. . . . . . . . .
*-*-*-*-*-*-*-*-*"
            .replace(" ", GAP)
            .replace("-", DASH_HOR)
            .replace("|", DASH_VER)
            .replace("'", BLANK_HOR)
            .replace(".", BLANK_VER)
            .replace("*", DOT);
        println!("Expected:\n{}\n\nBoard:\n{}", expected_display, board);
        assert_eq!(expected_display, display);
        
        let mut board = TurtleBoard::new_strict();
        board.add_vertical_line(-2, -3..2);
        board.add_horizontal_line(-2..7, 1);
        board.add_vertical_line(3, -2..4);
        let display = format!("{}", board);
        let expected_display =
"\
*'*'*'*'*'*'*'*'*'*
. . . . . | . . . .
*'*'*'*'*'*'*'*'*'*
. . . . . | . . . .
*'*'*'*'*'*'*'*'*'*
| . . . . | . . . .
*-*-*-*-*-*-*-*-*-*
| . . . . | . . . .
*'*'*'*'*'*'*'*'*'*
| . . . . | . . . .
*'*'*'*'*'*'*'*'*'*
| . . . . | . . . .
*'*'*'*'*'*'*'*'*'*
| . . . . . . . . .
*'*'*'*'*'*'*'*'*'*"
            .replace(" ", GAP)
            .replace("-", DASH_HOR)
            .replace("|", DASH_VER)
            .replace("'", BLANK_HOR)
            .replace(".", BLANK_VER)
            .replace("*", DOT);
        println!("Expected:\n{}\n\nBoard:\n{}", expected_display, board);
        assert_eq!(expected_display, display);
        
        let mut board = TurtleBoard::new_strict();
        board.add_vertical_line(-3, 1..2);
        board.add_vertical_line(-2, 1..2);
        let display = format!("{}", board);
        let expected_display =
"\
*'*
| |
*'*"
            .replace(" ", GAP)
            .replace("-", DASH_HOR)
            .replace("|", DASH_VER)
            .replace("'", BLANK_HOR)
            .replace(".", BLANK_VER)
            .replace("*", DOT);
        println!("Expected:\n{}\n\nBoard:\n{}", expected_display, board);
        assert_eq!(expected_display, display);
        
        let mut board = TurtleBoard::new_strict();
        board.add_horizontal_line(1..2, 7);
        let display = format!("{}", board);
        let expected_display =
"\
*-*"
            .replace(" ", GAP)
            .replace("-", DASH_HOR)
            .replace("|", DASH_VER)
            .replace("'", BLANK_HOR)
            .replace(".", BLANK_VER)
            .replace("*", DOT);
        println!("Expected:\n{}\n\nBoard:\n{}", expected_display, board);
        assert_eq!(expected_display, display);
    }
}

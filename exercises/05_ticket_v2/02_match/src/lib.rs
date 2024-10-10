enum Shape {
    Circle,
    Square,
    Rectangle,
    Triangle,
    Pentagon,
    Rhomboid,
}

impl Shape {
    pub fn n_sides(&self) -> u8 {
        use Shape::*;
        match &self {
            // I'll die on the hill that a circle has one side
            Circle => 0,
            Pentagon => 5,
            Triangle => 3,
            Square | Rectangle | Rhomboid => 4,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        assert_eq!(Shape::Circle.n_sides(), 0);
    }

    #[test]
    fn test_square() {
        assert_eq!(Shape::Square.n_sides(), 4);
    }

    #[test]
    fn test_rectangle() {
        assert_eq!(Shape::Rectangle.n_sides(), 4);
    }

    #[test]
    fn test_triangle() {
        assert_eq!(Shape::Triangle.n_sides(), 3);
    }

    #[test]
    fn test_pentagon() {
        assert_eq!(Shape::Pentagon.n_sides(), 5);
    }
}

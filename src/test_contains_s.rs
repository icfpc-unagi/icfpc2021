// lib.rsを弄るとぶつかりまくってだるいことになりそうだからとりあえず別ファイルで作業する

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_rectangle() -> Vec<Point> {
        vec![
            P(0, 0),
            P(0, 10),
            P(10, 10),
            P(10, 0),
        ]
    }

    #[test]
    fn rect1() {
        assert_eq!(P::contains_s(&generate_rectangle(), (P(1, 5), P(9, 5))), true);
    }

    #[test]
    fn rect2() {
        assert_eq!(P::contains_s(&generate_rectangle(), (P(11, 5), P(19, 5))), false);
    }

    #[test]
    fn rect3() {
        assert_eq!(P::contains_s(&generate_rectangle(), (P(0, 5), P(0, 5))), true);
    }

    #[test]
    fn rect4() {
        assert_eq!(P::contains_s(&generate_rectangle(), (P(0, 10), P(10, 10))), true);
    }

    #[test]
    fn rect5() {
        assert_eq!(P::contains_s(&generate_rectangle(), (P(0, 10), P(9, 10))), true);
    }

    #[test]
    fn rect6() {
        assert_eq!(P::contains_s(&generate_rectangle(), (P(1, 10), P(9, 10))), true);
    }

    #[test]
    fn rect7() {
        assert_eq!(P::contains_s(&generate_rectangle(), (P(-1, 10), P(11, 10))), false);
    }

    #[test]
    fn rect8() {
        assert_eq!(P::contains_s(&generate_rectangle(), (P(-1, 10), P(9, 10))), false);
    }

    #[test]
    fn rect9() {
        assert_eq!(P::contains_s(&generate_rectangle(), (P(0, 0), P(10, 10))), true);
    }

    #[test]
    fn rect10() {
        assert_eq!(P::contains_s(&generate_rectangle(), (P(0, 0), P(9, 9))), true);
    }

    #[test]
    fn rect11() {
        assert_eq!(P::contains_s(&generate_rectangle(), (P(-1, -1), P(11, 11))), false);
    }

    #[test]
    fn rect12() {
        assert_eq!(P::contains_s(&generate_rectangle(), (P(1, 0), P(9, 9))), true);
    }
}

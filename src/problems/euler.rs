#[cfg(test)]
mod tests {
    use crate::problems::problem001::problem001;
    use crate::problems::problem002::problem002;
    use crate::problems::problem003::problem003;
    use crate::problems::problem004::problem004;
    use crate::problems::problem005::problem005;
    use crate::problems::problem006::problem006;
    use crate::problems::problem007::problem007;
    use crate::problems::problem008::problem008;
    use crate::problems::problem009::problem009;
    use crate::problems::problem010::problem010;
    use crate::problems::problem011::problem011;
    use crate::problems::problem012::problem012;

    #[test]
    fn test_problem001() {
        let result = problem001(1000);
        assert_eq!(result, 233168);
    }

    #[test]
    fn test_problem002() {
        let result = problem002();
        assert_eq!(result, 4613732);
    }

    #[test]
    fn test_problem003() {
        let result = problem003(600851475143);
        assert_eq!(result, 6857);
    }

    #[test]
    fn test_problem004() {
        let result = problem004(1000);
        assert_eq!(result, 906609);
    }

    #[test]
    fn test_problem005() {
        let result = problem005(20);
        assert_eq!(result, 232792560);
    }

    #[test]
    fn test_problem006() {
        let result = problem006(100);
        assert_eq!(result, 25164150);
    }

    #[test]
    fn test_problem007() {
        let result = problem007(10000);
        assert_eq!(result, 104743);
    }

    #[test]
    fn test_problem008() {
        let result = problem008();
        assert_eq!(result, 23514624000);
    }

    #[test]
    fn test_problem009() {
        let result = problem009(1000);
        assert_eq!(result, 31875000);
    }

    #[test]
    fn test_problem010() {
        let result = problem010(2000000);
        assert_eq!(result, 142913828922);
    }

    #[test]
    fn test_problem011() {
        let result = problem011();
        assert_eq!(result, 70600674);
    }

    #[test]
    fn test_problem012() {
        let result = problem012();
        assert_eq!(result, 76576500);
    }
}


#[cfg(test)]
mod tests {
    use crate::problems::problem101::problem101;
    use crate::problems::problem102::problem102;
    use crate::problems::problem103::problem103;
    use crate::problems::problem104::problem104;
    use crate::problems::problem105::problem105;
    use crate::problems::problem106::problem106;
    use crate::problems::problem107::problem107;
    use crate::problems::problem108::problem108;
    use crate::problems::problem109::problem109;
    use crate::problems::problem110::problem110;

    #[test]
    fn test_problem101() {
        let result = problem101();
        assert_eq!(result, "37076114526");
    }

    #[test]
    fn test_problem102() {
        let result = problem102();
        assert_eq!(result, "228");
    }

    #[test]
    fn test_problem103() {
        let result = problem103();
        assert_eq!(result, "20313839404245");
    }

    #[test]
    fn test_problem104() {
        let result = problem104();
        assert_eq!(result, "329468");
    }

    #[test]
    fn test_problem105() {
        let result = problem105();
        assert_eq!(result, "73702");
    }

    #[test]
    fn test_problem106() {
        let result = problem106();
        assert_eq!(result, "21384");
    }

    #[test]
    fn test_problem107() {
        let result = problem107();
        assert_eq!(result, "259679");
    }

    #[test]
    fn test_problem108() {
        let result = problem108();
        assert_eq!(result, "180180");
    }

    #[test]
    fn test_problem109() {
        let result = problem109();
        assert_eq!(result, "38182");
    }

    #[test]
    fn test_problem110() {
        let result = problem110();
        assert_eq!(result, "9350130049860600");
    }
}

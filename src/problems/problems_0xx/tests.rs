#[cfg(test)]
mod tests {
    use crate::problems::problems_0xx::problem001::problem001;
    use crate::problems::problems_0xx::problem002::problem002;
    use crate::problems::problems_0xx::problem003::problem003;
    use crate::problems::problems_0xx::problem004::problem004;
    use crate::problems::problems_0xx::problem005::problem005;
    use crate::problems::problems_0xx::problem006::problem006;
    use crate::problems::problems_0xx::problem007::problem007;
    use crate::problems::problems_0xx::problem008::problem008;
    use crate::problems::problems_0xx::problem009::problem009;
    use crate::problems::problems_0xx::problem010::problem010;
    use crate::problems::problems_0xx::problem011::problem011;
    use crate::problems::problems_0xx::problem012::problem012;
    use crate::problems::problems_0xx::problem013::problem013;
    use crate::problems::problems_0xx::problem014::problem014;
    use crate::problems::problems_0xx::problem015::problem015;
    use crate::problems::problems_0xx::problem016::problem016;
    use crate::problems::problems_0xx::problem017::problem017;
    use crate::problems::problems_0xx::problem018::problem018;
    use crate::problems::problems_0xx::problem019::problem019;
    use crate::problems::problems_0xx::problem020::problem020;
    use crate::problems::problems_0xx::problem021::problem021;
    use crate::problems::problems_0xx::problem022::problem022;
    use crate::problems::problems_0xx::problem023::problem023;
    use crate::problems::problems_0xx::problem024::problem024;
    use crate::problems::problems_0xx::problem025::problem025;
    use crate::problems::problems_0xx::problem026::problem026;
    use crate::problems::problems_0xx::problem027::problem027;
    use crate::problems::problems_0xx::problem028::problem028;
    use crate::problems::problems_0xx::problem029::problem029;
    use crate::problems::problems_0xx::problem030::problem030;
    use crate::problems::problems_0xx::problem031::problem031;
    use crate::problems::problems_0xx::problem032::problem032;
    use crate::problems::problems_0xx::problem033::problem033;
    use crate::problems::problems_0xx::problem034::problem034;
    use crate::problems::problems_0xx::problem035::problem035;
    use crate::problems::problems_0xx::problem036::problem036;
    use crate::problems::problems_0xx::problem037::problem037;
    use crate::problems::problems_0xx::problem038::problem038;
    use crate::problems::problems_0xx::problem039::problem039;
    use crate::problems::problems_0xx::problem040::problem040;
    use crate::problems::problems_0xx::problem041::problem041;
    use crate::problems::problems_0xx::problem042::problem042;
    use crate::problems::problems_0xx::problem043::problem043;
    use crate::problems::problems_0xx::problem044::problem044;
    use crate::problems::problems_0xx::problem045::problem045;
    use crate::problems::problems_0xx::problem046::problem046;
    use crate::problems::problems_0xx::problem047::problem047;
    use crate::problems::problems_0xx::problem048::problem048;
    use crate::problems::problems_0xx::problem049::problem049;
    use crate::problems::problems_0xx::problem050::problem050;
    use crate::problems::problems_0xx::problem051::problem051;
    use crate::problems::problems_0xx::problem052::problem052;
    use crate::problems::problems_0xx::problem053::problem053;
    use crate::problems::problems_0xx::problem054::problem054;
    use crate::problems::problems_0xx::problem055::problem055;
    use crate::problems::problems_0xx::problem056::problem056;
    use crate::problems::problems_0xx::problem057::problem057;
    use crate::problems::problems_0xx::problem058::problem058;
    use crate::problems::problems_0xx::problem059::problem059;
    use crate::problems::problems_0xx::problem060::problem060;
    use crate::problems::problems_0xx::problem061::problem061;
    use crate::problems::problems_0xx::problem062::problem062;
    use crate::problems::problems_0xx::problem063::problem063;
    use crate::problems::problems_0xx::problem064::problem064;
    use crate::problems::problems_0xx::problem065::problem065;
    use crate::problems::problems_0xx::problem066::problem066;
    use crate::problems::problems_0xx::problem067::problem067;
    use crate::problems::problems_0xx::problem068::problem068;
    use crate::problems::problems_0xx::problem069::problem069;
    use crate::problems::problems_0xx::problem070::problem070;
    use crate::problems::problems_0xx::problem071::problem071;
    use crate::problems::problems_0xx::problem072::problem072;
    use crate::problems::problems_0xx::problem073::problem073;
    use crate::problems::problems_0xx::problem074::problem074;
    use crate::problems::problems_0xx::problem075::problem075;
    use crate::problems::problems_0xx::problem076::problem076;
    use crate::problems::problems_0xx::problem077::problem077;
    use crate::problems::problems_0xx::problem078::problem078;
    use crate::problems::problems_0xx::problem079::problem079;
    use crate::problems::problems_0xx::problem080::problem080;
    use crate::problems::problems_0xx::problem081::problem081;
    use crate::problems::problems_0xx::problem082::problem082;
    use crate::problems::problems_0xx::problem083::problem083;
    use crate::problems::problems_0xx::problem084::problem084;
    use crate::problems::problems_0xx::problem085::problem085;
    use crate::problems::problems_0xx::problem086::problem086;
    use crate::problems::problems_0xx::problem087::problem087;
    use crate::problems::problems_0xx::problem088::problem088;
    use crate::problems::problems_0xx::problem089::problem089;
    use crate::problems::problems_0xx::problem090::problem090;
    use crate::problems::problems_0xx::problem091::problem091;
    use crate::problems::problems_0xx::problem092::problem092;
    use crate::problems::problems_0xx::problem093::problem093;
    use crate::problems::problems_0xx::problem094::problem094;
    use crate::problems::problems_0xx::problem095::problem095;
    use crate::problems::problems_0xx::problem096::problem096;
    use crate::problems::problems_0xx::problem097::problem097;
    use crate::problems::problems_0xx::problem098::problem098;
    use crate::problems::problems_0xx::problem099::problem099;
    use crate::problems::problems_0xx::problem100::problem100;

    #[test]
    fn test_problem001() {
        let result = problem001();
        assert_eq!(result, "233168");
    }

    #[test]
    fn test_problem002() {
        let result = problem002();
        assert_eq!(result, "4613732");
    }

    #[test]
    fn test_problem003() {
        let result = problem003();
        assert_eq!(result, "6857");
    }

    #[test]
    fn test_problem004() {
        let result = problem004();
        assert_eq!(result, "906609");
    }

    #[test]
    fn test_problem005() {
        let result = problem005();
        assert_eq!(result, "232792560");
    }

    #[test]
    fn test_problem006() {
        let result = problem006();
        assert_eq!(result, "25164150");
    }

    #[test]
    fn test_problem007() {
        let result = problem007();
        assert_eq!(result, "104743");
    }

    #[test]
    fn test_problem008() {
        let result = problem008();
        assert_eq!(result, "23514624000");
    }

    #[test]
    fn test_problem009() {
        let result = problem009();
        assert_eq!(result, "31875000");
    }

    #[test]
    fn test_problem010() {
        let result = problem010();
        assert_eq!(result, "142913828922");
    }

    #[test]
    fn test_problem011() {
        let result = problem011();
        assert_eq!(result, "70600674");
    }

    #[test]
    fn test_problem012() {
        let result = problem012();
        assert_eq!(result, "76576500");
    }

    #[test]
    fn test_problem013() {
        let result = problem013();
        assert_eq!(result, "5537376230");
    }

    #[test]
    fn test_problem014() {
        let result = problem014();
        assert_eq!(result, "837799");
    }

    #[test]
    fn test_problem015() {
        let result = problem015();
        assert_eq!(result, "137846528820");
    }

    #[test]
    fn test_problem016() {
        let result = problem016();
        assert_eq!(result, "1366");
    }

    #[test]
    fn test_problem017() {
        let result = problem017();
        assert_eq!(result, "21124");
    }

    #[test]
    fn test_problem018() {
        let result = problem018();
        assert_eq!(result, "1074");
    }

    #[test]
    fn test_problem019() {
        let result = problem019();
        assert_eq!(result, "171");
    }

    #[test]
    fn test_problem020() {
        let result = problem020();
        assert_eq!(result, "648");
    }

    #[test]
    fn test_problem021() {
        let result = problem021();
        assert_eq!(result, "31626");
    }

    #[test]
    fn test_problem022() {
        let result = problem022();
        assert_eq!(result, "871198282");
    }

    #[test]
    fn test_problem023() {
        let result = problem023();
        assert_eq!(result, "4179871");
    }

    #[test]
    fn test_problem024() {
        let result = problem024();
        assert_eq!(result, "2783915460");
    }

    #[test]
    fn test_problem025() {
        let result = problem025();
        assert_eq!(result, "4782");
    }

    #[test]
    fn test_problem026() {
        let result = problem026();
        assert_eq!(result, "983");
    }

    #[test]
    fn test_problem027() {
        let result = problem027();
        assert_eq!(result, "-59231");
    }

    #[test]
    fn test_problem028() {
        let result = problem028();
        assert_eq!(result, "669171001");
    }

    #[test]
    fn test_problem029() {
        let result = problem029();
        assert_eq!(result, "9183");
    }

    #[test]
    fn test_problem030() {
        let result = problem030();
        assert_eq!(result, "443839");
    }

    #[test]
    fn test_problem031() {
        let result = problem031();
        assert_eq!(result, "73682");
    }

    #[test]
    fn test_problem032() {
        let result = problem032();
        assert_eq!(result, "45228");
    }

    #[test]
    fn test_problem033() {
        let result = problem033();
        assert_eq!(result, "100");
    }

    #[test]
    fn test_problem034() {
        let result = problem034();
        assert_eq!(result, "40730");
    }

    #[test]
    fn test_problem035() {
        let result = problem035();
        assert_eq!(result, "55");
    }

    #[test]
    fn test_problem036() {
        let result = problem036();
        assert_eq!(result, "872187");
    }

    #[test]
    fn test_problem037() {
        let result = problem037();
        assert_eq!(result, "748317");
    }

    #[test]
    fn test_problem038() {
        let result = problem038();
        assert_eq!(result, "932718654");
    }

    #[test]
    fn test_problem039() {
        let result = problem039();
        assert_eq!(result, "840");
    }

    #[test]
    fn test_problem040() {
        let result = problem040();
        assert_eq!(result, "210");
    }

    #[test]
    fn test_problem041() {
        let result = problem041();
        assert_eq!(result, "7652413");
    }

    #[test]
    fn test_problem042() {
        let result = problem042();
        assert_eq!(result, "162");
    }

    #[test]
    fn test_problem043() {
        let result = problem043();
        assert_eq!(result, "16695334890");
    }

    #[test]
    fn test_problem044() {
        let result = problem044();
        assert_eq!(result, "5482660");
    }

    #[test]
    fn test_problem045() {
        let result = problem045();
        assert_eq!(result, "1533776805");
    }

    #[test]
    fn test_problem046() {
        let result = problem046();
        assert_eq!(result, "5777");
    }

    #[test]
    fn test_problem047() {
        let result = problem047();
        assert_eq!(result, "134043");
    }

    #[test]
    fn test_problem048() {
        let result = problem048();
        assert_eq!(result, "9110846700");
    }

    #[test]
    fn test_problem049() {
        let result = problem049();
        assert_eq!(result, "296962999629");
    }

    #[test]
    fn test_problem050() {
        let result = problem050();
        assert_eq!(result, "997651");
    }

    #[test]
    fn test_problem051() {
        let result = problem051();
        assert_eq!(result, "121313");
    }

    #[test]
    fn test_problem052() {
        let result = problem052();
        assert_eq!(result, "142857");
    }

    #[test]
    fn test_problem053() {
        let result = problem053();
        assert_eq!(result, "4075");
    }

    #[test]
    fn test_problem054() {
        let result = problem054();
        assert_eq!(result, "376");
    }

    #[test]
    fn test_problem055() {
        let result = problem055();
        assert_eq!(result, "249");
    }

    #[test]
    fn test_problem056() {
        let result = problem056();
        assert_eq!(result, "972");
    }

    #[test]
    fn test_problem057() {
        let result = problem057();
        assert_eq!(result, "153");
    }

    #[test]
    fn test_problem058() {
        let result = problem058();
        assert_eq!(result, "26241");
    }

    #[test]
    fn test_problem059() {
        let result = problem059();
        assert_eq!(result, "129448");
    }

    #[test]
    fn test_problem060() {
        let result = problem060();
        assert_eq!(result, "26033");
    }

    #[test]
    fn test_problem061() {
        let result = problem061();
        assert_eq!(result, "28684");
    }

    #[test]
    fn test_problem062() {
        let result = problem062();
        assert_eq!(result, "127035954683");
    }

    #[test]
    fn test_problem063() {
        let result = problem063();
        assert_eq!(result, "49");
    }

    #[test]
    fn test_problem064() {
        let result = problem064();
        assert_eq!(result, "1322");
    }

    #[test]
    fn test_problem065() {
        let result = problem065();
        assert_eq!(result, "272");
    }

    #[test]
    fn test_problem066() {
        let result = problem066();
        assert_eq!(result, "661");
    }

    #[test]
    fn test_problem067() {
        let result = problem067();
        assert_eq!(result, "7273");
    }

    #[test]
    fn test_problem068() {
        let result = problem068();
        assert_eq!(result, "6531031914842725");
    }

    #[test]
    fn test_problem069() {
        let result = problem069();
        assert_eq!(result, "510510");
    }

    #[test]
    fn test_problem070() {
        let result = problem070();
        assert_eq!(result, "8319823");
    }

    #[test]
    fn test_problem071() {
        let result = problem071();
        assert_eq!(result, "428570");
    }

    #[test]
    fn test_problem072() {
        let result = problem072();
        assert_eq!(result, "303963552391");
    }

    #[test]
    fn test_problem073() {
        let result = problem073();
        assert_eq!(result, "7295372");
    }

    #[test]
    fn test_problem074() {
        let result = problem074();
        assert_eq!(result, "402");
    }

    #[test]
    fn test_problem075() {
        let result = problem075();
        assert_eq!(result, "161667");
    }

    #[test]
    fn test_problem076() {
        let result = problem076();
        assert_eq!(result, "190569291");
    }

    #[test]
    fn test_problem077() {
        let result = problem077();
        assert_eq!(result, "71");
    }

    #[test]
    fn test_problem078() {
        let result = problem078();
        assert_eq!(result, "55374");
    }

    #[test]
    fn test_problem079() {
        let result = problem079();
        assert_eq!(result, "73162890");
    }

    #[test]
    fn test_problem080() {
        let result = problem080();
        assert_eq!(result, "40886");
    }

    #[test]
    fn test_problem081() {
        let result = problem081();
        assert_eq!(result, "427337");
    }

    #[test]
    fn test_problem082() {
        let result = problem082();
        assert_eq!(result, "260324");
    }

    #[test]
    fn test_problem083() {
        let result = problem083();
        assert_eq!(result, "425185");
    }

    #[test]
    fn test_problem084() {
        let result = problem084();
        assert_eq!(result, "101524");
    }

    #[test]
    fn test_problem085() {
        let result = problem085();
        assert_eq!(result, "2772");
    }

    #[test]
    fn test_problem086() {
        let result = problem086();
        assert_eq!(result, "1818");
    }

    #[test]
    fn test_problem087() {
        let result = problem087();
        assert_eq!(result, "1097343");
    }

    #[test]
    fn test_problem088() {
        let result = problem088();
        assert_eq!(result, "7587457");
    }

    #[test]
    fn test_problem089() {
        let result = problem089();
        assert_eq!(result, "743");
    }

    #[test]
    fn test_problem090() {
        let result = problem090();
        assert_eq!(result, "1217");
    }

    #[test]
    fn test_problem091() {
        let result = problem091();
        assert_eq!(result, "14234");
    }

    #[test]
    fn test_problem092() {
        let result = problem092();
        assert_eq!(result, "8581146");
    }

    #[test]
    fn test_problem093() {
        let result = problem093();
        assert_eq!(result, "1258");
    }

    #[test]
    fn test_problem094() {
        let result = problem094();
        assert_eq!(result, "518408346");
    }

    #[test]
    fn test_problem095() {
        let result = problem095();
        assert_eq!(result, "14316");
    }

    #[test]
    fn test_problem096() {
        let result = problem096();
        assert_eq!(result, "24702");
    }

    #[test]
    fn test_problem097() {
        let result = problem097();
        assert_eq!(result, "8739992577");
    }

    #[test]
    fn test_problem098() {
        let result = problem098();
        assert_eq!(result, "18769");
    }

    #[test]
    fn test_problem099() {
        let result = problem099();
        assert_eq!(result, "709");
    }

    #[test]
    fn test_problem100() {
        let result = problem100();
        assert_eq!(result, "756872327473");
    }

}

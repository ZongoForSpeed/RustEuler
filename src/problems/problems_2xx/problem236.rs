use crate::maths::arithmetic::Arithmetic;
use crate::register_problem;
use fraction::GenericFraction;
use num_traits::One;

register_problem!(236, "Luxury Hampers", problem236);

type IFraction = GenericFraction<i64>;

pub fn problem236() -> String {
    // Suppliers 'A' and 'B' provided the following numbers of products for the luxury hamper market:
    //
    //									Product		'A'		'B'
    // 							  Beluga Caviar		5248	640
    //							 Christmas Cake		1312	1888
    // 							   Gammon Joint		2624	3776
    // 							   Vintage Port		5760	3776
    // 						 Champagne Truffles		3936	5664
    //
    // Although the suppliers try very hard to ship their goods in perfect condition, there is
    // inevitably some spoilage - i.e. products gone bad.
    //
    // The suppliers compare their performance using two types of statistic:
    //
    // 		- The five per-product spoilage rates for each supplier are equal to the number of products gone bad divided
    //      by the number of products supplied, for each of the five products in turn.
    //
    // 		- The overall spoilage rate for each supplier is equal to the total number of products gone bad divided by
    //      the total number of products provided by that supplier.
    //
    // To their surprise, the suppliers found that each of the five per-product spoilage rates was worse (higher) for
    // 'B' than for 'A' by the same factor (ratio of spoilage rates), m>1; and yet, paradoxically, the overall spoilage
    // rate was worse for 'A' than for 'B', also by a factor of m.
    //
    // There are thirty-five m>1 for which this surprising result could have occurred, the smallest of which is 1476/1475.
    //
    // What's the largest possible value of m?
    // Give your answer as a fraction reduced to its lowest terms, in the form u/v.
    let mut result: IFraction = IFraction::one();

    for b in 1..=640 {
        for k in 3..=1888 + 3776 + 5664 {
            let mut a = 59 * b;
            let mut r = 5 * k;
            let d = i64::gcd(a, r);
            a /= d;
            r /= d;

            let end = std::cmp::min(5248 / a, (1312 + 2624 + 3936) / r);

            for t in 1..=end {
                let u = 41 * b;
                let v = 5 * a * t;
                if u > v {
                    let x = (a + r) * t;
                    let y = b + k;
                    if 246 * 59 * u * v * x < 17405 * u * u * y {
                        continue;
                    }
                    let n = 246 * 59 * u * v * x - 17405 * u * u * y;
                    let d = 17405 * u * u - 246 * 90 * v * v;
                    if n % d == 0 {
                        let h = n / d;
                        if h <= 3776 {
                            if (90 * h * v) % (59 * u) == 0 {
                                let ratio: IFraction = IFraction::new(u, v);
                                if ratio > result {
                                    result = ratio;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem236::problem236;

    #[test]
    fn test_problem236() {
        let result = problem236();
        assert_eq!(result, "123/59");
    }
}

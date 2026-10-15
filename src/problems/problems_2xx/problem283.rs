use crate::maths::arithmetic::Arithmetic;
use crate::maths::integer_root::isqrt;
use crate::maths::primes::crible235;
use crate::register_problem;

register_problem!(
    283,
    "Integer sided triangles for which the area/perimeter ratio is integral",
    problem283
);

/// Consider the triangle with sides 6, 8 and 10. It can be seen that the perimeter and the area are both equal to
/// 24. So the area/perimeter ratio is equal to 1.
///
/// Consider also the triangle with sides 13, 14 and 15. The perimeter equals 42 while the area is equal to 84. So
/// for this triangle the area/perimeter ratio is equal to 2.
///
/// Find the sum of the perimeters of all integer sided triangles for which the area/perimeter ratios are equal to
/// positive integers not exceeding 1000.
pub fn problem283() -> String {
    let limite: u64 = 1000;

    let mut premiers = Vec::new();
    crible235::<u64, _>(10_000_000, |p| premiers.push(p));

    let mut resultat = 0;
    // cf. article http://forumgeom.fau.edu/FG2007volume7/FG200718.pdf
    for m in 1..limite + 1 {
        // Pour un m donné, toutes les solutions du problème A = m.P sont déterminées
        // par:
        // Pout tous tous les diviseurs u de 2.m et v premier avec u tel que
        // 1 ≤ v ≤ sqrt(3)*u
        // Il y a une identité :
        // 4m²(u² + v²) = [v/(a - 2mv / u - 2mu].[v/(b - 2mv / u - 2mu]
        // avec c = a + b - 4mv/u
        for u in (2 * m).divisors(&premiers) {
            let borne_v = isqrt(3 * u * u);
            for v in 1..=borne_v {
                if u64::gcd(u, v) == 1 {
                    // Toutes les factorisations de 4m²(u² + v²) = δ1 · δ2 va
                    // produire des solutions de la forme :
                    //
                    // a = (δ1 + 2mu) / v + 2mv / u;
                    // b = (δ2 + 2mu) / v + 2mv / u;
                    // c = (δ1 + δ2 + 4mu) / v;
                    //
                    // avec v | δ1 + 2mu et v | δ2 + 2mu
                    let produit = 4 * m * m * (u * u + v * v);
                    for delta1 in produit.divisors(&premiers) {
                        // δ1 ≤ 2m.sqrt(u² + v²)
                        if delta1 as u128 * delta1 as u128 > produit as u128 {
                            break;
                        }

                        let delta2 = produit / delta1;
                        // 4m²(u² + v²) = δ1 · δ2,
                        if (2 * m * u + delta1) % v == 0 && (2 * m * u + delta2) % v == 0 {
                            // v | δ1 + 2mu et v | δ2 + 2mu
                            let a = (delta1 + 2 * m * u) / v + 2 * m * v / u;
                            let b = (delta2 + 2 * m * u) / v + 2 * m * v / u;
                            let c = (delta1 + delta2 + 4 * m * u) / v;

                            if a <= c && b <= c {
                                resultat += a + b + c;
                            }
                        }
                    }
                }
            }
        }
    }

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem283() {
        assert_eq!(problem283(), "28038042525570324");
    }
}

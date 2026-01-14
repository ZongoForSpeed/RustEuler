use crate::maths::power::Power;
use crate::register_problem;
use string_builder::ToBytes;

register_problem!(230, "Fibonacci Words", problem230);

fn letter(n: u128) -> char {
    if n < 6 {
        // return ABBAB[n-1];
        return match n {
            1 | 4 => 'A',
            2 | 3 | 5 => 'B',
            _ => unreachable!(),
        };
    }
    let mut ab = (1, 1);
    while ab.1 < n {
        ab = (ab.1, ab.0 + ab.1);
    }
    letter(n + ab.0 - ab.1)
}

/*
namespace {
    char lettre(nombre n) {
        static const char *fibo = "ABBAB";
        if (n < 6)
            return fibo[n - 1];
        std::pair<nombre, nombre> ab(1, 1);
        while (ab.second < n) {
            ab = std::make_pair(ab.second, ab.first + ab.second);
        }
        return lettre(n + ab.first - ab.second);
    }
}
 */

pub fn problem230() -> String {
    // For any two strings of digits, A and B, we define FA,B to be the sequence (A,B,AB,BAB,ABBAB,...) in which each
    // term is the concatenation of the previous two.
    //
    // Further, we define DA,B(n) to be the nth digit in the first term of FA,B that contains at least n digits.
    //
    // Example:
    //
    // Let A=1415926535, B=8979323846. We wish to find DA,B(35), say.
    //
    // The first few terms of FA,B are:
    // 1415926535
    // 8979323846
    // 14159265358979323846
    // 897932384614159265358979323846
    // 14159265358979323846897932384614159265358979323846
    // Then DA,B(35) is the 35th digit in the fifth term, which is 9.
    //
    // Now we use for A the first 100 digits of pi behind the decimal point:
    //
    // 14159265358979323846264338327950288419716939937510
    // 58209749445923078164062862089986280348253421170679
    //
    // and for B the next hundred digits:
    //
    // 82148086513282306647093844609550582231725359408128
    // 48111745028410270193852110555964462294895493038196 .
    //
    // Find sum n = 0,1,...,17   10n× DA,B((127+19n)×7n) .
    let a:Vec<u8> = "1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679".to_bytes();
    let b:Vec<u8> = "8214808651328230664709384460955058223172535940812848111745028410270193852110555964462294895493038196".to_bytes();
    const O: u8 = '0' as u8;

    let mut result = 0;
    for n in 0..18 {
        let f = (127 + 19 * n) * u128::power(7, n) - 1;
        let c = match letter(f / 100 + 1) {
            'A' => a[f as usize % 100] - O,
            'B' => b[f as usize % 100] - O,
            _ => unreachable!(),
        };
        result += c as u128 * u128::power(10, n);
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem230::problem230;

    #[test]
    fn test_problem230() {
        let result = problem230();
        assert_eq!(result, "850481152593119296");
    }
}

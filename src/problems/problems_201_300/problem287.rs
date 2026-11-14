use crate::register_problem;

register_problem!(
    287,
    "Quadtree encoding (a simple compression algorithm)",
    problem287
);

fn pixel_noir(x: i64, y: i64, n: i64) -> bool {
    // (x - 2**N-1)² + (y - 2**N-1)² ≤ 2**(2N-2)
    let centre = 1i64 << (n - 1);
    (x - centre) * (x - centre) + (y - centre) * (y - centre) <= centre * centre
}

fn algorithme(x1: i64, y1: i64, x2: i64, y2: i64, n: i64) -> i64 {
    let test = 8 * i64::from(pixel_noir(x1, y1, n))
        + 4 * i64::from(pixel_noir(x2 - 1, y1, n))
        + 2 * i64::from(pixel_noir(x2 - 1, y2 - 1, n))
        + i64::from(pixel_noir(x1, y2 - 1, n));

    if test == 15 || test == 0 {
        return 2;
    }

    let xm = (x1 + x2) / 2;
    let ym = (y1 + y2) / 2;

    1 + algorithme(x1, y1, xm, ym, n)
        + algorithme(xm, y1, x2, ym, n)
        + algorithme(xm, ym, x2, y2, n)
        + algorithme(x1, ym, xm, y2, n)
}

/// The quadtree encoding allows us to describe a 2N×2N black and white image
/// as a sequence of bits (0 and 1). Those sequences are to be read from left
/// to right like this:
///
///      - the first bit deals with the complete 2N×2N region;
///      - "0" denotes a split:
///        the current 2n×2n region is divided into 4 sub-regions of dimension
///        2n-1×2n-1,
///        the next bits contains the description of the top left, top right,
///        bottom left and bottom right sub-regions - in that order;
///      - "10" indicates that the current region contains only black pixels;
///      - "11" indicates that the current region contains only white pixels.
///
/// Consider the following 4×4 image (colored marks denote places where a split
/// can occur):
///                                  ■ ■ ■ □
///                                  ■ ■ □ ■
///                                  □ □ ■ ■
///                                  □ □ ■ ■
///
/// This image can be described by several sequences, for example :
/// "001010101001011111011010101010", of length 30, or "0100101111101110",
/// of length 16, which is the minimal sequence for this image.
///
/// For a positive integer N, define DN as the 2N×2N image with the following
/// coloring scheme:
///
/// the pixel with coordinates x = 0, y = 0 corresponds to the bottom left
/// pixel, if (x - 2**(N-1))² + (y - 2**(N-1))² ≤ 2**(2N-2) then the pixel is black,
/// otherwise the pixel is white.
///
/// What is the length of the minimal sequence describing D24 ?
pub fn problem287() -> String {
    let n = 24;
    let p2n = 1i64 << 24;

    let resultat = 1
        + algorithme(0, 0, p2n / 2, p2n / 2, n)
        + 2 * algorithme(p2n / 2, 0, p2n, p2n / 2, n)
        + algorithme(p2n / 2, p2n / 2, p2n, p2n, n);

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem287() {
        assert_eq!(problem287(), "313135496");
    }
}

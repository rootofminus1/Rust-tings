use roots::find_roots_cubic;

use crate::operations::{mod_inv, modn, roots_to_vec};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point {
    Infinity,
    Affine(i32, i32),
}

pub struct EllipticCurve {
    pub a: i32,
    pub b: i32,
    pub p: u64,  // field modulus for the elliptic curve
}

impl EllipticCurve {
    pub fn new(a: i32, b: i32, p: u64) -> Self {
        Self { a, b, p }
    }

    // mirrors modpow - TODO: prob generalize accross groups, maybe
    pub fn mult(&self, mut n: u64, p: Point) -> Point {
        let mut result = Point::Infinity;  // identity element
        let mut addend = p;

        while n > 0 {
            if n & 1 == 1 {
                result = self.add(result, addend);
            }

            addend = self.add(addend, addend);
            n >>= 1;
        }

        result
    }

    pub fn add(&self, p1: Point, p2: Point) -> Point {
        match (p1, p2) {
            (Point::Infinity, q) => q,
            (p, Point::Infinity) => p,
            (Point::Affine(x1, y1), Point::Affine(x2, y2)) => {
                if x1 == x2 && modn(y1 + y2, self.p) == 0 {
                    return Point::Infinity;
                }

                let m = if p1 == p2 {
                    let num = modn(3 * x1 * x1 + self.a, self.p);
                    let den = mod_inv(modn(2 * y1, self.p), self.p);
                    modn(num * den, self.p)
                } else {
                    let num = modn(y2 - y1, self.p);
                    let den = mod_inv(modn(x2 - x1, self.p), self.p);
                    modn(num * den, self.p)
                };

                let x3 = modn(m * m - x1 - x2, self.p);
                let y3 = modn(m * (x1 - x3) - y1, self.p);

                Point::Affine(x3, y3)
            }
        }
    }


    pub fn mult_naive(&self, n: u64, p: Point) -> Point {
        let mut res = p;

        for i in 0..n-1 {
            res = self.add(res,p);
        }

        res
    }


    pub fn add_naive(&self, p1: (f64, f64), p2: (f64, f64)) -> (f64, f64) {
        let (x1, y1) = p1;
        let (x2, y2) = p2;

        // 1. line L, y = mx + b
        // more precisely y = m(x - x1) + y1 for a known point x1 y1

        // slope between them
        let m = if p1 == p2 {
            // d/dx
            (3.0 * x1.powi(2) + self.a as f64) / (2.0 * y1)
        } else {
            ((y2 - y1) / (x2 - x1)) as f64
        };


        // 2. solve L = E

        // cubic equation coefficients
        let c3 = 1_f64;
        let c2 = - m.powi(2);
        let c1 = 2.0 * m * (m * x1 - y1) + self.a as f64;
        let c0 = - m.powi(2) * x1.powi(2) + 2.0 * m * x1 * y1 - y1.powi(2) + self.b as f64;

        let roots = roots_to_vec(find_roots_cubic(c3, c2, c1, c0));
        // println!("roots: {:?}", roots);

        // 3. get only the ones that havent been mentioned already (p1, p2)
        const EPS: f64 = 1e-9;

        let rx = roots
            .into_iter()
            .find(|r| (r - x1).abs() > EPS && (r - x2).abs() > EPS)
            .unwrap();
        // println!("root: {:?}", rx);


        // 4. get y
        let ry = m * (rx - x1) + y1;

        // 5. flip
        let ry = - ry;

        (rx, ry)
    }
}
use std::cmp::{max, min};
use std::str::Chars;

pub struct Diff<'a> {
    a: Chars<'a>,
    b: Chars<'a>,
    m: isize,
    n: isize,
    pub ed: isize,
    pub lcs: String,
    pub ses: Vec<(Ses, char)>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Ses {
    Delete,
    Keep,
    Insert,
}

impl<'a> Diff<'a> {
    pub fn new(a: &'a str, b: &'a str) -> Diff<'a> {
        Diff {
            a: a.chars(),
            b: b.chars(),
            m: a.chars().count() as isize,
            n: b.chars().count() as isize,
            ed: -1,
            lcs: String::new(),
            ses: Vec::new(),
        }
    }
    pub fn ond(mut self) -> Diff<'a> {
        let size = (self.m + self.n + 3) as usize;
        let mut v = vec![-1; size];
        let mut lcs = vec![String::new(); size];
        let mut ses = vec![Vec::new(); size];
        let offset = self.m + 2;

        let (i, l, s) = self.snake(0, 0);
        v[offset as usize] = i;
        lcs[offset as usize] = l;
        ses[offset as usize] = s;
        if self.at_end(0, 0, i, &lcs[offset as usize], &ses[offset as usize]) {
            return self;
        }
        for d in 1..=self.m + self.n {
            for k in (max(-d, -self.m - 1)..=min(d, self.n + 1)).step_by(2) {
                let cur = (offset + k) as usize;
                let (del, ins) = (cur + 1, cur - 1);
                let i = if v[del] + 1 > v[ins] {
                    lcs[cur] = lcs[del].clone();
                    ses[cur] = ses[del].clone();
                    if v[del] + 1 <= self.m {
                        ses[cur].push((
                            Ses::Delete,
                            self.a.clone().nth((v[del] + 1 - 1) as usize).unwrap(),
                        ));
                    }
                    v[del] + 1
                } else {
                    lcs[cur] = lcs[ins].clone();
                    ses[cur] = ses[ins].clone();
                    if v[ins] <= self.n {
                        ses[cur].push((
                            Ses::Insert,
                            self.b.clone().nth((v[ins] + k - 1) as usize).unwrap(),
                        ));
                    }
                    v[ins]
                };

                let (i, l, mut s) = self.snake(k, i);
                v[cur] = i;
                lcs[cur].push_str(l.as_str());
                ses[cur].append(&mut s);
                if self.at_end(d, k, v[cur], &lcs[cur], &ses[cur]) {
                    return self;
                }
            }
        }
        self
    }
    fn snake(&self, k: isize, mut i: isize) -> (isize, String, Vec<(Ses, char)>) {
        let a = self.a.clone().skip(i as usize);
        let b = self.b.clone().skip((i + k) as usize);
        let mut lcs = String::new();
        let mut ses = Vec::new();
        for (a, b) in a.zip(b) {
            if a != b {
                break;
            }
            lcs.push(a);
            ses.push((Ses::Keep, a));
            i += 1;
        }
        (i, lcs, ses)
    }
    fn at_end(
        &mut self,
        d: isize,
        k: isize,
        v: isize,
        lcs: &String,
        ses: &Vec<(Ses, char)>,
    ) -> bool {
        if k == self.n - self.m && v == self.m {
            self.ed = d;
            self.lcs = lcs.clone();
            self.ses = ses.clone();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ed0() {
        for (lhs, rhs, lcs, ses) in vec![
            ("a", "a", "a", vec![(Ses::Keep, 'a')]),
            (
                "abc",
                "abc",
                "abc",
                vec![(Ses::Keep, 'a'), (Ses::Keep, 'b'), (Ses::Keep, 'c')],
            ),
        ] {
            let diff = Diff::new(lhs, rhs).ond();
            assert_eq!(0, diff.ed);
            assert_eq!(lcs, diff.lcs.as_str());
            assert_eq!(ses, diff.ses);
        }
    }
    #[test]
    fn ed1() {
        for (lhs, rhs, lcs, ses) in vec![
            ("a", "ab", "a", vec![(Ses::Keep, 'a'), (Ses::Insert, 'b')]),
            ("ab", "a", "a", vec![(Ses::Keep, 'a'), (Ses::Delete, 'b')]),
        ] {
            let diff = Diff::new(lhs, rhs).ond();
            assert_eq!(1, diff.ed);
            assert_eq!(lcs, diff.lcs.as_str());
            assert_eq!(ses, diff.ses);
        }
    }
    #[test]
    fn diff_test() {
        for (lhs, rhs, ed, lcs, ses) in vec![
            (
                "kitten",
                "sitting",
                5,
                "ittn",
                vec![
                    (Ses::Delete, 'k'),
                    (Ses::Insert, 's'),
                    (Ses::Keep, 'i'),
                    (Ses::Keep, 't'),
                    (Ses::Keep, 't'),
                    (Ses::Delete, 'e'),
                    (Ses::Insert, 'i'),
                    (Ses::Keep, 'n'),
                    (Ses::Insert, 'g'),
                ],
            ),
            (
                "abcdef",
                "dacfea",
                6,
                "acf",
                vec![
                    (Ses::Insert, 'd'),
                    (Ses::Keep, 'a'),
                    (Ses::Delete, 'b'),
                    (Ses::Keep, 'c'),
                    (Ses::Delete, 'd'),
                    (Ses::Delete, 'e'),
                    (Ses::Keep, 'f'),
                    (Ses::Insert, 'e'),
                    (Ses::Insert, 'a'),
                ],
            ),
        ] {
            let diff = Diff::new(lhs, rhs).ond();
            assert_eq!(ed, diff.ed);
            assert_eq!(lcs, diff.lcs.as_str());
            assert_eq!(ses, diff.ses);
        }
    }
}


pub mod problem1 {
    pub fn sum(slice: &[i32]) -> i32 {
        let mut sum = 0;
        for i in slice {
            sum += *i;
        }
        sum
    }

    pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        for val in vs {
            let mut flag = false;
            for v in &res {
                if *val == *v {
                    flag = true;
                    break;
                }
            }
            if flag == false {
                res.push(*val);
            }
        }
        res
    }

    pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
        let mut res = vec![];
        for val in vs {
            if pred(*val) {
                res.push(*val)
            }
        }
        res
    }
}


pub mod problem4 {

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Peg {
        A,
        B,
        C,
    }

    pub type Move = (Peg, Peg);

    pub fn hanoi(num:u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
        if num == 1 {
            vec![(src, dst)]
        } else {
            let mut mv = hanoi(num-1, src, dst, aux);
            mv.push((src, dst));
            let mv2 = hanoi(num-1, aux, src, dst);
            for i in mv2 {
                mv.push(i)
            }
            mv
        }
    }

}

pub mod problem3 {
    pub fn sieve(n: u32) -> Vec<u32> {
        let n = n as usize;
        let mut mask = vec![true; n];
        let mut res = vec![];

        for i in 2..n {
            if mask[i] {
                res.push(i as u32);
                let mut t = i*2;
                while t< n {
                    mask[t] = false;
                    t += i;
                }
            }
        }
        res
    }
}

mod tests_provided;

#[cfg(test)]
mod tests_students {
    use problem1::*;
    use problem3::*;

    #[test]
    fn test_sum_small() {
        let array = [1,2,3];
        assert_eq!(sum(&array), 6);
        assert_eq!(sum(&[]), 0);
    }

    #[test]
    fn test_dedup() {
        let vs = vec![1,2,3,3,4,4,1];
        assert_eq!(dedup(&vs), vec![1,2,3,4]);
    }

    #[test]
    fn test_sieve() {
        assert_eq!(sieve(7), vec![2,3,5]);
    }
}

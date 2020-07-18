use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use num::{integer::Roots, BigUint, Integer, PrimInt};
use primal::is_prime;
use std::{collections::BTreeMap, rc::Rc};

fn prime_sum_u64(i: u64, j: u64) -> u64 {
    // The vast majority are clones of others so sticking them into a rc to avoid the allocations
    let mut m: BTreeMap<(u64, u64), Rc<u64>> = BTreeMap::new();
    let mut to_do = vec![(i, j)];
    while let Some((v, p)) = to_do.pop() {
        let result = if v == 1 {
            Rc::new(0u64)
        }
        else if v == 2 {
            Rc::new(2u64)
        }
        else if p == 1 {
            Rc::new((2 + v) * (v - 1) / 2)
        }
        else if m.contains_key(&(v, p)) {
            continue;
        }
        else if p.pow(2) <= v && is_prime(p) {
            let a = if let Some(a) = m.get(&(v, p - 1)) {
                a.as_ref()
            }
            else {
                to_do.push((v, p));
                to_do.push((v, p - 1));
                continue;
            };

            let b = if let Some(b) = m.get(&(v / p, p - 1)) {
                b.as_ref()
            }
            else {
                to_do.push((v, p));
                to_do.push((v / p, p - 1));
                continue;
            };

            let c = if let Some(c) = m.get(&(p - 1, p - 1)) {
                c.as_ref()
            }
            else {
                to_do.push((v, p));
                to_do.push((p - 1, p - 1));
                continue;
            };
            Rc::new(a - (b - c) * p)
        }
        else {
            if let Some(a) = m.get(&(v, p - 1)) {
                Rc::clone(&a)
            }
            else {
                to_do.push((v, p));
                to_do.push((v, p - 1));
                continue;
            }
        };

        m.insert((v, p), result);
    }

    m.get(&(i, j)).unwrap().as_ref().clone()
}

fn sum_of_primes_under(n: u64) -> u64 {
    let r = n.sqrt() as u64;
    assert!(r * r <= n && (r + 1).pow(2) > n);
    let mut v: Vec<_> = (1..r + 1).map(|i| n / i).collect();
    v.extend((0..*v.last().unwrap()).rev());
    let mut s: BTreeMap<u64, u64> = v.iter().copied().map(|i| (i, i * (i + 1) / 2 - 1)).collect();
    for p in 2..r {
        if s[&p] > s[&(p - 1)] {
            // p is prime
            let sp = s[&(p - 1)];
            let p2 = p * p;
            for &ve in &v {
                if ve < p2 {
                    break;
                }
                *s.get_mut(&ve).unwrap() -= p * (s[&(ve / p)] - sp);
            }
        }
    }
    return s[&n];
}

fn sum_of_primes_u128(n: u128) -> u128 {
    let r = n.sqrt();
    assert!(r * r <= n && (r + 1).pow(2) > n);
    let mut v: Vec<_> = (1..r + 1).map(|i| n / i).collect();
    v.extend((0..*v.last().unwrap()).rev());
    let mut s: BTreeMap<u128, u128> = v.iter().copied().map(|i| (i, i * (i + 1) / 2 - 1)).collect();
    for p in 2..r {
        if s[&p] > s[&(p - 1)] {
            // p is prime
            let sp = s[&(p - 1)];
            let p2 = p * p;
            for &ve in &v {
                if ve < p2 {
                    break;
                }
                *s.get_mut(&ve).unwrap() -= p * (s[&(ve / p)] - sp);
            }
        }
    }
    return s[&n];
}

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("prime_sum");
    for i in [10000].iter() {
        group.bench_with_input(BenchmarkId::new("bigint", i), i, |b, i| b.iter(|| prime_sum_bigint(*i, i.sqrt())));
        group.bench_with_input(BenchmarkId::new("u64", i), i, |b, i| b.iter(|| sum_of_primes_under(*i)));
        group.bench_with_input(BenchmarkId::new("u128", i), i, |b, i| b.iter(|| sum_of_primes_under128(*i as u128)));
    }
    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);

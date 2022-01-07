extern crate sort_key;

use rand::{Rng, SeedableRng};
use sort_key::SortKey;

#[test]
fn unshift_only_scenario() {
    let mut sorted = vec![SortKey::default()];

    for _step in 1..=10_000_000 {
        unshift(&mut sorted);
        sorted.remove(1);
    }

    dbg!(&sorted[0].to_string());
}

#[test]
fn push_only_scenario() {
    let mut sorted = vec![SortKey::default()];

    for _step in 1..=10_000_000 {
        push(&mut sorted);
        sorted.remove(0);
    }

    dbg!(sorted[0].to_string());
}

#[test]
fn real_scenario() {
    let mut sorted = vec![SortKey::default()];
    let mut rng = rand::rngs::SmallRng::seed_from_u64(0xA94B1380F17CE18);

    for _step in 1..=10_000_000 {
        let r: f64 = rng.gen();

        if sorted.len() == 1 {
            if r <= 0.5 {
                unshift(&mut sorted);
            } else {
                push(&mut sorted);
            }
        } else if r <= sorted.len() as f64 / 40.0 {
            sorted.remove(rng.gen_range(0..sorted.len()));
        } else {
            let r: f64 = rng.gen();
            if r <= 0.1 {
                unshift(&mut sorted);
            } else if r <= 0.2 {
                push(&mut sorted);
            } else {
                let after = rng.gen_range(0..sorted.len() - 1);
                insert(&mut sorted, after);
            }
        }

        assert(&sorted);
    }

    dbg!(sorted
        .iter()
        .map(SortKey::to_string)
        .collect::<Vec<String>>());
}

#[test]
fn insert_only_scenario() {
    let mut sorted = vec![SortKey::default()];
    push(&mut sorted);

    for _step in 1..=1_000 {
        insert(&mut sorted, 0);
        sorted.remove(2);
    }

    dbg!(sorted
        .iter()
        .map(SortKey::to_string)
        .collect::<Vec<String>>());
}

#[test]
fn insert_with_rebalance_scenario() {
    let mut sorted = vec![SortKey::default()];
    let mut rng = rand::rngs::SmallRng::seed_from_u64(0xA94B1380F17CE18);

    for step in 1..=10_000_000 {
        let r: f64 = rng.gen();

        if sorted.len() == 1 {
            if r <= 0.5 {
                unshift(&mut sorted);
            } else {
                push(&mut sorted);
            }
        } else if r <= sorted.len() as f64 / 40.0 {
            sorted.remove(rng.gen_range(0..sorted.len()));
        } else {
            let after = rng.gen_range(0..sorted.len() - 1);
            insert(&mut sorted, after);
        }

        if step % 100 == 0 {
            match sorted.last_mut() {
                None => {}
                Some(last) => *last = last.after(),
            }
        }

        assert(&sorted);
    }

    dbg!(sorted
        .iter()
        .map(SortKey::to_string)
        .collect::<Vec<String>>());
}

fn unshift(sorted: &mut Vec<SortKey>) {
    sorted.insert(0, sorted[0].before());
}

fn push(sorted: &mut Vec<SortKey>) {
    sorted.push(sorted.last().unwrap().after());
}

fn insert(sorted: &mut Vec<SortKey>, after: usize) {
    let new_key = sorted[after].between(&sorted[after + 1]);
    sorted.insert(after + 1, new_key);
}

fn assert(sorted: &[SortKey]) {
    for i in 1..sorted.len() {
        assert!(sorted[i - 1] < sorted[i]);
    }
}

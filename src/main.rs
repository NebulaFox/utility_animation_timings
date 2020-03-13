
use std::collections::HashMap;
use std::convert::TryFrom;

use primes::factors;

#[derive(Clone)]
struct CompositeS {
    time_a: u64,
    time_b: u64,
    duration_frames: u64,
    duration_frames_factors: Vec<u64>
}

impl CompositeS {
    fn new(a: u64, b: u64) -> CompositeS {
        let df = (a *60) + 40 + (b*60) + 40;
        CompositeS {
            time_a: a,
            time_b: b,
            duration_frames: df,
            duration_frames_factors: factors(df)
        }
    }

    fn frames_a(&self) -> u64 {
        self.time_a * 60
    }

    fn frames_b(&self) -> u64 {
        self.time_b * 60
    }

    fn print(&self) {
        print!(" {:>2}({:>4}), {:>2}({:>4}) ({:>6}) {:?}",
            self.time_a, self.frames_a(),
            self.time_b, self.frames_b(),
            self.duration_frames,
            self.duration_frames_factors);
    }
}



fn print_composite(composite: &Vec<CompositeS>) {
    for c in composite {
        c.print();
        println!();
    }
}

fn composite_gt_n(composite: &Vec<CompositeS>, n:u64) -> Vec<CompositeS> {
    composite.iter()
        .filter(|c| c.duration_frames_factors.iter().any(|v| *v > n) )
        .map(|c| c.clone())
        .collect()
}

#[allow(dead_code)]
fn compute_composite_gt_n(composite: &Vec<CompositeS>, n: u64) {
    println!("Combinations with factors > {}:", n);
    let cn = composite_gt_n(&composite, n);
    if cn.is_empty() {
        println!("NONE");
    } else {
        print_composite(&cn);
    }
    
    println!("\n");
}

fn main() {
    let timings: Vec<u64> = vec![
        1, 2, 3, 5, 7,
        10
        // 10, 15, 20,
        // 30, 45, 60
    ];

    
    let composite: Vec<_> = timings.iter()
        .map(|t1| {
            timings.iter().map(|t2| {
                CompositeS::new(*t1, *t2)
            }).collect::<Vec<CompositeS>>()
        }).flatten()
        .collect();

    println!("All combinations:");
    print_composite(&composite);

    println!("\n");
    compute_composite_gt_n(&composite, 30);

    println!("\n");


    let factors_arr: Vec<HashMap<u64, i32>> = composite.iter()
        .map(|c| {
            c.duration_frames_factors.iter()
                .fold(HashMap::new(), |mut hm, f| {
                    let e = hm.entry(*f).or_insert(0);
                    *e += 1;
                    hm
                })
        })
        .collect();

    let common_factors: HashMap<u64, i32> = factors_arr.iter()
        .fold(HashMap::new(), |acc_hm, hm| {
            let next_hm = hm.iter()
                .fold(acc_hm, |mut acc_hm, (k, v)| {
                    let e = acc_hm.entry(*k).or_insert(*v);
                    if *e > *v {
                        *e = *v;
                    }
                    acc_hm
                });

            next_hm.iter()
                .filter(|(k, _)| hm.contains_key(k) )
                .fold(HashMap::new(), |mut hm, (k, v)| {
                    hm.insert(*k, *v);
                    hm
                })
        });

    println!("Common Factors:\n{:#?}\n", common_factors);

    let merged_factors: HashMap<u64, i32> = factors_arr.iter()
        .fold(HashMap::new(), |acc_hm, hm| {
            hm.iter()
                .fold(acc_hm, |mut acc_hm, (k, v)| {
                    let e = acc_hm.entry(*k).or_insert(*v);
                    if *e < *v {
                        *e = *v;
                    }
                    acc_hm
                })
        });

    let mut sorted_keys: Vec<_> = merged_factors.keys().map(|k| *k).collect();
    sorted_keys.sort_unstable();

    println!("Combine all factors:\n");

    let number: u128 = sorted_keys.iter()
        .map(|key| (key, merged_factors.get(&key).unwrap()))
        .fold(1u128, |num, (k,v)| {
            if num == 1 {
                print!("{}^{}", k, v);
            } else {
                print!(" x {}^{}", k, v);
            }

            num * u128::from( k.pow(u32::try_from(*v).unwrap()) )
        });

    print!("\n");
    println!("= {}", number);

    println!("\nLoop Values:\n");

    for c in composite {
        let lv = number / u128::from(c.duration_frames);
        println!(" {:>2}({:>4}), {:>2}({:>4}) ({:>6}) {:>20} {:>30}",
        c.time_a, c.frames_a(),
        c.time_b, c.frames_b(),
        c.duration_frames,
        lv,
        u128::from(c.duration_frames) * lv);
    }
}

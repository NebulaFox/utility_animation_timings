
use std::collections::HashMap;
use std::convert::TryFrom;

use primes::factors;

/* Factors */

type Factors = HashMap<u64, u32>;

fn sorted_factors(fs: &Factors) -> Vec<u64> {
    let mut sorted_keys: Vec<_> = fs.keys().map(|k| *k).collect();
    sorted_keys.sort_unstable();
    sorted_keys
}

fn number_and_print(fs: &Factors) -> u128 {
   let number = sorted_factors(fs).iter()
        .map(|key| (key, fs.get(&key).unwrap()))
        .fold(1u128, |num, (k,v)| {
            if num == 1 {
                print!("{}^{}", k, v);
            } else {
                print!(" x {}^{}", k, v);
            }

            num * u128::from( k.pow(u32::try_from(*v).unwrap()) )
        });
    print!("\n");
    number
}

/* CompositeS */

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

    let factors_arr: Vec<Factors> = composite.iter()
        .map(|c| {
            c.duration_frames_factors.iter()
                .fold(HashMap::new(), |mut hm, f| {
                    hm.entry(*f)
                        .and_modify(|e| *e += 1)
                        .or_insert(0);
                    hm
                })
        })
        .collect();

    let merged_factors: Factors = factors_arr.iter()
        .fold(HashMap::new(), |acc_hm, hm| {
            hm.iter()
                .fold(acc_hm, |mut acc_hm, (k, v)| {
                    acc_hm.entry(*k)
                        .and_modify(|e| {
                            if *e < *v {
                                *e = *v;
                            }
                        })
                        .or_insert(*v);
                    
                    acc_hm
                })
        });

    println!("Combine all non-common factors:");
    let merged_factors_number = number_and_print(&merged_factors);
    println!("= {}", merged_factors_number);

    println!("\nLoop Values:\n");

    for c in composite {
        let lv = merged_factors_number / u128::from(c.duration_frames);
        println!(" {:>2}({:>4}), {:>2}({:>4}) ({:>6}) {:>20} {:>30}",
        c.time_a, c.frames_a(),
        c.time_b, c.frames_b(),
        c.duration_frames,
        lv,
        u128::from(c.duration_frames) * lv);
    }
}

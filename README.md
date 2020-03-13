# Timings

This is a utility program
to determine the ultimate number
that will allow a generated set of animations to loop.
The set of animations is generated based on a formula,
using values from a set of timings.

## Prerequisite

* Animations are in units of frames.
* Animations run at 60 frames per second.
* Timings are defined in seconds.

The animation is:

* A hold for `a` seconds
* A transition for 40 frames
* A hold for `b` seconds
* A transition for 40 frames

Then, the formula is the following

```rust
(a *60) + 40 + (b*60) + 40
```


## Breakdown

The defined array of timings are

```rust
let timings: Vec<u64> = vec![ // in seconds
    1, 2, 3, 5, 7,
    10
    // 10, 15, 20,
    // 30, 45, 60
];
```

will be combined as such

```rust
let composite: Vec<_> = timings.iter()
    .map(|t1| {
        timings.iter().map(|t2| {
            CompositeS::new(*t1, *t2)
        }).collect::<Vec<CompositeS>>()
    }).flatten()
    .collect();
```

`CompositeS` creates a struct containing `duration_frames`,
which is defined by the formula

```rust
(a *60) + 40 + (b*60) + 40
```

`CompositeS` also has `duration_frames_factor`
which is calculated by the function
[factors](http://lostinmyterminal.com/primes/primes/fn.factors.html)
from the [prime](https://github.com/wackywendell/primes) crate.

```rust
type Factors = HashMap<u64, u32>;

let factors_arr: Vec<Factors> = composite.iter()
    .map(|c| {
        c.duration_frames_factors.iter()
            .fold(HashMap::new(), |mut hm, f| {
                let e = hm.entry(*f).or_insert(0);
                *e += 1;
                hm
            })
    })
    .collect();
```

The array of `CompositeS` is then mapped over
to create a `HashMap` of number of factors, or `Factors` for each
`duration_frames_factor` of `CompositeS`.

```rust
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
```



The resulting array of `Factors` is then reduced
to a single `Factors` containing the highest number
of each factor.

```rust
// NOT in main.rs
let number = &merged_factors.iter()
    .fold(1u128, |num, (k,v)| {
        num * u128::from( k.pow(u32::try_from(*v).unwrap()) )
    });
```

The factors of `merged_factors` are then reduced
to a single number.

```rust
// NOT in main.rs
let loop_values = composite.iter()
    .map(|c| {
        let lv = number / u128::from(c.duration_frames);
        (lv, u128::from(c.duration_frames) * lv))
    })
```

`composite` is mapped over to test the that `number` is
indeed a number of all factors of all `duration_frames`
from all `CompositeS`.
The test also checks the division is indeed a integer
and not a truncated float.
Running the program will show that this indeed the case.

The `number` is therefore the number that
allows the generated set of animations to loop.

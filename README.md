# Timings

This is a utility program
to determine the ultimate number
that will allow a defined set of timings,
that when combined by a formula,
to be loopable.

## Breakdown

The defined array of timings

```rust
let timings: Vec<u64> = vec![
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

`CompositeS` creates a struct containing `duration_frames`
defined by the formula

```rust
(a *60) + 40 + (b*60) + 40
```

`duration_frames` is the animation timings.
`CompositeS` also has `duration_frames_factor`
which is determined by `factors`,
a function from the [prime](https://github.com/wackywendell/primes) crate.

```rust
let merged_factors: HashMap<u64, i32> = composite.iter()
    .map(|c| {
        c.duration_frames_factors.iter()
            .fold(HashMap::new(), |mut hm, f| {
                let e = hm.entry(*f).or_insert(0);
                *e += 1;
                hm
            })
    })
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
```

The array of `CompositeS` is then mapped over
to create a `HashMap` of number of factors for each
`duration_frames_factor` of `CompositeS`.

The resulting array of `HashMap` of factors is then reduced
to a single `HashMap` containing the highest number
of each factor, `merged_factors`.

```rust
let mut sorted_keys: Vec<_> = merged_factors.keys().map(|k| *k).collect();
sorted_keys.sort_unstable();
```

The `keys` of the `merged_factors` are then stored and sorted, `sorted_keys`.

```rust
let number: u128 = sorted_keys.iter()
    .map(|key| (key, merged_factors.get(&key).unwrap()))
    .fold(1u128, |num, (k,v)| {
        num * u128::from( k.pow(u32::try_from(*v).unwrap()) )
    });
```

The `sorted_keys` are then tupled with their value from
`merged_factors` and then reduced to a single number;
effectively, `number *= k.power_of(v)`.

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
And so is loopable for an integer value.

Running the program will show that this indeed the case.

## Changes

* The `number` was originally `u64` but overflowed
  while testing certain numbers.
* Change some `for` loop usage to `fold` iteration
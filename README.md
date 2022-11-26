# irisdata
[Fisher's Iris data set](https://en.wikipedia.org/wiki/Iris_flower_data_set). Add this to your project with `cargo add irisdata`, then you can use these data however you want:

```rust
let virginica_avg_petal_length = irisdata::IRIS_DATA
    .iter()
    .filter_map(|i| {
        if i.species == Species::IrisVirginica {
            Some(i.petal_length)
        } else {
            None
        }
    })
    .sum::<f32>()
    / 50.;

assert_eq!(5.5520005, avg);
```

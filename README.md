# Match Type

Expand macro differently depending on type of input expression.


## Notes

```rs
let x = 0i8;
let y = m!(x);
```

```rs
macro m(e:E) -> E {
    match_type! e {
        i8 => {true},
        str => {false}, 
    }
}
```

==>>

```rs
let x = 0i8;
let y = {
    trait __Q {
        fn __q(self) -> Self;
    }

    impl __Q for i8 {
        fn __q(self) -> 
    }
}
```
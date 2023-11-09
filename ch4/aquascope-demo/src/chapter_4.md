# Chapter 4

```aquascope,interpreter
#fn free<T>(_t: T) {}

#fn main() {
let b = Box::new([0; 100]);
free(b);
assert!(b[0] == 0);
#}
```


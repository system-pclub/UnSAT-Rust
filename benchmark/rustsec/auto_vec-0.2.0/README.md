# AutoVec
Vec that automatically remove the child when the child is being dropped.  
**This crate is still in development! If you discovered any bugs or problems, please report them on [github issues](https://github.com/lluvz/AutoVec/), which will be much appreciated.**
## Purpose of this crate
The purpose of this crate is create a container to gather variables to be read or processed collectively, while still allowing individual variables to be mutated freely.  
This means that the container need to store raw pointers rather than references. However, if the child variables are moved or dropped, the raw pointers will pointer to invalid datas. Thus, the dropped child need to be removed from the vector, and its place in the memory cannot be changed.  
Luckily, Rust provide the `Drop` trait which allows to define a `drop()` callback to be called when the variable is dropped, which can be used to inform the container to remove the dropped child.  
Rust also provide `pin` to prevent the child from begin moved thus changing its position in the memory.  
Also note that mutating the child itself, e.g. `child1 = child2` will cause child1 to be dropped thus removing it from the vec. This is needed for the reason that the above operation will change the pointer to its container.

# Examples
```
use auto_vec::*;
let mut t1 = AutoChild::new(0);
let mut t2 = AutoChild::new(1);
let mut v = AutoVec::new();
v.add(&mut t1);
v.add(&mut t2);
println!("{:?}", v);
drop(t1);
println!("{:?}", v);
// t1 has been automatically removed from the vector
```
```
use auto_vec::*;
fn main() {
    let mut t1 = AutoChild::new(0);
    let mut t2 = AutoChild::new(1);
    let mut v = AutoVec::new();
    v.add(&mut t1);
    v.add(&mut t2);
    for i in &mut v {
        *i = 3;
    }
    println!("{:?}", v);
}
```
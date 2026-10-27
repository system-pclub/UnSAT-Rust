# Borrowck Scarifices
Keeps the borrow checker happy by combination of safe and unsafe sacrifices.
*This crate mostly deals with limitations of the borrow checker and is not supposed 
to be used to create segfaults.*

## Access to "the rest of the elements" while iterating over a slice

*Splittable* trait is provided to facilitate a common pattern of modifying a container element while iterating over all
other elements of the same container, as in the pseudo-example below:
``` compile_fail
    let mut x = [1,2,3];        
        let hero = &mut x[1];
        for (i,elem) in x.iter_mut().enumerate()
        {
            if i == 1 {continue}
            if elem > hero{
                *hero = 0;
            }
            else{
                *elem = 0;
            }
        }
```
A Splittable trait allows for stuff similar to `split_at_mut()` but better suited to facilitate this usecase.

## Lifetime extension

When making mutable iterators there is a common trick that is often needed to make borrowck admit that giving a mutable reference to an element in next() is indeed safe.
The `lifetime_detach()` allows for the user to do the usual "extend the lifetime" ritual, all in a nice generic package. This also makes it easier to 
make mutable and immutable iterators in one swoop using duplicate! macro.

## Stability, contributions

PR's are welcome, but keep the scope to things related to borrow checking tricks, rather than "anything related to memory".

# Other similar things you may want to do
Also check these crates for more of similar features:
 * https://crates.io/crates/splitmut
 * https://crates.io/crates/safe-transmute
 * https://crates.io/crates/selfie
 * https://crates.io/crates/polonius-the-crab

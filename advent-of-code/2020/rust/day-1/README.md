# Day 1

## The challenge: ## 

### Part 1: ###
```
Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.
``` 

### Part 2: ###
```
The Elves in accounting are thankful for your help; one of them even offers you a starfish coin they had left over from a past vacation. They offer you a second one if you can find three numbers in your expense report that meet the same criteria.
``` 

## My approach ##
It took me a while to get a hang of this problem. Coming from a C-related background, this problem immediately called for the use of nested for loops. It's quite a typical problem really.

The only issue I faced when trying to implement it, was that I had to fight with Rust's ownership model. If you don't properly define the references from the get go, the program will not compile, since you are dealing with three different scopes here. The solution, as already hinted at above, is to obtain an iterator to the referenced vector with all the numbers. This allows us to create multiple iterators to the same object without consuming said object.

After that it's a piece of cake. Just check for the given condition and if it is met multiply the value of all three iterators. 

## Possible improvements ##
While I think that my implementation is sufficient for the given problem, there might be a few ways to further improve the code and make it a bit more "rust-like". For one, we could refactor the data transformation itself and make it more functional: 

```rust
let numbers: Vec<i32> = lines.map(|line| line.parse().unwrap()).collect();
``` 

I think this improves readability and makes the code more elegant.

Possibly, you may be able to tackle the three nested for loops from a different angle, by making use of more Iterator specific implementations. I feel like I am still not versed well enough in Rust to come up with this out of the box, but this could definitely be an idea for further refactoring. For the moment, my current implementation suffices though.  

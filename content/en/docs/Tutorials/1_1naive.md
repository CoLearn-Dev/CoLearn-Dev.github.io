---
title: "1.1 Naive Simulation"
date: 2022-03-17
weight: 4
---
We begin by discussing one simplest secure aggregation protocol. The protocol is to **add masking values before sending numbers from client (participant) to the server (survey organizer)**.  Each pair of users generate a random masking number, and one user adds the number to his value, while the other subtracts the number from his value. When they send their values to the server, the inputs will be random when viewed alone, but summing them up will cancel out the masking values and give the correct summed value.

Let's imagine the university wants to estimate the average hours of sleep its students have everyday, and it presents a survey to students, asking them to fill in the number.  

Let's try to simulate with **Rust**, we first take 1000 random sleep hours from range 5 to 12, which means 100 students take the survey, and their answers are uniformly distributed in the range [5, 12].

```rust
let num_participants = 1000;
let range = Uniform::from(5..12);
let client_vals: Vec<Wrapping<u32>> = rand::thread_rng().sample_iter(&range).take(num_participants).map(|x| Wrapping(x)).collect();
let mut masked_vals: Vec<Wrapping<u32>> = client_vals.clone();
```

Then, to conduct the secure aggregation protocol above, we mask these values before sending them to the server. For each pair of students, we generate a random masking value in range of u32 values. 

Notice that we used `Wrapping<u32>` type instead of `u32` for all values in the code. This is because we want the server to get completely no information from the client, and we need the random number to be taken from the whole range of `u32`. However, operations like summing two large values might cause overflow in rust. Wrapping u32 up in rust could tolerate overflow to allow modular arithmetic, which enhances the privacy of the client.

We add the value to one student's sleeping time, and then subtract the value from the other's sleeping time.  One thing to notice is that each student's sleeping time is masked by **999** masking values since it has a pair with every other student.

```rust
for i in 0..num_participants {
    for j in i+1..num_participants {
        let masking_val: Wrapping<u32> = Wrapping(rand::thread_rng().gen());
        masked_vals[i] = masked_vals[i] + masking_val;
        masked_vals[j] = masked_vals[j] - masking_val;
    }
}
```

Finally, we are gonna aggregate the masked values as a simulation action for server, and check if the server gets the same aggregate value as simply summing up all unmasked values. 

```rust
let naive_aggregate: Wrapping<u32> = client_vals.iter().sum();
let server_aggregate: Wrapping<u32> = masked_vals.iter().sum();
println!("Server Aggregate result: {:.2}", server_aggregate);
println!("Naive Aggregate result: {:.2}", naive_aggregate);

/* Example output
Server Aggregate result: 7978
Naive Aggregate result: 7978

Since each client's initial value is randomly generated, each run of the program may generate different outputs. We only need both results to be same to prove our correctness.
*/
```
We can see that the aggregate server, without knowing the actual sleeping hours of each student, still get the correct summation of the values, and they can compute the average sleeping time with the value. This is how secure aggregation works.



### Full code
```rust
use rand::{distributions::Uniform, Rng};
use std::num::Wrapping;

fn main() {
    let num_participants = 1000;
    let range = Uniform::from(5..12);
    let client_vals: Vec<Wrapping<u32>> = rand::thread_rng().sample_iter(&range).take(num_participants).map(|x| Wrapping(x)).collect();
    let mut masked_vals: Vec<Wrapping<u32>> = client_vals.clone();
    for i in 0..num_participants {
        for j in i+1..num_participants {
            let masking_val: Wrapping<u32> = Wrapping(rand::thread_rng().gen());
            masked_vals[i] = masked_vals[i] + masking_val;
            masked_vals[j] = masked_vals[j] - masking_val;
        }
    }
    let naive_aggregate: Wrapping<u32> = client_vals.iter().sum();
    let server_aggregate: Wrapping<u32> = masked_vals.iter().sum();
    println!("Server Aggregate result: {:.2}", server_aggregate);
    println!("Naive Aggregate result: {:.2}", naive_aggregate);
}
```
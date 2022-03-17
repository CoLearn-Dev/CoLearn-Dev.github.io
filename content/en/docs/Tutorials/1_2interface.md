---
title: "1.2 Interface"
date: 2022-03-17
weight: 4
---

In the last chapter, we showed the correctness of the basic secure aggregation protocol. However, in the real world, to support the application that's mentioned in chapter 1.0, we need to use separate servers for different clients and aggregation server.

In this chapter,  we first illustrate the interface for both the client and the aggregation server. We will extend it to a real-world network-based solution in the next chapter.

### Client

First, let's define a struct called `Client` , with one field representing the original value it wants to send to the server, one field representing the masked value, and last field representing it's name.

```rust
struct Client {
    value: Wrapping<u32>,
    masked_value: Wrapping<u32>,
    name: String,
}
```

We have 4 functions for the interface of the client:

* `new` is used to initialize a new client with a sleeping time passed in as `sending_value` and a name passed in as `name`

  ```rust
  pub fn new (name: String, sending_value: Wrapping<u32>) -> Client {
      Client {value: sending_value, name:name, masked_value: sending_value}
  }
  ```
  
* The aggregation server calls `interact_with_others` to provide a client with lists of its collaborating clients, and the client will generate a masking value for each of its collaborator, subtract the masking value from the masked value, and call `mask_by_adding` for each of its collaborators to let them add the masking value to their masked value.

  ```rust
  fn mask_by_adding (&mut self, masking_val: Wrapping<u32>) {
      self.masked_value = self.masked_value + masking_val;
  }
  
  fn interact_with_others (&mut self, other_clients: &mut Vec<Client>) {
      for curr_collaborator in other_clients {
          let masking_val: Wrapping<u32> = Wrapping(rand::thread_rng().gen());
          self.masked_value = self.masked_value - masking_val;
          curr_collaborator.mask_by_adding(masking_val);
      } 
  }
  ```

* `share_value` is called for each client when the clients finished masking their values and are ready to share the masked value to the server.

  ```rust
  fn share_value (&self) -> Wrapping<u32> {
      self.masked_value
  }
  ```

### Server

We also need a struct for `Server`, with the one field representing the aggregate result, and the other representing the list of all its clients.

```rust
struct Server {
    aggregate_value: Wrapping<u32>,
    clients: Vec<Client>,
}
```

We have two functions for server's interface:

* `initialize` tells its clients to prepare for sharing the masked value by calling `interact_with_others` for each of its client.

  ```rust
  fn initialize(&mut self) {
      for i in 0..self.clients.len() {
          let mut current_client = self.clients.remove(i);
          current_client.interact_with_others(&mut self.clients);
          self.clients.insert(i, current_client);
      }
  }
  ```

* `aggregate` asks for server's clients to share their masked values, and sums up all the values to get the final aggregate value

  ```rust
  fn aggregate(&mut self) -> Wrapping<u32> {
      let mut ret:Wrapping<u32> = Wrapping(0u32);
      for i in 0..self.clients.len() {
          ret += self.clients[i].share_value();
      }
      self.aggregate_value = ret;
      self.aggregate_value
  }
  ```

  

### Combine everything

Now we have some idea about what servers and clients can do. Let's try to simulate the sleeping-time survey again.

We assume there are 1000 participants (clients), and we initialize 1000 clients with a for loop and initialize a server instance by passing in the vector of clients and initial aggregate value of zero to the server constructor.

```rust
let num_participants = 1000;
let mut clients: Vec<Client> = Vec::new();
// Generate all the clients with a for loop.
for i in 0..num_participants {
    let client_name: String = String::from(format!("Client #{}", i+1));
    let client_value: Wrapping<u32> = Wrapping(rand::thread_rng().gen_range(5..12));
    let mut curr_client = Client::new(client_name, client_value);
    clients.push(curr_client);
}
// Generate a server and tells it all the clients we just generated.
let mut server: Server = Server{aggregate_value:Wrapping(0), clients:clients};
```

Before letting clients process their values, we first aggregate their original values. Then we call `initialize` of the server which asks each client to mask their values. Finally we print out the server's aggregate value to see if it equals the naive aggregate value.

```rust
let naive_aggregate: Wrapping<u32> = server.clients.iter().map(|c| c.value).sum();
println!("Naive Aggregate result: {:.2}", naive_aggregate);

server.initialize();
println!("Server Aggregate result: {:.2}", server.aggregate());

/* Example output
Server Aggregate result: 7942
Naive Aggregate result: 7942

Since each client's initial value is randomly generated, each run of the program may generate different outputs. We only need both results to be same to prove our correctness.
*/
```

With a clearer interface, the server only gets the masked value from its clients, and each client only knows its own value.  We can see that two results are the same. These are the interfaces for clients and the aggregation server to implement secure aggregation.

### Full Code

```rust
use rand::{distributions::Uniform, Rng};
use std::num::Wrapping;

#[derive(Debug)]
struct Client {
    value: Wrapping<u32>,
    masked_value: Wrapping<u32>,
    name: String,
}

impl Client {
    pub fn new (name: String, sending_value: Wrapping<u32>) -> Client {
        Client {value: sending_value, name:name, masked_value: sending_value}
    }

    fn mask_by_adding (&mut self, masking_val: Wrapping<u32>) {
        self.masked_value = self.masked_value + masking_val;
    }

    fn interact_with_others (&mut self, other_clients: &mut Vec<Client>) {
        for curr_collaborator in other_clients {
            let masking_val: Wrapping<u32> = Wrapping(rand::thread_rng().gen());
            self.masked_value = self.masked_value - masking_val;
            curr_collaborator.mask_by_adding(masking_val);
        } 
    }

    fn share_value (&self) -> Wrapping<u32> {
        self.masked_value
    }
}

#[derive(Debug)]
struct Server {
    aggregate_value: Wrapping<u32>,
    clients: Vec<Client>,
}

impl Server {
    fn initialize(&mut self) {
        for i in 0..self.clients.len() {
            let mut current_client = self.clients.remove(i);
            current_client.interact_with_others(&mut self.clients);
            self.clients.insert(i, current_client);
        }
    }

    fn aggregate(&mut self) -> Wrapping<u32> {
        let mut ret:Wrapping<u32> = Wrapping(0u32);
        for i in 0..self.clients.len() {
            ret += self.clients[i].share_value();
        }
        self.aggregate_value = ret;
        self.aggregate_value
    }
}

fn main() {
    let num_participants = 1000;
    let mut clients: Vec<Client> = Vec::new();
    // Generate all the clients with a for loop.
    for i in 0..num_participants {
        let client_name: String = String::from(format!("Client #{}", i+1));
        let client_value: Wrapping<u32> = Wrapping(rand::thread_rng().gen_range(5..12));
        let mut curr_client = Client::new(client_name, client_value);
        clients.push(curr_client);
    }
    // Generate a server and tells it all the clients we just generated.
    let mut server: Server = Server{aggregate_value:Wrapping(0), clients:clients};
    
    let naive_aggregate: Wrapping<u32> = server.clients.iter().map(|c| c.value).sum();
    println!("Naive Aggregate result: {:.2}", naive_aggregate);

    server.initialize();
    println!("Server Aggregate result: {:.2}", server.aggregate());
}



```

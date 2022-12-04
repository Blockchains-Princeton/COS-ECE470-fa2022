# Bitcoin Client Project, Part 4

In this part of the project, you will implement the **miner** module of the Bitcoin client. The miner will produce blocks that solve the proof-of-work puzzle.

## Repository management and submission

1. Like the previous assignments, use GitHub and download the zip file. Rename it to your netid as `netid.zip`. Upload the zip file on canvas.
2. TAs will put additional tests (private) on the submission and run them to award marks.

## Code provided
The following files are related to this assignment.
- *src/miner/mod.rs* and *src/miner/worker.rs* - where the mining process takes place.
- *src/api/mod.rs* - an API with which you can interact with your program when running.
- *src/main.rs* - the main function of the program. You need to read and change the code that creates a miner in this part.

To see how the code in these files works, you can run `cargo run -- -vv`, and you will see these logs in the terminal
> Miner initialized into paused mode
> 
> API server listening at 127.0.0.1:7000

This means the miner is not started yet; however, you can use API to start it. In a browser (or *curl* command), go to
http://127.0.0.1:7000/miner/start?lambda=1000000

Then you will see this log in the terminal
> Miner starting in continuous mode with lambda 1000000

This means the miner is started and keeps working in the *main mining loop*. We also provide a parameter *lambda* and use it in the sleep function inside the main mining loop, to avoid excessive CPU usage. In the above example, lambda is 1000000 (microseconds), the miner will sleep for this duration after every iteration of the main mining loop.

`-vv` in `cargo run -- -vv` means the level of logging is 2 (info). With `-vvv` the level is 3 (debug), and you can get more logs in the terminal.

## Programming

You have seen that the miner is working in the *main mining loop*, so the programming goal for this part is to prepare the miner and implement the main mining loop.

### Preparation for miner

You need to add the required components to **Context** struct in *src/miner/mod.rs*

Specifically, the miner needs the following,
1. Blockchain. The miner calls *blockchain.tip()* and sets it as the parent of the block being mined. 
2. A receiver of *ControlSignal*. We want to control the miner to start/stop mining. Also, sometimes, we need the miner to update the context, e.g., the parent of the block being mined. 
3. (Not required in this part) Memory pool. Miner takes transactions from the memory pool and sets them as the content.

After the miner successfully generates a block, it sends the block to a channel *finished_block_chan*. We provide a struct named **Worker** that listens to this channel. The worker does the following:
1. When a block is received from the channel, it needs to insert the block into the blockchain.
2. It also uses a network server handle to broadcast the newly generated blocks' hashes. (Not required in this part.)

Hence, in this part, you need to add blockchain into miner **Context** and **Worker** struct. These structs run in different threads (cf. `thread::Builder::new`); hence we need the thread-safe wrapper of blockchain. Please follow these steps,
1. Read the document of [Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html) and [Mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html) in std crate of Rust.
2. Add `Arc<Mutex<Blockchain>>` to the definition of miner **Context** and **Worker** struct.
3. Add `blockchain: &Arc<Mutex<Blockchain>>` to the argument of *new()* functions. Inside *new()* functions, use `Arc::clone(blockchain)` to get a clone and pass it to the structs.

Finally, you need to go to *src/main.rs*, and change the code related to *new()* functions. You need to first create a new **Blockchain** (already implemented), turn it into `Arc<Mutex<Blockchain>>`, and pass this into *new()* functions.


### Main mining loop

The main mining loop tries random nonces to solve the proof-of-work puzzle. We have provided the loop with some starter code. The actual mining logic may start from the "TODO for student: actual mining" comment.

To build a block, you need to gather a block's fields. In a block header, the fields are gathered as follows,
1. parent - use *blockchain.tip()*. You can have a variable to store it and update it whenever you mine a new block. You could also update the tip when *ControlSignal::Update* is received (we have not implemented this control signal).
2. timestamp - use `std::time`, you can refer to [this document](https://doc.rust-lang.org/std/time/constant.UNIX_EPOCH.html). We suggest using millisecond as the unit rather than second, since second may be too coarse when we measure block delay in the future.
3. difficulty - it should be computed from parent and ancestor blocks with some adaptive rule. In this project, we use the simple rule: a static/constant difficulty: The difficulty of this block should be the same as that of the parent block. (Hence the difficulty will be set in the genesis block)
4. Merkle root - compute it by creating a Merkle tree from the content, i.e., signed transactions.
5. nonce - generate a random nonce (use *rand* crate) in every iteration, or increment nonce (say, increment by 1) in every iteration. PS do you think there is any difference in the probability of solving the puzzle?

As for the block content, you can put arbitrary content since we don't have a memory pool yet in this step. You can place an empty vector or some random transactions.

After you have used these fields to build a block, just check whether the proof-of-work hash puzzle is satisfied by
```
block.hash() <= difficulty
```

If it is satisfied, the block is successfully generated. Congratulations! Just send the block to the channel *finished_block_chan*. And keep on mining for another block. Do not forget to update the parent of the block being mined to the tip of the blockchain!

### Miner worker
To avoid writing an enormous struct and to make auto-grading feasible, we split the miner into two smaller modules, **Context** and **Worker**, and the latter has the following functionality.
1. When a block is received from the channel *finished_block_chan*, it needs to insert the block into the blockchain.
2. It also uses a network server handle to broadcast the newly generated blocks' hashes. (Not required in this part.) 

In this part, you need to finish the first point.

### Miner for test
You need to write a function `fn test_new() -> (Context, Handle, Receiver<Block>)` in *src/miner/mod.rs*, which creates a miner context, a miner handle, and a receiver for testing purposes. This function is called inside the auto-grader and should have no input parameter. It can simply be a one-liner of calling your *new()* function.

## Grading

After you finish the programming, you will have a program that can mine blocks and form a blockchain. You can run `cargo test miner_three_block` to test whether the miner can mine 3 blocks in a one-minute time limit.

We will auto-grade the program by testing whether the miner can mine 10 blocks within a one-minute time limit. (Parameter *lambda=0*.) To do that, you need to set a proper difficulty. Since we use static difficulty, it's sufficient to set that of the genesis block. (Recall that the genesis block is created when calling *Blockchain::new()*.) You can set it to be the largest hash `0xffff....ffff`. In code, you can use `[255u8; 32].into()`.

## Double check
We have provided an (incomplete) autograder. Same instructions as the previous parts.
Please ensure that `test_new()` does not admit any arguments. This function will be called by the autograder.

## FAQ

- *My mined block is not being inserted by the worker* 
    - This may be a locking issue. When you call `blockchain = self.blockchain.lock().unwrap()`, the mutex lock will be held by this thread until it goes out of scope, or it is explicitly dropped. You may be facing a scenario where the worker thread is waiting for the miner loop thread to release the mutex lock before it can insert the block.
    Please use scopes to ensure that the lock is dropped (Eg: `let tip = {self.blockchain.lock().unwrap().tip()};` instead of `let tip = self.blockchain.lock().unwrap().tip();`). 
    The lock can also be dropped using `drop()`. Eg: 
    ```let bc = self.blockchain.lock().unwrap(); let tip = bc.tip(); drop(bc);```)
- *Should I create a new, empty blockchain in the `test_new()` function?* 
     - Yes.
- *I pass the `miner_three_block` test case, but I face an error: `a thread 'miner' panicked at 'Send finished block error: "SendError(..)"`* 
     - This error comes up because the receiving end of finished_block_chan gets disconnected after the test function exits, but the miner is still running in another thread and sending blocks on the same channel. This is fine.


## Advance notice
1. Miner also needs a memory pool. We will cover them in the future.
2. ControlSignal::Update will be used in the future.
3. We will cover the network module in the next part.

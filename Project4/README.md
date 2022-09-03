# Bitcoin Client Project, Part 4

In this part of the project, you are going to implement the **miner** module of Bitcoin client. The miner module, or miner, will produce blocks that solve proof-of-work puzzle.

## Repository management and submission

1. Similar to the previous assignments, use github and download zip file. Rename it to your netid as `netid.zip`. Upload the zip file on canvas.
2. TAs will put additional tests (private) on the submission and run them to award marks.

## Code provided
The following files are related to this assignment.
- *src/miner/mod.rs* and *src/miner/worker.rs* - where the mining process takes place.
- *src/api/mod.rs* - an API with which you can interact with your program when it is running.
- *src/main.rs* - the main function of the program. In this part, you need to read and change the code that creates a miner.

To see how the code in these files works, you can run `cargo run -- -vv` and you will see these logs in the terminal
> Miner initialized into paused mode
> 
> API server listening at 127.0.0.1:7000

This means the miner is not started yet, however, you can use API to start it. In a browser (or *curl* command), go to
http://127.0.0.1:7000/miner/start?lambda=1000000

Then you will see this log in the terminal
> Miner starting in continuous mode with lambda 1000000

This means the miner is started and keeps working in the *main mining loop*. We also provide a parameter *lambda* and use it in sleep function inside the main mining loop, since we don't want the CPU to run crazily. Here lambda is 1000000 (micro seconds), so in each iteration of the main mining loop, it will sleep for that long.

`-vv` in `cargo run -- -vv` means the level of logging is 2 (info). With `-vvv` the level is 3 (debug) and you can get more log in the terminal.

## Programming

You have seen that the miner is working in the *main mining loop*, so the programming goal for this part is to prepare the miner and implement the main mining loop.

### Preparation for miner

You need to add required components to **Context** struct in *src/miner/mod.rs*

Specifically, the miner needs the following,
1. Blockchain. Miner calls *blockchain.tip()* and set it as the parent of the block being mined. 
2. A receiver of *ControlSignal*. We want to control the miner to start/stop mining. Also, sometimes, we need the miner to update the context, e.g., the parent of the block being mined. 
3. (Not required in this part) Memory pool. Miner takes transactions from the memory pool and set them as the content.

After the miner successfully generates a block, it sends the block to a channel *finished_block_chan*. We provide a struct named **Worker** that listens to this channel. The worker does the following:
1. When a block is received from the channel, it needs to insert the block into blockchain.
2. It also uses network server handle to broadcast the newly genereated blocks' hashes. (Not required in this part.)

Hence, in this part, you need to add blockchain into miner **Context** and **Worker** struct. These structs run in another threads (cf. `thread::Builder::new`), hence we need the thread safe wrapper of blockchain. Please follow these steps,
1. Read the document of [Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html) and [Mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html) in std crate of Rust.
2. Add `Arc<Mutex<Blockchain>>` to the definition of miner **Context** and **Worker** struct.
3. Add `blockchain: &Arc<Mutex<Blockchain>>` to the argument of *new()* functions. Inside *new()* functions, use `Arc::clone(blockchain)` to get a clone and pass it to the structs.

At last, you need to go to *src/main.rs*, and change the code related to *new()* functions. You need to first create a new **Blockchain**, then turn it into `Arc<Mutex<Blockchain>>`, then pass it into *new()* functions.


### Main mining loop

The main mining loop is the loop that is trying random nonces to solve the proof-of-work puzzle. We have provided the loop with some code. The actual mining may start from "TODO for student: actual mining" comment.

To build a block, you need to gather a block's fields. In a block header, the fields are gathered as follows,
1. parent - use *blockchain.tip()*. You can have a variable to store it, and only call *blockchain.tip()* when *ControlSignal::Update* is received.
2. timestamp - use `std::time`, you can refer [this document](https://doc.rust-lang.org/std/time/constant.UNIX_EPOCH.html). We suggest to use millisecond as the unit rather than second, since when we measure block delay in the future, second may be too coarse.
3. difficulty - it should be computed from parent and ancestor blocks with some adaptive rule. In this project, we use the simple rule: a static/constant difficulty. This rule just means the difficulty of this block should be the same with that of parent block.
4. merkle root - compute it by creating a merkle tree from the content, i.e., signed transactions.
5. nonce - generate a random nonce (use *rand* crate) in every iteration, or increment nonce (say, increment by 1) in every iteration. P.S. do you think there is any difference in terms of the probability of solving the puzzle?

As for the block content, you can put arbitrary content, since in this step we don't have memory pool yet. You can put an empty vector, or some random transactions.

After you have all these fields and build a block, just check whether the proof-of-work hash puzzle is satisfied by
```
block.hash() <= difficulty
```

If it is satisfied, the block is successfully generated. Congratulations! Just send the block to the channel *finished_block_chan*. And keep on mining for another block. Oh, do not forget to update the parent of the block being mined.

### Miner worker
To avoid writing an enormous struct and to make auto-grading feasible, we split the miner into two smaller modules, **Context** and **Worker**, and the latter has the following functionality.
1. When a block is received from the channel *finished_block_chan*, it needs to insert the block into blockchain.
2. It also uses network server handle to broadcast the newly genereated blocks' hashes. (Not required in this part.) 

In this part, you need to finish the first point.

### Miner for test
You need to write function `fn test_new() -> (Context, Handle, Receiver<Block>)` in *src/miner/mod.rs* which creates a miner context, a miner handle, and a receiver for testing purpose. This function is called inside the auto-grader and should has no input parameter. It can simple be a one-liner of calling your *new()* function.

## Grading

After you finish the programming, you will have a program that can mine blocks and form a blockchain. You can run `cargo test miner_three_block` to test whether the miner can mine 3 blocks in one-minute time limit.

We will auto-grade the program by testing whether the miner can mine 10 blocks in one-minute time limit. (Parameter *lambda=0*.) To do that, you need to set a proper difficulty. Since we use static difficulty, it's sufficient to set that of the genesis block. (Recall that the genesis block is created when calling *Blockchain::new()*.) You can set it to be the largest hash `0xffff....ffff`. In code, you can use `[255u8; 32].into()`.

## Double check
We provide (incomplete) auto-grader for you to test that your code format fits auto-grader. However, passing this auto-grader doesn't guarantee getting full grades. For this assignment, put your zip file with [autograder.sh](autograder.sh) and [add_test.py](add_test.py) in a new directory, from where run
```
bash autograder.sh
```
And you can open the output file _log.txt_, and see whether the auto-grader's tests are passed. You need to have `bash`, `unzip`, and `python3` to run this double check.

## Advance notice
1. Miner also needs memory pool. We will cover them in the future.
2. ControlSignal::Update will be used in the future.
3. We will cover network module in the next part.

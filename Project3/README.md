# Bitcoin Client Project, Part 3

In this part of the project, you are going to finish the **Block** struct and the **Blockchain** struct.

## Repository management and submission

1. Similar to the previous assignments, use github and download zip file. Rename it to your netid as `netid.zip`. Upload the zip file on canvas.
2. TAs will put additional tests (private) on the submission and run them to award marks.

## Code provided
The code we provide for this project is the same as previous assignments. The following files are related to this assignment.
1. *src/types/block.rs* - please finish **Block** struct and some functions related to it in this file.
2. *src/blockchain/mod.rs* - please finish **Blockchain** struct and some functions related to it in this file. (You can also split codes into several files inside directory *src/blockchain/*.)

## Programming

### Block

You need to define a **Block** similar to that in Bitcoin. We require that a block must include:
1. parent - a hash pointer to parent block. Please use **H256** that we provide.
2. nonce - a random integer that will be used in proof-of-work mining. We suggest to simply use **u32**.
3. difficulty - the mining difficulty, i.e., the threshold in proof-of-work check. Please use **H256** since we provide the comparison function, with which you can simply write `if hash <= difficulty`. (Proof-of-work check or mining is not required in this part.)
4. timestamp - the timestamp when this block is generated.
5. merkle\_root - the Merkle root of data (explained below in 6.).

The above fields are also known as **Header**. We suggest (but not require) you to create a struct **Header** to include them.

6. data/content - the actual transactions carried by this block. We suggest to use a **Vec** of **SignedTransaction**. SignedTransaction struct is the one you wrote in previous assignment.

We suggest (but not require) you to create a struct **Content** to include the content.

Notice that to create the Merkle root of **SignedTransaction**, you need to implement trait **Hashable** for **SignedTransaction**. The way to implement that trait is first serialize it into bytes, then call SHA256 to hash the bytes.

You need to implement trait **Hashable** for **Block**. They way to hash a block is to hash **Header** rather than **Content**. So you can first implement **Hashable** for **Header**. When you hash a **Block**, you can directly call the hash function of **Header**.

To test and debug, you need to finish the function `generate_random_block()`. This function takes an argument named *parent*. The generated block should contain that *parent*. And the *nonce* should be a random integer. As for content, you can simply let it be empty. So merkle\_root should be the Merkle root of an empty input. As for fields such as difficulty and timestamp, choose whatever you like.

### Blockchain

You need to finish a struct named **Blockchain**, which contains the necessary information of a direct acyclic graph (DAG) and provides functions related to the longest chain rule. The following functions are required:
1. new() - create a new blockchain that only contains the information of the genesis block. (Define genesis block by your self.)
2. insert() - insert a block into the blockchain. You can (but not required) make it return struct `Result` since if you insert an invalid block, you need to handle the error. 
3. tip() - return the last block's hash in the longest chain.
4. all_blocks_in_longest_chain() - return all blocks' hashes in a vector, from the genesis to the tip. This function will not be tested in this part, and will be used in the future.

#### Storage choice

We suggest that you use a **HashMap** in standard crate to store blocks. You can use hash as key and the block as value. It can look up for blocks by hash very conveniently.

You can also store the tip, and update it after inserting a block. If, say, your current tip is hash(B1), and you insert a new block B2. You need to update tip to hash(B2) if and only if the length of chain B2 is *strictly greater* than that of B1.

Since we are following the longest chain rule, you may also store the length/height of each block. And use the height to determine the longest chain. E.g., genensis block has height 0. This step is not required.

You can implement with persistent storage such as database, but this is not the point of this project and we suggest to just use in-memory storage.

#### Thread safety choice

In the future, the **Blockchain** struct will be shared between threads, such as miner thread and network thread. So this struct must be thread safe. However, this is not hard to do with lock. **You don't need to worry about it in this part.** You can implement a non-thread-safe **Blockchain** and leave the thread safety problem to future parts.

## Grading

If your programming is working, you will pass the test named *insert_one*. (By running `cargo test`.)

We will use other private tests to grade your submission.
The tests will insert around 50 blocks into a new blockchain, and check whether the tip is correct. The tests contain forking/branching scenarios to check the correctness of your longest chain rule.

We will *NOT* call insert function with invalid blocks. Specifically, we will not insert a block whose parent is not already inserted.

## Double check
We provide (incomplete) auto-grader for you to test that your code format fits auto-grader. However, passing this auto-grader doesn't guarantee getting full grades. For this assignment, put your zip file with [autograder.sh](autograder.sh) and [add_test.py](add_test.py) in a new directory, from where run
```
bash autograder.sh
```
And you can open the output file _log.txt_, and see whether the auto-grader's tests are passed. You need to have `bash`, `unzip`, and `python3` to run this double check.

## Advance Notice
1. If you want to learn about thread safety of the Blockchain struct, you can try `Arc<Mutex<Blockchain>>` in your code.
2. Our goal is to decouple blockchain status from ledger status, and focus on the former. As a result, we don't involve transaction execution or ledger update or UTXO in this part. They will be handled in future parts.
3. We don't involve proof-of-work check or mining, but we need to prepare for them. So we require fields nonce and difficulty inside a block. You can start to think about how to mine or check blocks.
4. Blockchain struct will be used in multiple places in the future. For example, when you implement a miner, you insert a mined block into blockchain; when you want to mine on the longest chain, you need to get the tip as the block's parent; when you receive a block from p2p network, you insert it.
5. We don't require you to put a coin base transaction inside blocks in this part.

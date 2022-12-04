# Bitcoin Client Project, Part 3

In this part of the project, you will implement the **Block** struct and the **Blockchain** struct.

## Repository management and submission

1. Like the previous assignments, use GitHub and download the zip file. Rename it to your netid as `netid.zip`. Upload the zip file on canvas.
2. TAs will put additional tests (private) on the submission and run them to award marks.

## Code provided
We have provided started code in this repository. The following files are related to this assignment.
1. *src/types/block.rs* - Please finish the **Block** struct and some related functions in this file.
2. *src/blockchain/mod.rs* - Please finish the **Blockchain** struct and some related functions in this file. (You can also split codes into several files inside the directory *src/blockchain/*.)

## Programming

### Block

You need to define a **Block** similar to that in Bitcoin. We require that a block must include:
1. parent - A hash pointer to the parent block. Please use **H256** that we provide.
2. nonce - A random integer that will be used in proof-of-work mining. We suggest using **u32**.
3. difficulty - The mining difficulty, i.e., the threshold in the proof-of-work check. Please use **H256**: since we have provided a comparison function, with which you can write `if hash <= difficulty`. (Proof-of-work check or mining is not required in this part.)
4. timestamp - The timestamp at which this block is generated. (you can use `std::time`)
5. merkle\_root - the Merkle root of data (explained below in 6.).

The above fields are also known as **Header**. We suggest (but do not require) you create a struct **Header** to include them.

6. data/content - The actual transactions carried by this block. We suggest using a **Vec** of **SignedTransaction**. You have already written the SignedTransaction struct in a previous assignment.

We suggest (but do not require) you create a struct **Content** to include the content.

Notice that to create the Merkle root of **SignedTransaction**, you must implement the trait **Hashable** for **SignedTransaction**. This trait should be implemented by serializing it into bytes, then calling SHA256 to hash the bytes.

You need to implement the trait **Hashable** for **Block**. The way to hash a block is to hash **Header** rather than **Content**. So you can first implement **Hashable** for **Header**. When you hash a **Block**, you can directly call the hash function of **Header**. Please make sure you serialize the **Header** before hashing it.

To test and debug, you must implement the function `generate_random_block()`. This function takes the hash of the parent block as an argument. The generated block should contain that *parent*. The *nonce* should be a random integer. You can let the content be empty. So merkle\_root should be the Merkle root of an empty input (make sure this is accounted for in your Merkle implementation). As for fields such as difficulty and timestamp, choose whatever you like.

### Blockchain

You need to finish a struct named **Blockchain**, which contains the necessary information of a direct acyclic graph (DAG) and provides functions related to the longest chain rule. The following functions are required:
1. new() - Create a new blockchain that only contains the information of the genesis block. Define genesis block by yourself. 
2. insert() - insert a block into the blockchain. You can (but not required) make it return struct `Result` to enable error handling when an invalid block is inserted. (We will not deal with invalid blocks in this part)
3. tip() - Return the last block's hash in the longest chain. The tip should be computed in the new and insert functions; this should just return it.
4. all_blocks_in_longest_chain() - return all blocks' hashes in a vector from the **genesis to the tip**. This function will not be tested in this part and will be used in the future.

#### Storage choice

We suggest you use a **HashMap** in the standard crate to store blocks. You can use the hash as the key and the block as the value. This enables you to look up the blocks by hash very conveniently.

You can also store the tip and update it after inserting a block. If, say, your current tip is hash(B1), and you insert a new block B2. You need to update the tip to hash(B2) if and only if the length of chain B2 is *strictly greater* than that of B1.

You may also store the length/height of each block in the blockchain and use it to determine the longest chain. E.g., genensis block has height 0. This step is not required.

You can implement this with persistent storage, such as a database, but this is not the point of this project, and we suggest you use in-memory storage.

#### Thread safety choice

In the future, the **Blockchain** struct will be shared between threads, such as miner and network threads. So this struct must be thread-safe. However, this is not hard to do with lock. **You don't need to worry about it in this part.** You can implement a non-thread-safe **Blockchain** and leave the thread safety problem to future parts.

## Grading

If your program works, you will pass the test named *insert_one*. (By running `cargo test`.)

We will use other private tests to grade your submission. We will use the generate_random_block function that you implemented for tests.
The tests will insert around 50 blocks into a new blockchain and check whether the tip is correct. The tests contain forking/branching scenarios to check the correctness of your longest chain rule. We encourage you to write your own tests to verify the correctness and avoid losing points.

We will *NOT* call the insert function with invalid blocks. Specifically, we will not insert a block whose parent is not already inserted.

## Double check
We have provided an (incomplete) autograder. Same instructions as the previous parts.

## FAQ
- *Can the fields of Header/Content structs of the blockchain be made public?* 
    - Yes, they can be made public. You can also define a `get` function instead.
- *What values should the fields in the genesis block have?* 
    - Note that the fields (for the genesis block) such as nonce, difficulty, timestamp, parent should be fixed and not random. You can set nonce and timestamp to `0` and difficulty to `0xff..ff` and parent to `0x00..00` (or any other fixed values for that matter).
- *How does one set values to a variable of type H256?* 
    - You can create a `[u8;32]` with fixed values and convert it to H256 using `.into()`. Alternatively, you can use the `hex_literal` crate and use `.into()`.

## Advance Notice
1. If you want to learn about thread safety of the Blockchain struct, you can try `Arc<Mutex<Blockchain>>` in your code.
2. Our goal is to decouple blockchain status from ledger status and focus on the former. As a result, we don't involve transaction execution, ledger update, or UTXO in this part. They will be handled in future parts.
3. We don't use proof-of-work check or mining yet, but we must prepare for them. So we require the fields nonce and difficulty inside a block. You can start to think about how to mine or check blocks.
4. The Blockchain struct will be used in multiple places in the future. For example, when you implement a miner, you insert a mined block into the blockchain; when you want to mine on the longest chain, you need to get the tip as the block's parent; when you receive a block from p2p network, you insert it.
5. We don't require you to put a coin base transaction inside blocks in this part.

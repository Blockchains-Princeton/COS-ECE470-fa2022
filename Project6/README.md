# Bitcoin Client Project, Part 6

In this part of the project, we will combine the last 3 weeks' work to make a functioning data blockchain. Most of this week's work will combine mining, network, and blockchain modules. You must add PoW validation and a block buffer to handle orphan blocks.

## Repository management and submission

1. Like the previous assignments, use GitHub and download the zip file. Rename it to your netid as `netid.zip`. Upload the zip file on canvas.
2. TAs will use autograder to run your code to award marks.

## Programming

### Checks
Please add the following checks when processing a new block in *src/network/worker.rs*.

#### PoW validity check

Add code to check the PoW validity of a block by checking if:

1. PoW check: check if `block.hash() <= difficulty`. (Note that difficulty is a misnomer here since a higher 'difficulty' here means that the block is easier to mine).
2. Difficulty in the block header is consistent with your view. We have a fixed mining difficulty for this project; thus, this would just involve checking if the difficulty equals the parent block's difficulty. (This step should be done after parent check.)

#### Parent check

1. Check if the block's parent exists in your local copy of your blockchain; if the parent exists, add the block to your blockchain.
2. If this check fails, you must add the block to an 'orphan buffer'. You may need to create a struct for this.
3. If this check fails, also send **GetBlocks** message containing this parent hash. (This is the same as part 5 instructions.)

#### Orphan block handler

Check if the new processed block is a parent to any block in the orphan buffer; if that is the case, remove the block from the orphan buffer and process the block. This step should be done iteratively: insertion of a block A enables the processing of a former orphan B, insertion of B may allow another former orphan block C to be processed, and so on.

### Make sure modules of previous assignments work together

Make sure that blockchain, miner, and network modules work well together. If it is working, you will have a data blockchain. We call this blockchain a data blockchain since we are not adding any meaningful transactions or transaction validation at this stage yet. (If you like, you can put data into transactions, which will be carried by blocks and be on-chain eventually.)

The program can mine and communicate blocks and reach consensus on the blockchain. Here, consensus means that when multiple nodes are connected and running, they should have the same blocktree (including the longest blockchain and other blocks not in the longest chain) and keep the chain growing.

### API

We require an API named `/blockchain/longest-chain` to grade the program. It is already defined in this line in __src/api/mod.rs__
```
"/blockchain/longest-chain" => {
```

It should output an array of strings in the hex format of block hashes in the longest chain. The order of block hashes should be number-ordered. That is, block 0 (genesis), followed by block 1, block 2, etc. The output should be in JSON format, and here is an example of JSON format:

> ["0000000000000000000000000000000000000000000000000000000000000000","93b6a5b271bf03019da96d49506660dcdcad2376c3119c4cb9c47cb0f27fbbf1"]

Please ensure this API works and outputs the correct JSON format since it is crucial for auto-grading. You can run your program by `cargo run` or directly run the binary in `target`. Then you can call `http://127.0.0.1:7000/blockchain/longest-chain` in your browser or use a command like `curl` to check if it works.

## Grading

Now that we have a working blockchain program, we will auto-grade the program by running 3 nodes (processes) of it locally. Let's call them nodes A, B, and C. We will start nodes A, B, and C. We will connect node A to node B and node B to node C. Notice that nodes A and C are not connected.

For mining, we will start 3 nodes' miner with `lambda=0`. **You should choose a proper block difficulty in your code.** A suggestion is to have a _smaller_ difficulty so that the mining rate of blocks can be lower to reduce forking. (Below, we require >=10 blocks/minute, and you can have a larger value, e.g., 20 blocks/minute, to have higher confidence to meet our requirement.)

Let them run for 5 minutes. Then we will use the API to check the longest chains in them. The grading is related to the comparison between the three nodes.

1. Longest chain length: the min length of the three nodes. If it >=50, you get full grade for this item.
2. Length difference: the max length - the min length. If it <=3, you get full grade for this item.
3. Common prefix: these nodes should have the same longest chain, except for the few last blocks. If the number of different blocks at the end <=3, you get full grade for this item.

## Double check
We do not provide any script for this assignment. You can double-check by following these procedures (which will be our grading procedures):

1. Unzip your zip file by this command: `unzip -qq netid.zip -d netid`, make sure your code is in this directory: `netid/COS-ECE470-fa2022-main`.
2. Run `cargo build`, which generates `netid/COS-ECE470-fa2022-main/target/debug/bitcoin`. It is the runnable binary of your code. (Windows may have `*.exe`, and it's ok.)
3. Run three processes of this binary and remember to give different IP/ports to them. For example, use these 3 commands:
- `./bitcoin --p2p 127.0.0.1:6000 --api 127.0.0.1:7000`
- `./bitcoin --p2p 127.0.0.1:6001 --api 127.0.0.1:7001 -c 127.0.0.1:6000`
- `./bitcoin --p2p 127.0.0.1:6002 --api 127.0.0.1:7002 -c 127.0.0.1:6001`
4. Start mining by mining API, and let it run for 5 minutes. For example: (During grading we will use `lambda=0`)
- http://127.0.0.1:7000/miner/start?lambda=1000000
- http://127.0.0.1:7001/miner/start?lambda=1000000
- http://127.0.0.1:7002/miner/start?lambda=1000000

5. Use the `/blockchain/longest-chain` API to get the longest chain in 3 nodes
- http://127.0.0.1:7000/blockchain/longest-chain
- http://127.0.0.1:7001/blockchain/longest-chain
- http://127.0.0.1:7002/blockchain/longest-chain
6. Check whether the longest chains satisfy the aforementioned criteria.


## FAQ
- *How should difficulty be set?* 
     - During grading, the miner will be run with `lambda=0`. You should set a difficulty in the genesis block such that 10-20 blocks are mined per minute. Setting the difficulty to be too easy may result in excessive forking and the longest chains of the nodes not being in sync.
     - You can use the `hex_literal` crate to set the difficulty (Eg: `let difficulty = hex!("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").into()`)
- *Once started, how can one pause the miner?* 
     - You can use a very high value of `lambda` to pause mining, Eg: http://127.0.0.1:7000/miner/start?lambda=100000000000
- *The longest chains are not in sync, what might be the problem?*
    - You can put print statements at different points of your code (miner, network messages) to check if a node is getting stuck somewhere (maybe waiting for a `blockchain.lock()` to be released - check this by adding a print statement before and after locking statements). Make sure you are using `drop(blockchain)` if you are using a locked blockchain in your network worker or miner.  
     - The genesis block may have some randomness. It should be fully deterministic; this ensures all nodes start with the same genesis block.
     - The network may be jammed due to too many messages, causing a delay: You may be sending a lot of empty `NewBlockHashes` messages. The difficulty may be set too easy resulting in too many blocks being mined.
     - It may be easier to debug by starting with just two nodes with one of them mining. Decreasing the mining rate (by setting a larger lambda, or using a low value for `difficulty`) would also help debugging.
- *How should one structure the code for handling orphan blocks?* 
    - A simple way to do this is to initialize a orphan buffer HashMap before the `loop` starts in the worker. Instead of having a map from `hash` to `block`, it might be better to have a map from `parent hash` to `block`. In the `match` statement for `Message::Blocks`, check if the new processed block is a parent to any block in the orphan buffer. If that is the case, remove the block from the orphan buffer and process the block. This step should be done iteratively. I.e., once an orphan block is inserted, check if the orphan buffer has any of its children, and so on.
- *Is it okay if one fails the test cases of previous parts? (e.g. `reply_blocks_test`)* 
     - Yes, it is fine if you fail test cases of previous parts.

## Advance notice
1. In the next part, you will make the data meaningful, i.e., expressive for cryptocurrency operations.
2. In this project, we don't consider handling spamming attacks. The orphan buffer may be spammed by blocks from an adversary (not a big issue with real PoW), but we don't require you to solve this problem.

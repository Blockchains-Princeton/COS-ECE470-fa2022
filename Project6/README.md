# Bitcoin Client Project, Part 6

In this part of the project, we will combine last 3 weeks' work to make a functioning data blockchain. Most of this week's work will be combining mining, network and blockchain module. You will need to add PoW validation and a block buffer to handle orphan blocks.

## Repository management and submission

1. Similar to the previous assignments, use github and download zip file. Rename it to your netid as `netid.zip`. Upload the zip file on canvas.
2. TAs will use autograder to run your code to award marks.

## Programming

### Checks
When processing a new block in *src/network/worker.rs*, please add the following checks.

#### PoW validity check

Add code to check the PoW validity of a block by checking if:

1. PoW check: check if `block.hash() <= difficulty`. (Note that difficulty is a misnomer here since a higher 'difficulty' here means that the block is easier to mine).
2. Difficulty in the block header is consistent with your view. We have a fixed mining difficulty for this project, thus, this would just involve checking if difficulty equals the parent block's difficulty. (This step should be done after parent check.)

#### Parent check

1. Check if the block's parent exists in your local copy of your blockchain, if the parent exists, add the block to your blockchain.
2. If this check fails, you need to add the block in an 'orphan buffer'. You may need to create a struct for this.
3. If this check fails, also send **GetBlocks** message, containing this parent hash. (This is the same as part 3 instructs.)

#### Orphan block handler

Check if the new processed block is a parent to any block in the orphan buffer, if that is the case, remove the block from orphan buffer and process the block. This step should be done iteratively. I.e., a block makes a former orphan block be processed, and the latter makes another former orphan block be processed, and so on.

### Make sure modules of previous assignments work together

Make sure that blockchain, miner, network modules work well together. If it is working, you will have a data blockchain. We call this blockchain a data blockchain since we are not adding any meaningful transactions or transaction validation at this stage yet. (If you like, you can put data into transactions, who will be carried by blocks and be on-chain eventaully.)

The program can mine and communicate blocks and reach consensus on the blockchain. Here, consensus refers to that when multiple nodes are connected and running, they should have the same blocktree (including the longest blockchain and other blocks not in the longest chain) and keep the chain growing.

### API

To grade the program, we require an API named `/blockchain/longest-chain`. It is already defined in this line in __src/api/mod.rs__
```
"/blockchain/longest-chain" => {
```

It should output an array of strings that are the hex format of block hashes in the longest chain. The order of block hashes should be number-ordered. That is, block 0 (genesis), followed by block 1, block 2, etc. The output should be json format, and here is an example of json format:

> ["0000000000000000000000000000000000000000000000000000000000000000","93b6a5b271bf03019da96d49506660dcdcad2376c3119c4cb9c47cb0f27fbbf1"]

Please make sure this API works and outputs the correct json format, since it is crucial for auto-grading. You can run your program by `cargo run` or directly run the binary in `target`. Then you can call `http://127.0.0.1:7000/blockchain/longest-chain` in your browser or using command like `curl` to check if it works.

## Grading

Now that we have a working blockchain program, we will auto-grade the program by running 3 nodes (processes) of it locally. Let's call them node A, B, and C. We will start node A, B, and C. We will connect node A to node B, and node B to node C. Notice that node A and C is not connected.

For mining, we will start 3 nodes' miner with `lambda=0`. You should choose proper block difficulty in your code. A suggestion is to have a _smaller_ difficulty, so that the mining rate of blocks can be smaller to reduce forking. (Below we have a requirement for >=10 block/minute, and you can have a value larger than it, e.g., 20 block/minute, to have a higher confidence to meet our requirement.)

Let they run for 5 minutes. Then we will use API to check the longest chains in them. The grading is related to the comparison between three nodes.

1. Longest chain length: the min length of the three nodes. If it >=50, you get full grade for this item.
2. Length difference: the max length - the min length. If it <=3, you get full grade for this item.
3. Common prefix: these nodes should have the same longest chain, except for the few last blocks. If the last few different blocks length <=3, you get full grade for this item.

## Double check
We do not provide any script for this assignment. You can double check by following these procedures (which will be our grading procedures):

1. unzip your zip file by this command: `unzip -qq netid.zip -d netid`, make sure your code is in this directory: `netid/ece598pv-sp2022-main`.
2. run `cargo build`, which generates `netid/ece598pv-sp2022-main/target/debug/bitcoin`. It is the runnable binary of your code. (Windows may have `*.exe` and it's ok.)
3. run three processes of this binary and remember to give different ip/ports to them. For example, use these 3 commands:
- `./bitcoin --p2p 127.0.0.1:6000 --api 127.0.0.1:7000`
- `./bitcoin --p2p 127.0.0.1:6001 --api 127.0.0.1:7001 -c 127.0.0.1:6000`
- `./bitcoin --p2p 127.0.0.1:6002 --api 127.0.0.1:7002 -c 127.0.0.1:6001`
4. start mining by mining API, and let it run for 5 minutes.
5. use `/blockchain/longest-chain` API to get the longest chain in 3 nodes
6. check whether the longest chains satisfy aforementioned criteria.

## Advance notice
1. In the next part, you are going to make the data meaningful, i.e., expressive for cryptocurrency operations.
2. In this project, we don't consider handling spamming attacks. Orphan buffer may be spammed by blocks from an adversary (not a big issue with real PoW), but we don't require you to solve this problem.

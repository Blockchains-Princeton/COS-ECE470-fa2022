# Bitcoin Client Project, Part 5

In this part, you will implement the **network** module of the Bitcoin client. The network module is in charge of communicating with other nodes/clients. It forms the peer-to-peer (p2p) network and uses a gossip protocol to exchange data, including blocks and transactions. (Transactions will not be covered in this part.)

## Repository management and submission

1. Similar to the previous assignments, use GitHub and download zip file. Rename it to your netid as `netid.zip`. Upload the zip file on canvas.
2. TAs will put additional tests (private) on the submission and run them to award marks.

## Code provided
The following files are related to this assignment.
- *src/network/message.rs* - defines the types of messages.
- *src/network/worker.rs* - defines the behavior after receiving messages.

In other files of *src/network/*, we provide a multithread TCP socket server. The default number of threads is 4, and you can change it by parameter `--p2p-workers`. To see how the network server works, you can start two processes of your program by running these two commands respectively
```
cargo run -- -vvv --p2p 127.0.0.1:6000 --api 127.0.0.1:7000
cargo run -- -vvv --p2p 127.0.0.1:6001 --api 127.0.0.1:7001 -c 127.0.0.1:6000
```

`--p2p` parameter means that the first process will listen on 127.0.0.1:6000, and the second process will listen on 127.0.0.1:6001.

`-c` parameter means that the second process will try to connect to 127.0.0.1:6000, which is the address of the first process.

You can see this log on the first process, indicating that the first process accepts connection from the second process.
> New incoming connection from ...

We also provide an API to do ping/pong. You can run
`http://127.0.0.1:7000/network/ping` to send a ping message from the first process to the second process. You will also see a debug log about the ping/pong message.

Ping/pong messages are defined in *src/network/message.rs*, and the behavior after receiving messages is defined in *src/network/worker.rs*. Please read them since you are going to write your own messages.

Notice: the connection is bidirectional, so after process 2 connects to process 1, you don't need to make process 1 create another connection to process 2.

## Programming

You will finish the behavior when a few message types are received.

### Message types

You need to use these three message types. They have already been defined in *src/network/message.rs*.

1. NewBlockHashes(Vec\<H256\>)
2. GetBlocks(Vec\<H256\>)
3. Blocks(Vec\<Block\>)

### Gossip protocol

You need to define the gossip protocol, i.e., the behavior when messages are received, in *src/network/worker.rs*.

First, you need to add a thread-safe wrapper of Blockchain into **Worker** struct in *src/network/worker.rs*. It is similar to [previous part](../Project4). Notice that the server we provide is a multithread one, so please be careful with thread safety.

Then, you can define the gossip protocol as follows.
1. For **NewBlockHashes**, if the hashes are not already in the blockchain, you need to ask for them by sending **GetBlocks**.
2. For **GetBlocks**, if the hashes are in the blockchain, you can get these blocks and reply with a **Blocks** message.
3. For **Blocks**, insert the blocks into the blockchain if they are not already in it. You must also broadcast a **NewBlockHashes** message if the blocks are new to this node. **NewBlockHashes** message should contain hashes of blocks newly received.
4. Optional. If a block's parent is missing, put this block into a buffer and send **Getblocks** message. The buffer stores the blocks whose parent is not seen yet. When the parent is received, that block can be popped out from the buffer and inserted into the blockchain.

Hint: `peer.write()` and `self.server.broadcast()` may be useful to send a message. Also, make sure that the vectors of block hashes/blocks that you are sending on the channels should be non empty. Add a check that sends these messages only if the content of the vectors is non-empty.

### Combine with miner

When a miner successfully generates a new block, broadcast the message **NewBlockHashes**. Hint: in _src/miner/worker.rs_, `self.server.broadcast()` may be useful. Also, in `main.rs`, make sure you give the same thread-safe blockchain instance to both miner and network worker.

### Network for test
You need to write function `fn generate_test_worker_and_start() -> (TestMsgSender, ServerTestReceiver, Vec<H256>)` in *src/network/worker.rs* which creates structs for testing purpose. This function is called inside the auto-grader and should have no input parameter. We have provided a part of the function. You need to finish the part that adds **Blockchain** inside **Worker**, and return a vector of block hashes in the blockchain's longest chain (it could be just the genesis block hash).

## Grading

After you finish the programming, you will have a program that can connect to other peers and have the gossip protocol. You can run `cargo test reply_new_block_hashes` / `reply_get_blocks` / `reply_blocks` to test whether the gossip protocol is working.

We will auto-grade the program using tests similar to the ones mentioned above.

## Double check
We have provided an (incomplete) autograder. Same instructions as the previous parts.
Please ensure that `generate_test_worker_and_start()` does not admit any arguments. This function will be called by the autograder.

## FAQ
- *How does one obtain a vector of hashes of all blocks for the generate_worker_and_start method?* 
     - Use the `all_blocks_in_longest_chain` function you had defined in a previous assignment. Note that the order in which the hashes should appear in the output should be from `[genesis,..,tip]`.
- *Where should `peer.write()`  and where should `self.serve.broadcast()` be used?* 
     - `peer.write()` replies to only the peer from whom a message was received. This is useful in replying to targeted messages such as NewBlockHashes or GetBlocks. `self.server.broadcast()` sends message to all peers. This is useful to let all peers know you have added new blocks such as after mining or receiving new blocks from some peer.
- *How does one test this code with the miner code written in previous part?* 
     - You can run two processes connected to each other and start the miner in one of them (see the two `cargo run` commands given at the start of this file and use `http://127.0.0.1:7000/miner/start?lambda=1000000` to activate miner in the first node). Also print the number of blocks in the longest chain of each process and the block tip, or use `http://127.0.0.1:7000/blockchain/longest-chain` and `http://127.0.0.1:7001/blockchain/longest-chain` to see the blocks in longest chain. If your broadcast code for the miner is working correctly and the network code is also working correctly, you should see the same blocks in both chains at any given time (i.e. the blocks mined by one of them would be added to the other).
<!-- - *How should one structure the code for handling orphan blocks?* 
    - A simple way to do this is to initialize a orphan buffer HashMap before the `loop` starts in the worker. Instead of having a map from `hash` to `block`, it might be better to have a map from `parent hash` to `block`. In the `match` statement for `Message::Blocks`, check if the new processed block is a parent to any block in the orphan buffer. If that is the case, remove the block from the orphan buffer and process the block. This step should be done iteratively. I.e., once an orphan block is inserted, check if the orphan buffer has any of its children, and so on. -->

## Advance notice
1. When a block is received, it should be validated/checked first. We will cover this in the future.
2. Communication of transactions will be covered in the future.
3. The buffer is optional in this part and will be covered in the next part.

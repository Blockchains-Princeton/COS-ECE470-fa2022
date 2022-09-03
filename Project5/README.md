# Bitcoin Client Project, Part 5

In this part, you are going to implement the **network** module of Bitcoin client. The network module is in charge of communicating with other nodes/clients. It forms the peer-to-peer (p2p) network and uses gossip protocol to exchange data, including blocks and transactions. (Transactions will not be covered in this part.)

## Repository management and submission

1. Similar to the previous assignments, use github and download zip file. Rename it to your netid as `netid.zip`. Upload the zip file on canvas.
2. TAs will put additional tests (private) on the submission and run them to award marks.

## Code provided
The following files are related to this assignment.
- *src/network/message.rs* - defines the types of messages.
- *src/network/worker.rs* - defines the behavior after receiving messages.

In other files of *src/network/*, we provide a multithread tcp socket server. The default number of thread is 4 and you can change it by parameter `--p2p-workers`. To see how the network server works, you can start two processes of your program by running these two commands respectively
```
cargo run -- -vvv --p2p 127.0.0.1:6000 --api 127.0.0.1:7000
cargo run -- -vvv --p2p 127.0.0.1:6001 --api 127.0.0.1:7001 -c 127.0.0.1:6000
```

`--p2p` parameter means that the first process will listen on 127.0.0.1:6000 and the second process will listen on 127.0.0.1:6001.

`-c` parameter means that the second process will try to connect to 127.0.0.1:6000, which is the address of the first process.

On the first process, you can see this log, indicating that the first process accepts connection from the second process.
> New incoming connection from ...

We also provide an API to do ping/pong. You can run
`http://127.0.0.1:7000/network/ping` to send a ping message from the first process to the second process. You will also see a debug log about the ping/pong message.

Ping/pong messages are defined in *src/network/message.rs*, and the behavior after receiving messages are defined in *src/network/worker.rs*. Please read them since you are going to write your own messages.

Notice: the connection is bidirectional, so after process 2 connects to process 1, you don't need to make process 1 create another connection to process 2.

## Programming

You will finish the behavior when a few message types are received.

### Message types

You need to use these three message types. They are already defined in *src/network/message.rs*.

1. NewBlockHashes(Vec\<H256\>)
2. GetBlocks(Vec\<H256\>)
3. Blocks(Vec\<Block\>)

### Gossip protocol

You need to define the gossip protocol, i.e., the behavior when messages are received, in *src/network/worker.rs*.

First, you need to add thread safe wrapper of Blockchain into **Worker** struct in *src/network/worker.rs*. It is similar to [previous part](../MidtermProject2). Notice that the server we provide is a multithread one, so please be careful with thread safety.

Then, you can define the gossip protocol as follows.
1. For **NewBlockHashes**, if the hashes are not already in blockchain, you need to ask for them by sending **GetBlocks**.
2. For **GetBlocks**, if the hashes are in blockchain, you can get these blocks and reply them by **Blocks** message.
3. For **Blocks**, insert the blocks into blockchain if not already in it. Also if the blocks are new to this node, you need to make a broadcast of a **NewBlockHashes** message. **NewBlockHashes** message should contain hashes of blocks newly received.
4. Optional. If a block's parent is missing, put this block into a buffer and send **Getblocks** message. The buffer stores the blocks whose parent is not seen yet. When the parent is received, that block can be popped out from buffer and inserted into blockchain.

Hint: `peer.write()` and `self.server.broadcast()` may be useful to send a message.

### Combine with miner

When miner succesfully generates a new block, broadcast the message **NewBlockHashes**. Hint: in _src/miner/worker.rs_, `self.server.broadcast()` may be useful.

### Network for test
You need to write function `fn generate_test_worker_and_start() -> (TestMsgSender, ServerTestReceiver, Vec<H256>)` in *src/network/worker.rs* which creates structs for testing purpose. This function is called inside the auto-grader and should has no input parameter. We have provide a part of the function. You need to finish the part that add **Blockchain** inside **Worker**, and return a vector of block hashes that is in the blockchain (it could be just the genesis block hash).

## Grading

After you finish the programming, you will have a program that can connect to other peers nad have the gossip protocol. You can run `cargo test reply_new_block_hashes` / `reply_get_blocks` / `reply_blocks` to test whether the gossip protocol is working.

We will auto-grade the program using tests that are similar to the aforementioned tests.

## Double check
We provide (incomplete) auto-grader for you to test that your code format fits auto-grader. However, passing this auto-grader doesn't guarantee getting full grades. For this assignment, put your zip file with [autograder.sh](autograder.sh) and [add_test.py](add_test.py) in a new directory, from where run
```
bash autograder.sh
```
And you can open the output file _log.txt_, and see whether the auto-grader's tests are passed. You need to have `bash`, `unzip`, and `python3` to run this double check.

## Advance notice
1. When a block is received, it should be validated/checked first. We will cover this in the future.
2. Communication of transactions will be covered in the future.
3. The buffer is optional in this part, and will be covered in the next part.

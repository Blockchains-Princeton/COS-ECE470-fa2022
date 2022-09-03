# Bitcoin Client Project, Part 7

This part of the project will deal with including transactions in the codebase. Integrate the transaction structure inside the block content, add network functionality to transaction propagation and adding a transaction mempool to be used by the miner to include transaction content in the block being mined.

## Repository management and submission

1. Similar to the previous assignments, use github and download zip file. Rename it to your netid as `netid.zip`. Upload the zip file on canvas.
2. TAs will use autograder to run your code to award marks.

## Code provided
No additional code will be provided for this assignment. **Notice: starting from project part 6, we won't use tests to do autograding, so it is okay to fail previous tests.**

## Programming

### Transaction network messages

Add the following new messages and the procedures to process message (which are similar to those of block related messages):
1. NewTransactionHashes(Vec<H256>), similar to NewBlockHashes
2. GetTransactions(Vec<H256>), similar to GetBlocks
3. Transactions(Vec<Transaciton>), similar to Blocks

### Transaction format

You are free to choose any format for transaction structure. We recommend using a transaction structure that is either compatable with the UTXO model in Bitcoin or the account based model in Ethereum. 

- UTXO model transaction: input contains the hash of previous transaction and the index; output contains a recipient address and a value. It can support multiple inputs/outputs in a transaction. You can refer to [Bitcoin](https://en.bitcoin.it/wiki/Transaction) transaction but don't need to adopt its complex scripting language.
- Account based model transaction: it should contain a recipient address, a value, and a account-nonce. It only supports single sender and single receiver. This should be simpler to implement than UTXO model.

Now it's time to add transaction and its **Signature** to **SignedTransaction**. As introduced in previous assignment, crate *ring*'s signature may not be very convenient to use, so you can convert them to vector of bytes: `let signature_vector: Vec<u8> = signature.as_ref().to_vec();`.  You also need to add signer's public key to **SignedTransaction**.

Remeber to implement trait **Hashable** for **SignedTransaction**, you should already finish that since it is required by a previous assignment.

### Checks
When receiving and processing a new transaction in *src/network/worker.rs*, please add the following checks.

#### Transaction signature check

- Check if the transaction is signed correctly by the public key(s). 

- (Will not be tested or graded at this stage.) In UTXO model, also check the public key(s) matches the owner(s)'s address of these inputs. In account based model, check if the public key matches the owner's address of the withdrawing account.

#### Double spend checks

- (Will not be tested or graded at this stage.) In UTXO model, check if the inputs to the transactions are not double spent. In account based model, check if the balance is enough and the suggested nonce is equal to one plus the current account nonce.

#### Add those checks when processing blocks

When receiving and processing a block, also check transactions inside it.

### Transaction Mempool

Create a transaction **Mempool** structure to store all the received valid transactions which have not been included in the blockchain yet.
If a new transaction passes the above checks, add it to the mempool.
**Mempool** will also be used by miner to include transactions in the blocks being mined. The miner will add transactions in the mempool to the block till it reaches the block size limit (upper limit). You can choose the size limit by yourself (remember to meet the requirements in **Grading** section). There is no lower limit on transactions, i.e., it is possible that a block contains no transaction. On processing a new block (which is not an orphan or stale), remove corresponding transacitons from the mempool.

Similar to **Blockchain**, you need the thread safe wrapper `Arc<Mutex<>>`.

### Transaction generator

To demonstrate transaction is working well with the client, you need to add transactions into your running client. The transactions can be a simple payment in account based model, or a transaction with just one input and one output in UTXO model. You are free to choose the sender and recipient.

In order to do that, you need to write a transaction generator:
- create a new struct that starts a new thread and generate a transaction periodically,
- set a parameter named `theta` that controls the speed of that (theta=0 means fastest transaction generation, we will use theta=100 in grading)
- write an API in *src/api/* named `/tx-generator/start` and has parameter `theta`. This API should be very similar to `miner/start`.

When a transaction is generated, add the transactions to mempool and broadcast the hash to the network.

**Since you are not storing state (will be covered in the next part), you can create transactions with any random content.**

### API

To grade the program, we require an API named `/blockchain/longest-chain-tx`. It is already defined but unimplemented in this line in __src/api/mod.rs__
```
"/blockchain/longest-chain-tx" => {
```

It should output two layers of arrays of strings that are the hex format of (signed) transaction hashes in the longest chain. The output should be json format. For exmaple, if the longest chain has block 0, block 1, and block 2. And block 0 (genesis block) contains no transaction; block 1 contains transactions whose hashes are "0000000000000000000000000000000000000000000000000000000000000001", "0000000000000000000000000000000000000000000000000000000000000002" (the order of them matters); block 2 contains "000000000000000000000000000000000000000000000000000000000000000a". Then this example should output:

> [[], ["0000000000000000000000000000000000000000000000000000000000000001", "0000000000000000000000000000000000000000000000000000000000000002"], ["000000000000000000000000000000000000000000000000000000000000000a"]]

Notice that the API should not output all transactions in one array, because we would think it is one block containing all transactions.

Please write this API and outputs the correct json format, since it is crucial for auto-grading. You can run your program by `cargo run` or directly run the binary in `target`. Then you can call `http://127.0.0.1:7000/blockchain/longest-chain-tx` in your browser or using command like `curl` to check if it works.

## Grading

Similar to the previous assignment, we will auto-grade the program by running 3 nodes (processes) of it locally. Let's call them node A, B, and C. We will start node A, B, and C. We will connect node A to node B, and node B to node C. Notice that node A and C is not connected.

For mining, we will start 3 nodes' miner with `lambda=0`. You should choose proper block difficulty in your code.

For transaction generator, we will start 3 nodes' transaction generator with `theta=100`. You need to write transaction generator so that with this theta, it generates enough transaction to meet the following grading criteria.

Let they run for 5 minutes. Then we will use API to check the longest chain transactions in them. The grading is related to the comparison between three nodes.

1. Transaction throughput: the min count of transactions of the three nodes. If it >=500 (>=100 tx/min), you get full grade for this item.
2. Transactions per block: the average is taken over the blockchain, excluding the genesis block, then the min of three nodes is taken. If it >=10 and <=500, you get full grade for this item.
3. No duplicate transactions: there should be no duplicate transactions in the blockchain. However, we relax the grading of this item. If unique transaction count divided by total transaction count >=0.9 for every node, you get full grade for this item.
4. Common prefix: since it is already graded in the previous one, we relax the grading of this item. If the first transaction inside the second block of the three nodes are the same, you get full grade for this item. We use the second block since we exclude the genesis block.

## Double check
We do not provide any script for this assignment. You can double check by following these procedures (which will be our grading procedures):

1. unzip your zip file by this command: `unzip -qq netid.zip -d netid`, make sure your code is in this directory: `netid/COS-ECE470-fa2022-main`.
2. run `cargo build`, which generates `netid/COS-ECE470-fa2022-main/target/debug/bitcoin`. It is the runnable binary of your code. (Windows may have `*.exe` and it's ok.)
3. run three processes of this binary and remember to give different ip/ports to them. For example, use these 3 commands:
- `./bitcoin --p2p 127.0.0.1:6000 --api 127.0.0.1:7000`
- `./bitcoin --p2p 127.0.0.1:6001 --api 127.0.0.1:7001 -c 127.0.0.1:6000`
- `./bitcoin --p2p 127.0.0.1:6002 --api 127.0.0.1:7002 -c 127.0.0.1:6001`
4. start mining by mining API, tx-generator by its API (theta=100), and let it run for 5 minutes.
5. use `/blockchain/longest-chain-tx` API to get the longest chain transactions in 3 nodes
6. check whether they satisfy aforementioned criteria.

## Advance notice
1. In the next part, we will need to add state validity to the transaction which corresponds to the double spend checks.
2. We will do ICO in the next part.

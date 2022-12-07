# Bitcoin Client Project, Part 8

This is the last part of the project, and you will finish the Bitcoin client. You need to maintain a state for the ledger that the blockchain creates and add all the necessary checks related to it. 

## Repository management and submission

1. Like the previous assignments, use GitHub and download the zip file. Rename it to your netid as `netid.zip`. Upload the zip file on canvas.
2. TAs will use autograder to run your code to award marks.

## Code provided
No additional code will be provided for this assignment. **Notice: starting from project part 6, we won't use tests to do autograding, so it is okay to fail previous tests.**

## Programming

### Transaction Checks
In the last part, you included transactions into blocks. To prevent misbehavior such as double spending, you need to add the following checks:

#### Transaction signature check
- Check if the transaction is signed correctly by the public key(s).
- In the UTXO model, check if the public key(s) matches the owner(s)'s address of these inputs. (This step needs struct **State**, see below.)
- In the account-based model, check if the public key matches the owner's address of the withdrawing account. (This step needs struct **State**, see below.)

#### Spending check
- In the UTXO model, check if the inputs to the transactions are not spent, i.e. exist in **State** (see below). Also, check that the values of inputs are not less than those of outputs.
- In the account-based model, check if the balance is enough and if the suggested account nonce equals one plus the account nonce. This check also needs **State** (see below).

You should also consider cases where each individual transaction may be valid; but cannot be valid together in the same block.
### State

Ledger state, or **State**, is a collection of all the required information to check transactions.  

- In the UTXO model, **State** should contain all the unspent transaction outputs. The format of an unspent transaction output may contain *(transaction hash, output index, value, recipient)*. Output index refers to the index in transactions (remember transactions are multi-output.) Recipient refers to the recipient address of that output and is used as the owner of that unspent transaction output.
- In the account-based model, **State** should contain all the accounts' information. It may have *(account address, account nonce, balance)*.

To access data conveniently, **we recommend using a HashMap to store State**. In the UTXO model, we recommend `HashMap<(transaction hash, output index), (value, recipient)>`. In the account-based model, we recommend `HashMap<account address, (account nonce, balance)>`.

#### State update
When executing a block, i.e., executing transactions in that block, we need to update the state.
- In the UTXO model, remove those *inputs*, and add *outputs* to the state.
- In the account-based model, change the accounts' nonce and balance. Create new accounts if required.

#### Initial state (ICO)
You can do an initial coin offering (ICO) by inserting an entry into **State** struct. **The grading section requires the ICO to insert exactly one entry.**
- In the UTXO model, add unspent transaction outputs and specify the recipients to be the addresses you control.
- In the account-based model, create accounts whose addresses are under your control.

#### State per block
Since there is branching/forking in the blockchain, and the longest chain may change, you need to store one copy of **State** for each block. A copy of **State** for a block refers to the state after executing the block. We recommend using HashMap-like storage, e.g., `HashMap<block hash, state>`. When you check transactions, you can get the corresponding state from it. When you update the state, you do the update on a new state copy, and insert it.

Another way to deal with forking is to implement a reverse state transition corresponding to a transaction. Say the longest chain changes from A->B->C->D to A->B->E->F->G, you can perform reverse state transition on blocks D and C and a forward state transition from blocks E, F, G. This method is more complex than the previous one.

### Transaction generator
The transaction generator should generate transactions that pass the checks. It can read the blockchain and the state to ensure this. On different nodes/processes, the transaction generator should control different key pairs.

### Transaction Mempool update
After implementing the state transition, ensure that the transactions in the mempool are valid with respect to the new state; this is necessary since some transactions may classify as double-spends after the state update, and you may need to remove those transactions.

### API

To grade the program, we require an API named `/blockchain/state?block=`.

It should output a representation of the state at a certain block in the longest chain. The output should be a JSON format array. For example, `/blockchain/state?block=10` should output the state at block 10 in the longest chain (suppose the chain is longer than 10). The state representation should be different for UTXO and account-based models.

- UTXO model: all the unspent transaction output entries should be in an array, like the following example that has three entries:

<!-- > ["UTXO1","UTXO2","UTXO3"] -->
`["(ef307355202461e57e08b40c0c024440b468518ef9b4a8770b288a75a94e3f8b, 2, 5, 71fa98fadbe1da07384165aa31eb069044060209)","(ef307355202461e57e08b40c0c024440b468518ef9b4a8770b288a75a94e3f8b, 1, 234, 896e9556ea1dc78ca55c236d6f6a5f81bff13dfd)","(9f1c534efb06233235a89a35f264aaf83ed2b991c7b79e114600c39021622bad, 0, 8645, 71fa98fadbe1da07384165aa31eb069044060209)",]`
<!-- ("UTXO1" is a placeholder for your entry representation.) And make sure to include "transaction hash, output index, value, recipient" 4-tuple in the unspent transaction output entry representation. This 4-tuple should be converted to a **single string**. -->

Each UTXO representation should include "transaction hash, output index, value, recipient" 4-tuple.

- Account-based model: all accounts' information should be in an array, like the following example that has three entries:

> `["(d5025bb3b5085be913b0778c82f5c73aa831ed2c, 1, 985035)","(a0d741628fc826e09475d341a780acde3c4b8070, 2, 14965)", "(4r5tghb6b608hbttyb50778c8255cr56a8315t67, 2, 13542)"]`

<!-- ("Account1" is a placeholder for your entry representation.) And make sure to include "address, account nonce, balance" 3-tuple in the account information representation. This 3-tuple should be converted to a **single string**.  -->
Each account information representation should include "address, account nonce, balance" 3-tuple.

Please refer to the code of parsing `lambda` for miner API, and use a similar logic to parse the parameter `?block=10`.  Please ensure this API outputs the correct JSON format, since it is crucial for auto-grading. You can run your program by `cargo run` or directly run the binary in `target`. Then you can call http://127.0.0.1:7000/blockchain/state?block=10 or another number in your browser or use a command like `curl` to check if it works.


## Conclusion

Now that you have finished the last part, you have a simplified Bitcoin client! With a transaction generator simulating users' transactions, the system should run smoothly and securely.

## Grading

(Grading setting is the same as the previous assignment.) We will auto-grade the program by running 3 nodes (processes) of it locally. Let's call them nodes A, B, and C. We will start nodes A, B, and C. We will connect node A to node B and node B to node C. Notice that nodes A and C are not connected.

For ICO, we require only one UTXO entry (UTXO model) or only one account (account-based model). As the state evolves, new UTXO entries or accounts should be created.

For mining, we will start 3 nodes' miner with `lambda=0`. You should choose a proper block difficulty in your code.

For transaction generator, we will start 3 nodes' transaction generator with `theta=100`. You need to write a transaction generator so that with this theta, it generates enough transactions to meet the following grading criteria.

Let them run for 5 minutes. Then we will use API to check the states in them. We will use `/blockchain/state?block=0` (the genesis state at ICO), `/blockchain/state?block=10` and `/blockchain/state?block=20`. The grading is related to the comparison between three nodes.

1. The initial state after ICO should contain <=3 entries.
2. The state at 10 and 20 should contain >=3 entries. (UTXO model: at least 3 UTXO entries; account-based model: at least 3 account informations.)
3. State should evolve: states at 0, 10, and 20 should not be exactly the same.
4. Common prefix: since it is already graded in the previous one, we relax the grading of this item. The state at block 10 should be the same across 3 nodes. Note that your chain should already be much longer than 10 blocks.

## Double check
We do not provide any script for this assignment. You can double-check by following these procedures (which will be our grading procedures):

1. Unzip your zip file by this command: `unzip -qq netid.zip -d netid`, make sure your code is in this directory: `netid/COS-ECE470-fa2022-main`.
2. Run `cargo build`, which generates `netid/COS-ECE470-fa2022-main/target/debug/bitcoin`. It is the runnable binary of your code. (Windows may have `*.exe`, and it's ok.)
3. Run three processes of this binary and remember to give different ip/ports to them. For example, use these 3 commands:
- `./bitcoin --p2p 127.0.0.1:6000 --api 127.0.0.1:7000`
- `./bitcoin --p2p 127.0.0.1:6001 --api 127.0.0.1:7001 -c 127.0.0.1:6000`
- `./bitcoin --p2p 127.0.0.1:6002 --api 127.0.0.1:7002 -c 127.0.0.1:6001`
4. Start mining by mining API, tx-generator by its API (theta=100), and let it run for 5 minutes.
5. Use `/blockchain/state` API to get the states in 3 nodes
6. Check whether they satisfy the aforementioned criteria.

## FAQ

- *How can one set up initial addresses for each node?* 
     - In the ICO, you can set addresses for each node according to deterministic keypairs using random seeds (eg: using `Ed25519KeyPair::from_seed_unchecked(&[random_seed;32]).unwrap()`)
- *How can one pass the keypairs of addresses defined in the genesis to the transaction generators?* 
     - You could pass the respective initial keypairs to the transaction generators according to the `p2p_addr`.
- *Is it an issue if the ordering of entries in the state api list is different across nodes?* 
     - No, we will sort the entries beforee comparison during grading.
- *If the blockchains were in sync for the Part 7 but diverge in this part after the introduction of State, what might be the problem?* 
    - You can put print statements within the new code you added to see if a node is getting stuck somewhere (maybe waiting for a blockchain.lock() to be released). Make sure you are using `drop(mempool)` or `drop(blockchain)` if you are using a locked mempool or blockchain in your network worker, miner or transaction generator.
- *How can one increase the number of valid transactions in a block?* 
    - You can generate new addresses/accounts controlled by a node within the transaction generator (and create a transaction sending coins to this new account) 


#### Note
We do not ask you to implement transaction fees, mining rewards, and the corresponding coinbase transaction for this project.
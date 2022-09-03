# Bitcoin Client Project, Part 8

This is the last part of the project, and you are going to finish the Bitcoin client. You need to maintain a state for the ledger that the blockchain creates and add all the necessary checks related to it. 

## Repository management and submission

1. Similar to the previous assignments, use github and download zip file. Rename it to your netid as `netid.zip`. Upload the zip file on canvas.
2. TAs will use autograder to run your code to award marks.

## Code provided
No additional code will be provided for this assignment. **Notice: starting from project part 6, we won't use tests to do autograding, so it is okay to fail previous tests.**

## Programming

### Transaction Checks
In last part, you include transactions into blocks. In order to prevent misbehavior such as double spending, you need to add the following checks:

#### Transaction signature check
- Check if the transaction is signed correctly by the public key(s).
- In UTXO model, check the public key(s) matches the owner(s)'s address of these inputs. (This step needs struct **State**, see below.)
- In account based model, check if the public key matches the owner's address of the withdrawing account. (This step needs struct **State**, see below.)

#### Spending check
- In UTXO model, check if the inputs to the transactions are not spent, i.e. exist in **State** (see below). Also check the values of inputs are not less than those of outputs.
- In account based model, check if the balance is enough and the suggested account nonce is equal to one plus the account nonce. This check also needs **State** (see below).

### State

Ledger state, or **State**, is a collection of all the required information to check transactions.  

- In UTXO model, **State** should contain all the unspent transaction outputs. The format of an unspent transaction output may contain *(transaction hash, output index, value, recipient)*. Output index refers to the index in transactions (remember transactions are multi-output.) Recipient refers to the recipient address of that output, and is used as the owner of that unspent transaction output.
- In account based model, **State** should contain all the accounts' information. It may contain *(account address, account nonce, balance)*.

To access data conveniently, we recommend use HashMap to store State. In UTXO model, we recommend `HashMap<(transaction hash, output index), (value, recipient)>`. In account based model, we recommend `HashMap<account address, (account nonce, balance)>`.

#### State update
When executing a block, i.e., executing transactions in that block, we need to update the state.
- In UTXO model, remove those *inputs*, and add *outputs* to the state.
- In account based model, change account's nonce and balance. Create new accounts if you need.

#### Initial state (ICO)
You can do initial coin offering (ICO) by inserting an entry into **State** struct. **In the grading section, we require the ICO to insert exactly one entry.**
- In UTXO model, add unspent transaction outputs and specify the recipients to be the addresses you control.
- In account based model, create accounts whose addresses are under your control.

#### State per block
Since there is branching/forking in the blockchain, and the longest chain may change, you need to store one copy of **State** for each block. A copy of **State** for a block refers to the state after executing the block. We recommend using a HashMap-like storage, e.g., `HashMap<block hash, state>`. When you check transactions, you can get the corresponding state from it. When you update state, you do the update on a new state copy, and insert it.

Another way to deal with forking is to implement a reverse state transition corresponding to a transction, say that the longest chain changes from A->B->C->D to A->B->E->F->G, you can perform reverse state transition on blocks D and C and a forward state transition from blocks E, F, G. This way is more complex than the previous one.

### Transaction generator
Transaction generator should generate transactions that pass the checks. It can read the blockchain and the state to ensure that. On different nodes/processes, transaction generator should control different key pairs.

### Transaction Mempool update
After implementing state transition, ensure that the transactions in the mempool are valid with respect to the new state, this is necessary since some transactions may classify as double-spends after the state update, you may need to remove those transactions.

### API

To grade the program, we require an API named `/blockchain/state?block=`.

It should output a representation of the state at a certain block in the longest chain. The output should be a json format array. For exmaple, `/blockchain/state?block=10` should output the state at block 10 in the longest chain (suppose the chain is longer than 10). The state representation should be different for UTXO and account based model.

- UTXO model: all the unspent transaction output entries should be in an array, like the following example that has three entries:

> ["UTXO1","UTXO2","UTXO3"]

("UTXO1" is a placeholder for your entry representation.) And make sure to include "transaction hash, output index, value, recipient" 4-tuple in the unspent transaction output entry representation.

- Account-based model: all the account's information should be in an array, like the following example that has three entries:

> ["Account1","Account2","Account3"]

("Account1" is a placeholder for your entry representation.) And make sure to include "address, account nonce, balance" 3-tuple in the account information representation.

On how to parse the parameter `?block=10` you can refer to the code of parsing `lambda` for miner API. Please write this API and outputs the correct json format, since it is crucial for auto-grading. You can run your program by `cargo run` or directly run the binary in `target`. Then you can call `http://127.0.0.1:7000/blockchain/state?block=10` or other number in your browser or using command like `curl` to check if it works.


## Conclusion

Now that you finish the last part, you have a simplified Bitcoin client! With transaction generator simulating user's transactions, the system should run smoothly and securely.

## Grading

(Grading setting is the same as the previous assignment.) We will auto-grade the program by running 3 nodes (processes) of it locally. Let's call them node A, B, and C. We will start node A, B, and C. We will connect node A to node B, and node B to node C. Notice that node A and C is not connected.

For ICO, we require only one UTXO entry (UTXO model) or only one account (account-based model). As the state evolves, new UTXO entries or accounts should be created.

For mining, we will start 3 nodes' miner with `lambda=0`. You should choose proper block difficulty in your code.

For transaction generator, we will start 3 nodes' transaction generator with `theta=100`. You need to write transaction generator so that with this theta, it generates enough transaction to meet the following grading criteria.

Let they run for 5 minutes. Then we will use API to check the states in them. We will use `/blockchain/state?block=0` (the state after ICO), `/blockchain/state?block=10` and `/blockchain/state?block=20`. The grading is related to the comparison between three nodes.

1. The initial state after ICO should contain only 1 entry.
2. The state at 10 and 20 should contain >=3 entries. (UTXO model: at least 3 UTXO entries; account-based model: at least 3 account informations.)
3. State should evolve: state at 0, at 10 and at 20 should not be exactly the same.
4. Common prefix: since it is already graded in the previous one, we relax the grading of this item. The state at block 10 should be the same across 3 nodes. Note that your chain should already be much longer than 10 blocks.

## Double check
We do not provide any script for this assignment. You can double check by following these procedures (which will be our grading procedures):

1. unzip your zip file by this command: `unzip -qq netid.zip -d netid`, make sure your code is in this directory: `netid/ece598pv-sp2022-main`.
2. run `cargo build`, which generates `netid/ece598pv-sp2022-main/target/debug/bitcoin`. It is the runnable binary of your code. (Windows may have `*.exe` and it's ok.)
3. run three processes of this binary and remember to give different ip/ports to them. For example, use these 3 commands:
- `./bitcoin --p2p 127.0.0.1:6000 --api 127.0.0.1:7000`
- `./bitcoin --p2p 127.0.0.1:6001 --api 127.0.0.1:7001 -c 127.0.0.1:6000`
- `./bitcoin --p2p 127.0.0.1:6002 --api 127.0.0.1:7002 -c 127.0.0.1:6001`
4. start mining by mining API, tx-generator by its API (theta=100), and let it run for 5 minutes.
5. use `/blockchain/state` API to get the states in 3 nodes
6. check whether they satisfy aforementioned criteria.

#### Note
We do not ask you to implement transaction fees and mining rewards and the corresponding coinbase transactoin for this project.
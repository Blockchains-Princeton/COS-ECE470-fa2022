# Bitcoin Client Project, Part 2
In this assignment, you will implement some crypto-primitives and basic data structures. You will need the code that we provide in this repo. Please follow the instructions.

## Repository management and submission:
1. Similar to the previous assignment, you can continue to work on your github repo. After pushing to your github repo, click `Code -> Download ZIP' on github to download a zip file.
2. Rename it to your netid as `netid.zip`. Upload the zip file on canvas. Please check your file size and it should be less than 1MB or 2MB.
3. TAs will put additional tests (private) on the submission and run them to award marks.

## Code provided
We have provided incomplete code for implementing some crypto-primitives and data structures like merkle tree. The following files are related to this assignment and you should read them.
1. _src/types/hash.rs_ - Provides __H256__ struct(32 byte array),  __Hashable__ trait, with its implementation for H256. (You don't need to write anything in this file.)

2. _src/types/merkle.rs_ - struct defition of **MerkleTree** struct and the related function declaration

You will write your code in this file.

As for other files in the repo, you don't have to worry about them in this assignment. They may appear in future assignments/projects.

## Programming
You need to implement the missing parts in the code. They include the following.

### Merkle Tree
This part is in file *src/types/merkle.rs*. You need to complete the merkle tree struct and some functions. You can find a good article about it [here](https://nakamoto.com/merkle-trees/). Specifically, the functions you need to implement are:
1. *new()* - this function takes a slice of Hashable data as input, and create the merkle tree. 
2. *root()* - given a merkle tree, return the root. The computation of the root is inside *new()*, this function should just return the root.
3. *proof()* - given a merkle tree, and also given the index, this function returns the proof in the form of a vector of hashes.
4. *verify()* - given a root, a hash of datum, a proof (a vector of hashes), an index of that datum (same index in *proof()* function), and a leaf_size (the length of leaves/data in *new()* function), returns whether the proof is correct.

We provide some small test functions in this file and you can run `cargo test`. In these test functions, we also provide a brief explanation about the expected computation.

*new()* function can take any Hashable data, but for simpilicity we will test merkle tree over **H256**, whose Hashable trait is already provided inside *src/types/hash.rs*.

A tricky part about *new()* is when the input length is not a power of 2, you will need some more steps to create the merkle tree as follows.
> Whenever a level of the tree has odd number of nodes, duplicate the last node to make the number even.

## Grading

After you finish the programming, you can run `cargo test merkle_root` / `merkle_proof` / `merkle_verifying` to test whether your implementation is working.

We will auto-grade the program using tests that are similar to the aforementioned tests. For merkle tree tests, we will use an input of length 8.

## Double check
We provide (incomplete) auto-grader for you to test that your code format fits auto-grader. However, passing this auto-grader doesn't guarantee getting full grades. For this assignment, put your zip file with [autograder.sh](autograder.sh) and [add_test.py](add_test.py) in a new directory, from where run
```
bash autograder.sh
```
And you can open the output file _log.txt_, and see whether the auto-grader's tests are passed. You need to have `bash`, `unzip`, and `python3` to run this double check.
## Double check (windows)
Same as Part 1.
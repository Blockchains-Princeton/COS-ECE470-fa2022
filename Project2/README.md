# Bitcoin Client Project, Part 2
In this assignment, you will implement some crypto-primitives and basic data structures. You will need the code that we provide in this repo. Please follow the instructions.

## Repository management and submission:
1. Similar to the previous assignment, you can continue to work on your GitHub repo. After pushing to your GitHub repo, click `Code -> Download ZIP' to download a zip file.
2. Rename it to your netid as `netid.zip`. Upload the zip file on canvas. Please ensure that the file size <2MB.
3. TAs will put additional tests (private) on the submission and run them to award marks.

## Code provided
We have provided incomplete code for implementing some crypto-primitives and data structures like merkle tree. The following files are related to this assignment, and you should read them.
1. _src/types/hash.rs_ - Provides __H256__ struct(32 byte array),  __Hashable__ trait, with its implementation for H256. (You don't need to write anything in this file.)

2. _src/types/merkle.rs_ - struct definition of **MerkleTree** struct and the related function declaration. You will write your code in this file.

The other files in the repo are not relevant to this assignment. They may appear in future assignments/projects.

## Programming
You need to implement the missing parts in the code. They include the following.

### Merkle Tree
This part is in file *src/types/merkle.rs*. You need to complete the Merkle tree struct and some functions. You can find a good article about it [here](https://nakamoto.com/merkle-trees/). Specifically, the functions you need to implement are:
1. *new()* - This function takes a slice of Hashable data as input and creates the Merkle tree. 
2. *root()* - given a Merkle tree, return the root. The root should be computed in *new()*; this function should just return it.
3. *proof()* - given a Merkle tree, and an index (starts from 0), this function returns the proof in the form of a vector of hashes (The proof should not include the leaf and root).
4. *verify()* - given a root, a hash of datum, a proof (a vector of hashes), an index of that datum (same index in *proof()* function), and a leaf_size (the length of leaves/data in *new()* function), returns whether the proof is correct.

We have provided a few simple test functions in this file, and you can run `cargo test`. In these test functions, we also briefly explain the expected computation.

*new()* function can take any Hashable data, but for simplicity, we will test the Merkle tree over **H256**, whose Hashable trait is already provided inside *src/types/hash.rs*.

A tricky part about *new()* is when the input length is not a power of 2, you will need some more steps to create the Merkle tree as follows.
> Whenever a tree level has an odd number of nodes, duplicate the last node to make the number even.

## Grading

After you finish the programming, you can run `cargo test merkle_root` / `merkle_proof` / `merkle_verifying` to test whether your implementation is working.

We will auto-grade the program using tests similar to the ones mentioned above. We will not test edge cases. We encourage you to write your own test cases to ensure that your implementation is correct.

## Double check
We have provided an (incomplete) auto-grader to test that your code format fits the auto-grader. However, passing this auto-grader doesn't guarantee to get full grades. For this assignment, put your netid.zip file with [autograder.sh](autograder.sh) and [add_test.py](add_test.py) in a new directory, from where run
```
bash autograder.sh
```
You can open the output file _log.txt_and see whether your code passes the auto-grader tests. Requirements to run the double-check: `bash`, `unzip`, `python3`.

If you see "Code format wrong" on the screen, it may be due to changing these lines in the code: `// DO NOT CHANGE THIS COMMENT, IT IS FOR AUTOGRADER.`

If in _log.txt_ you cannot see the correct log; your zip file may have incorrect directories for the auto-grader to compile.

## Double check (windows)
Similar to the Unix double-checking auto-grader, we have provided `autograder_windows.bat`. First, you need to unzip your netid.zip file manually and put the `COS-ECE470-fa2022-main` folder inside a folder named after your netid. Put the folder `your-netid` in the a new folder containing [autograder.sh](autograder.sh) and [add_test.py](add_test.py). Make sure that `Cargo.toml` and `src/` is in `your-netid/COS-ECE470-fa2022-main`. Then double click `autograder_windows.bat`, enter your netid as instructed, and the result will be shown in a cmd window.

You need to install `python3` to run it.

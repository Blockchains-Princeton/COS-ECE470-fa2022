# Bitcoin Client Project, Part 1

Hi, welcome! This is your first assignment in this course. This assignment aims to help you familiarize yourself with the **Rust** programming language. We will use Rust throughout this course, so it is a good idea to start learning it as early as possible.

## Introduction

We expect you to have some programming experience and are familiar with at least one programming language. If you don't know Rust language, it is totally okay since in this assignment, you'll self-teach Rust language by reading the documents of:

- the Rust language itself;
- Cargo, the rust package manager, and building tool;
- Rust standard libraries;
- Rust crates.

Then you will finish simple tasks in the codebase we provide. We divide the project into several sub-projects. This is the first part of the project.
If you are already familiar with Rust, this simple assignment will take less than 30 minutes!

In this course project, you will build a simplified Bitcoin client. The client's goal is not to run in the Bitcoin mainnet or any public testnet. Instead, the goal is to run it inside your team and let you have fun with it. You have plenty of freedom in designing and implementing this project.

## Reading 
Please refer to [Rust by example](https://doc.rust-lang.org/rust-by-example/) to learn Rust grammar.

Please refer to [https://doc.rust-lang.org/cargo/](https://doc.rust-lang.org/cargo/) to learn Cargo, the Rust package manager, and building tool. After reading chapter 1, you'll be able to install Rust and Cargo and run a Rust project.

For [Rust standard crate](https://doc.rust-lang.org/stable/std/), we recommend you learn two very important structs: **String** and **Vec**.

You can learn about other public crates here: [https://docs.rs/](https://docs.rs/). A *crate* just means a library or a package and can be managed by Cargo. You will learn how to use the following crate:
- [ring](https://docs.rs/ring/0.16.20/ring/), a cryptographic crate. Specifically, you need to learn how to do SHA256 hash.

For these crates, their GitHub page or homepage may also be helpful. Feel free to read them.

## Repository management and submission:
1. This repo provides the codebase for assignments. Please make a new **private** repo, e.g. hebbarashwin/COS-ECE470-fa2022 (Please do not change the name of the repo). Import code from https://github.com/Blockchains-Princeton/COS-ECE470-fa2022.git.
2. You can run tests (by command `cargo test`) provided in the code to check the validity of their implementation. However, passing these tests doesn't guarantee to get full grades. 
3. Push to your GitHub repo's **main** branch (Please use `gitignore` file to avoid pushing unnecessary files! Specifically, the `target` directory should not be pushed), and click `Code -> Download ZIP' on GitHub to download a zip file. **Avoid zipping your code on your computer since the directories or files on your computer may cause an error for auto-grading.**
4. Before submitting your code, you can double-check by running the auto-grading script we provide to make sure we can auto-grade your code. (Details below.) 
5. Rename it to your netid as `netid.zip`. Upload the zip file on canvas. Please ensure that the file size <2MB.
6. TAs will put additional tests (private) to the auto-grader and run them to award marks.

## Code provided
We have provided incomplete code for implementing some crypto-primitives. The following files are related to this assignment.

_src/types/address.rs_ - Provides __Address__ struct (20 byte array).

_src/types/transaction.rs_ - struct defition of **Transaction** struct and function declaration for __sign()__ and __verify()__ .

As for other files in the repo, you don't have to worry about them in this assignment. They may appear in future assignments/projects.

## Programming
After you fork this repo, we first suggest running the command `cargo test` to see whether the code is compiling on your machine. (If compilation has errors, please check if you are running the latest stable version of Cargo.) If the compiling is successful, you will see something like this:
```
running X tests
test XXX ... FAILED
test XXX ... FAILED
```
It's expected that tests fail with the code we provide. After you finish this assignment, some of the tests will pass. We encourage you to add your own tests to your code as well.

These are the tasks of this assignment:

1. You need to implement the missing parts in file _src/types/address.rs_:

- `fn from_public_key_bytes(bytes: &[u8])`

- It uses SHA256 (from **ring** crate (version >= 0.16.20)) to hash the input bytes, and takes the last 20 bytes, and converts them into a __Address__ struct. The code now contains `unimplemented!()`, and you can delete it and write your own code.

- We provide a small test function named **from_a_test_key()**. After you finish coding, you can run `cargo test from_a_test_key` and see this function's result in the output. It will look like the following.
```
test types::address::test::from_a_test_key ... ok
```
- To test your code, you are free to write more tests.

2. The missing parts in file _src/types/transaction.rs_: 

- Fill in the **Transaction** struct. We don’t expect the cryptocurrency and payment to be functional at this point, so you can put any content in transactions. A simple choice is to put **sender**, **receiver**, and **value** inside transactions. **sender**, **receiver** are of type **Address** and **value** is integer.
- Fill in the `sign` and `verify` functions. These two functions should sign and verify the digital signature of the **Transaction** struct. Please use **ring** (version >= 0.16.20) crate. You can use the [bincode](https://docs.rs/bincode/latest/bincode/) crate to serialize and deserialize any struct. The code we provide contains some `unimplemented!()` and you can delete it and write your own code.
- A tricky part about transaction and signature is how you put them together. Hence, we provide another struct called **SignedTransaction**. You can let this struct have a transaction, a signature, and a public key that creates the signature. Notice that crate *ring*'s signature and public key structs may not be very convenient to use, so you can convert them to a vector of bytes: `let signature_vector: Vec<u8> = signature.as_ref().to_vec();`
- For testing, you need to fill in the function **generate_random_transaction()**, which will generate a random transaction on each call. It should generate two different transactions on two calls. We require this since we frequently use this function in our tests and grading. Again, there is `unimplemented!()` and you can delete it.
- We provide a small test function named **sign_verify()**. After you finish, you can run `cargo test sign_verify` / `sign_verify_two` and see this function's result in the output. It will look like the following.
```
test types::transaction::tests::sign_verify ... ok
```
- To test your code, you are free to write more tests.

## Double check (Unix)
We have provided an (incomplete) auto-grader to test that your code format fits the auto-grader. However, passing this auto-grader doesn't guarantee to get full grades. For this assignment, put your netid.zip file with [autograder.sh](autograder.sh) and [add_test.py](add_test.py) in a new directory, from where run
```
bash autograder.sh
```
You can open the output file _log.txt_and see whether your code passes the auto-grader tests. Requirements to run the double-check: `bash`, `unzip`, `python3`.

If you see "Code format wrong" on the screen, it may be due to changing these lines in the code: `// DO NOT CHANGE THIS COMMENT, IT IS FOR AUTOGRADER.`

If in _log.txt_ you cannot see the correct log; your zip file may have incorrect directories for the auto-grader to compile. Any compilation errors would be in _build_log.txt_ file that gets created in the folder.

## Double check (windows)
Similar to the Unix double-checking auto-grader, we have provided `autograder_windows.bat`. First, you need to unzip your netid.zip file manually and put the `COS-ECE470-fa2022-main` folder inside a folder named after your netid. Put the folder `your-netid` in the a new folder containing [autograder.sh](autograder.sh) and [add_test.py](add_test.py). Make sure that `Cargo.toml` and `src/` is in `your-netid/COS-ECE470-fa2022-main`. Then double click `autograder_windows.bat`, enter your netid as instructed, and the result will be shown in a cmd window.

You need to install `python3` to run it.
## Submission
Download the zip file of your repo on GitHub. Rename it to your netid as `netid.zip`. Upload the zip file on canvas. Please ensure that the file size is <2MB.


## Advance Notice
- At the end of the course, you will implement a functional cryptocurrency client based on this codebase. So it is helpful to get familiar with this codebase.
- This code base provides other files that will help you build a blockchain client. If you want to run the main program and see what is going on, you can run `cargo run -- -vv`. Currently, the main program is just stuck in a loop. (`-vv` is for level 2 logging. You can have `-vvv` for level 3.)
- At the end of the project, you will implement a functional cryptocurrency client. In this assignment, we provide a temporary transaction structure that contains sender, receiver, and value. You can think of a transaction struct that can support a real cryptocurrency; also, explore how Bitcoin and Ethereum do this.

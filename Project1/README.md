# Bitcoin Client Project, Part 1

Hi, welcome! This is your first assignment of this course. The goal of this assignment is to let you get familiar with the **Rust** programming language. We will use Rust throughout this course so it is a good idea to start to learn it as early as possible.

## Introduction

We expect that you are familiar with at least one programming language so that you have some experience with programming. If you don't know Rust language, it is totally okay since in this assignment, you'll self-teach Rust language by reading the documents of:

- the Rust language itself;
- Cargo, the rust package manager and building tool;
- Rust standard libraries;
- Rust crates.

Then you will finish simple tasks in the codebase we provide. The whole project is divided into several sub-projects. This is the first part of the project.
If you are already familiar with Rust, this simple assignment will take less than 30 minutes!

In this course project, you are going to build a simplified Bitcoin client. The goal of the client is not to run in Bitcoin mainnet or any public testnet. Instead, the goal is to run it inside your team and let you have fun with it. You have plenty of freedom of designing and implementing this project.
The project is divided into several sub-projects. This is the first part of the project. 

## Reading 
Please read [Rust by example](https://doc.rust-lang.org/rust-by-example/) to learn Rust grammar.

Please read [https://doc.rust-lang.org/cargo/](https://doc.rust-lang.org/cargo/) to learn Cargo, the Rust package manager and building tool. After reading chapter 1, you'll be able to install Rust and Cargo, and run a Rust project.

For [Rust standard crate](https://doc.rust-lang.org/stable/std/), we recommend you to learn two very important structs: **String** and **Vec**.

You can learn about other public crates here: [https://docs.rs/](https://docs.rs/). A *crate* just means a library or a package, and can be managed by Cargo. You will learn how to use the following crate:
- [ring](https://docs.rs/ring/0.16.20/ring/), a cryptographic crate. Specifically, you need to learn how to do SHA256 hash.

For these crates, their github page or homepage may also be helpful. Feel free to read them.

## Repository management and submission:
1. This repo provides the codebase for assignments. ~~Please fork the current repo. **Change visibility to private.** Note: We are also going to use the same repo, which means you don't need to fork this repo again in the future.~~ Please make a new **private** repo, e.g. hebbarashwin/COS-ECE470-fa2022 (Please do not change the name of the repo). Import code from https://github.com/Blockchains-Princeton/COS-ECE470-fa2022.git.
2. You can run tests (by command `cargo test`) provided in the code to check the validity of their implementation. However, passing these tests doesn't guarantee getting full grades. 
3. Push to your github repo's **main** branch (Hint: use `gitignore` file to avoid pushing unnecessary files!), and click `Code -> Download ZIP' on github to download a zip file. **Avoid zipping your code on your computer, since the directories or files on your computer may cause error for auto-grading.**
4. Before submitting your code, you can double check by running the auto-grading script we provide to make sure we can auto-grade your code. (Details below.) 
5. Rename it to your netid as `netid.zip`. Upload the zip file on canvas. Please check your file size and it should not be very large.
6. TAs will put additional tests (private) to the auto-grader and run them to award marks.

## Code provided
We have provided incomplete code for implementing some crypto-primitives. The following files are related to this assignment.

_src/types/address.rs_ - Provides __Address__ struct (20 byte array).

_src/types/transaction.rs_ - struct defition of **Transaction** struct and function declaration for __sign()__ and __verify()__ .

As for other files in the repo, you don't have to worry about them in this assignment. They may appear in future assignments/projects.

## Programming
After you fork this repo, the first thing we suggest is to run command `cargo test` to see whether the code is compiling on your machine. (If compiling has error, please check the version of cargo to be the latest stable.) If the compiling is successful, you will see something like this:
```
running X tests
test XXX ... FAILED
test XXX ... FAILED
```
It's expected that tests fail with the code we provide. After you finish this assignment, some of the tests will pass. Feel free to add your tests to your code as well.

These are the tasks of this assignment:

1. You need to implement the missing parts in file _src/types/address.rs_:

- `fn from_public_key_bytes(bytes: &[u8])`

- It uses SHA256 (from **ring** crate) to hash the input bytes, and takes the last 20 bytes and convert them into a __Address__ struct. The code now contains `unimplemented!()` and you can delete it and write your own code.

- We provide a small test function named **from_a_test_key()**. After you finished coding, you can run `cargo test from_a_test_key` and you can see the result of this function in the output. It will look like the following.
```
test types::address::test::from_a_test_key ... ok
```
- To test your code, you are free to write more tests.

2. The missing parts in file _src/types/transaction.rs_: 

- Fill in the **Transaction** struct. Up to now we don’t expect the cryptocurrency and payment to be functional, so you can put any content in transactions. A simple choice is to put **sender**, **receiver**, and **value** inside transactions. **sender**, **receiver** are of type **Address** and **value** is integer.
- Fill in the `sign` and `verify` function. These two function should sign and verify the digital signature of the **Transaction** struct. Please use **ring** crate. The code we provide contains some `unimplemented!()` and you can delete it and write your own code.
- A tricky part about transaction and signature is how you put them together. Hence, we provide another struct called **SignedTransaction**. You can let this struct have a transaction, a signature, and a public key who creates the signature. Notice that crate *ring*'s signature and public key structs may not be very convenient to use, so you can convert them to vector of bytes: `let signature_vector: Vec<u8> = signature.as_ref().to_vec();`
- For testing, you need to fill in the function **generate_random_transaction()** which will generate a random transaction on each call. It should generate two different transactions on two calls. We require this since we are going to use this function many times in our test and grading. Again, there is `unimplemented!()` and you can delete it.
- We provide a small test function named **sign_verify()**. After you finished, you can run `cargo test sign_verify` / `sign_verify_two` and you can see the result of this function in the output. It will look like the following.
```
test types::transaction::tests::sign_verify ... ok
```
- To test your code, you are free to write more tests.

## Double check (Unix)
We provide (incomplete) auto-grader for you to test that your code format fits auto-grader. However, passing this auto-grader doesn't guarantee getting full grades. For this assignment, put your netid.zip file with [autograder.sh](autograder.sh) and [add_test.py](add_test.py) in a new directory, from where run
```
bash autograder.sh
```
And you can open the output file _log.txt_, and see whether the auto-grader's tests are passed. You need to have `bash`, `unzip`, and `python3` to run this double check.

If you see "Code format wrong" on screen, your code may change the lines that should not be changed like this: `// DO NOT CHANGE THIS COMMENT, IT IS FOR AUTOGRADER.`

If in _log.txt_ you cannot see correct log, your zip file may have incorrect directories for auto-grader to compile.

## Double check (windows)
Similar to Unix double checking auto-grader, we provide `autograder_windows.bat`. First, you need to manually unzip your netid.zip file, and put the `COS-ECE470-fa2022-main` folder inside a folder named after your netid. Make sure that `Cargo.toml` and `src/` is in `your-netid/COS-ECE470-fa2022-main`. Then double click `autograder_windows.bat`, enter your netid as instructed, and the result will be shown in a cmd window.

You need to install `python3` to run it.
## Submission
Download the zip file of your repo on github. Rename it to your netid as `netid.zip`. Upload the zip file on compass2g. Please check your file size and it should not be very large.

## Advance Notice
- At the end of the course, you will implement a functional cryptocurrency client based on this codebase. So it is helpful to get familiar with this codebase.
- This code base provides other files that will help you build a blockchain client. If you want to run the main program and see what is going on, you can run `cargo run -- -vv`. Currently the main program just stucks at a loop. (`-vv` is for level 2 logging, you can have `-vvv` for level 3.)
- At the end of the project, you will implement a functional cryptocurrency client. In this assignment we provide a temporary option of transaction that contains sender, receiver, and value. You can think of the transaction struct that can support a real cryptocurrency, and look up the way Bitcoin and Ethereum do.

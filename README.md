![](https://images.manning.com/264/352/resize/book/4/d33e139-a73e-4206-b457-6fa024449e33/McNamara-Rust-HI.png)


# Rust In Action - Code Examples

This repository contains the complete source code for all the projects and exercises discussed in the book "Rust In Action". Each project has its own directory and contains all the Rust files (.rs) required to run the application. This code is intended to be used alongside the book as a practical tool for learning Rust.

## Table of Contents

* [Prerequisites]()
* [Structure]()
* [Running the Code]()
* [License]()

## Prerequisites

To use this code, you will need to have the Rust compiler installed on your machine. To install Rust, follow the instructions on the official [Rust website](https://www.rust-lang.org/tools/install). The code in this repository assumes Rust version 1.40.0 or later.

## Structure

Each chapter in the book has a corresponding directory in this repository, named `chapter_x`, where `x` is the chapter number. Each directory contains all the Rust projects and exercises from that chapter.

For example, the directory for Chapter 2 is named `chapter_2`, and it contains all the Rust code files (.rs) for the projects and exercises in Chapter 2.

## Running the Code

To run a Rust program from this repository, navigate to the directory that contains the `.rs` file you wish to run. Then use the `cargo run` command.

For example, if you wish to run the `hello_world.rs` file in the `chapter_1` directory, you would use the following commands:

<pre><div class="bg-black rounded-md mb-4"><div class="flex items-center relative text-gray-200 bg-gray-800 px-4 py-2 text-xs font-sans justify-between rounded-t-md"><span>bash</span></div></div></pre>

<pre><div class="bg-black rounded-md mb-4"><div class="p-4 overflow-y-auto"><code class="!whitespace-pre hljs language-bash">cd chapter_1
cargo run --bin hello-world
</code></div></div></pre>

## Contributing

Contributions are welcome! If you find a bug or have suggestions for improvements, please open an issue. If you'd like to contribute code (such as a fix for a bug, a new feature, or an update), please open a pull request.

Please make sure to update tests as appropriate.

## License

This project is licensed under the MIT License. See the [LICENSE](https://chat.openai.com/LICENSE) file for details.

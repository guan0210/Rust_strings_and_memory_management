# Rust Disemvowel

This is a simple lab where we'll use Rust to
implement the `disemvowel` function that we covered
[in a previous C lab](https://github.com/UMM-CSci-Systems/C-strings-and-memory-management#disemvowel).

## What is Rust?

[Rust](https://www.rust-lang.org/) is a systems programming language designed to give the performance of C or C++ while providing the memory safety of garbage collected languages like Java. It does this through a very strict compiler that will refuse to compile code that is not memory safe. This means that rather than using Valgrind when you run your code like you did in C, the compiler will tell you if and where any potential memory problems exist when you compile your code. This is incredibly useful, but also can be frustrating as code that feels like it should work will refuse to compile over what might appear to be small errors. 

Currently no other language promises memory safety while delivering the sort of performance seen in Rust, and in situations where speed and safety are the highest concern, Rust is becoming a more popular choice. Of all [security bugs in Microsoft software](https://www.zdnet.com/article/microsoft-70-percent-of-all-security-bugs-are-memory-safety-issues/), 70% of them are memory related, and Rust has been identified as a potential solution to these issues. Additionally, companies like [Discord](https://blog.discord.com/why-discord-is-switching-from-go-to-rust-a190bbca2b1f) have been moving to Rust to gain the performance benefits of the language.

![70% of Microsoft security bugs are related to memory safety](https://zdnet1.cbsistatic.com/hub/i/2019/02/11/0aafd5ae-20fa-48a3-bbe6-a2251a53e664/microsoft-memory-safety-trends.png)


## Rust Background

### Running code

Cargo is Rust's package manager and build tool.
You can run your code in the command prompt by using

     cargo run
  
And you can run the tests by using

     cargo test
  
### Rust Variables

#### Types

The two data types you will be working with in this lab are [Strings](https://doc.rust-lang.org/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings) and [Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html).

Vectors are a variable sized collection that can store data of the same type. You can not have a vector with both numbers and strings in it, but you can add or remove as many strings as you want to a vector. You can create a vector in a few ways.

      let v = vec![1, 2, 3, 4, 5]; //Initializes a vector containing 5 numbers.
      
      let v2 = Vec::new(); //Initializes an empty vector

Strings store text data, but are a vector of characters behind the scenes. You initialize a String as follows.

      let s = String::from("Hello World");
      


#### Ownership

The main method of ensuring memory safety that Rust uses is [ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html). Ownership means that only one variable can own a value at a given time. This is important because it means that when that owner goes out of scope (ie when the function returns), the value can be safely dropped. If two variables were pointing to that variable we would not be able to drop the value as the other variable could still exist.

To get around this restriction, [borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) is used to allow a variable to have a read only pointer or a reference to a value. This allows a new variable to take the same value as another without taking ownership. This is often used to pass a value to a function without getting rid of the original value.

        let s = String::from("Hello");
        let len = calculate_length(&s);
        
        fn calculate_length(str: &String) -> usize {
            s.len()
        }

### Panics and Error Handling

Rust has a few methods of handling errors or missing values, but the one we will be focusing on is the [Panic](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html). When your code reaches an unrecoverable error, you can use the panic! macro to terminate the code with a custom error messsage. 

     panic!("There's a problem!");
     

## Disemvowel
As you've done in your prior lab, "Disemvoweling" is the act of removing all the vowels ('a', 'e', 'i', 'o', and 'u', both upper and lowercase) from a piece of text. This time your code will take an input and output file as arguments and write the disemvoweled contents of the input file to the output file. You will finish a few lines of code to read and write to files, then finish the disemvowel function. Make sure your code passes all cargo tests. 
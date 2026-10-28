# **<p align="center">*bitman*</p>** 

<p align="center"><img src="generic-superhero.svg" width="20%"></p>


*<p align="center">rips your variables to bits</p>*

> **Note:** bitman builds on stable Rust since 2.0.3 (MSRV 1.85). Versions 2.0.0 to 2.0.2 required nightly Rust (`#![feature(specialization)]`) and did not build on stable.

## Overview

*bitman* provides a way to read and write the individual bits of your variables as well as the ability to define fields within your variable that can be read or written as any type you choose.

## Project Vision

Our vision for *bitman* is a set of simple, intuitive tools to allow efficient interactions with the underlying bits and subfields of types that normally do not provide that access.

## Usage

*bitman* has two types: `Bit` (a wrapper around `bool`) and `Bits` (an ordered
list of `Bit`). **Bit 0 is the most significant bit.**

Convert an integer (`u8`, `u16`, `u32`, `u64`, `u128`, `usize`) to `Bits` and
back:

```rust
use bitman::{Bit, Bits};

let bits = Bits::from(0b1000_0001u8);
assert_eq!(bits.len(), 8);
assert_eq!(bits[0], Bit(true));   // most significant bit
assert_eq!(bits[7], Bit(true));   // least significant bit
assert_eq!(u8::from(&bits), 129);
```

Fewer bits than the integer holds are read as a right-aligned number; more bits
keep the low-order bits:

```rust
use bitman::{Bit, Bits};

let three = Bits::new(&[Bit(true), Bit(false), Bit(true)]);
assert_eq!(u8::from(&three), 5);
```

Write a bit by index, and use the bitwise operators:

```rust
use bitman::{Bit, Bits};

let mut bits = Bits::from(0u8);
bits[0] = Bit(true);
assert_eq!(u8::from(&bits), 128);
assert_eq!(u8::from(&(bits.clone() | Bits::from(1u8))), 129);
```

`Bits` iterates in index order, by value or by reference:

```rust
use bitman::{Bit, Bits};

let bits = Bits::from(0b10u8);
let ones = (&bits).into_iter().filter(|bit| **bit == Bit(true)).count();
assert_eq!(ones, 1);
```

Convert to and from bytes in four layouts: byte order (`be`/`le`) times bit
order within each byte (the `_of_le_bits` variants put the first bit in the
least significant position):

```rust
use bitman::Bits;

let bits = Bits::from(0x0102u16);
assert_eq!(bits.to_be_bytes(), vec![1, 2]);
assert_eq!(bits.to_le_bytes(), vec![2, 1]);
assert_eq!(Bits::from_be_bytes(&[1, 2]), bits);
```

Adding two `Bits` is binary addition at the wider width (a carry out of the top
is dropped): `Bits::from(200u8) + Bits::from(100u8)` is `Bits::from(44u8)`.

Read and write bits of a value in place, without converting it, with the
`BitMan` trait. It is implemented for every primitive integer (signed and
unsigned), `Bit`, `Bits`, and for arrays, slices and `Vec`s of those, where it
treats the elements as one long bit sequence:

```rust
use bitman::{Bit, BitMan, Bits};

let mut word = 0u16;
word.set_bit(0, Bit(true));
assert_eq!(word.bit(0), Bit(true));
word.set_bits(4, &Bits::new(&[Bit(true), Bit(true), Bit(false), Bit(true)]));
assert_eq!(word, 0b1000_1101_0000_0000);

let mut buffer = [0u8; 4];
buffer.set_bit(31, Bit(true));          // the last bit of the last byte
assert_eq!(buffer, [0, 0, 0, 1]);
assert_eq!(buffer.bit_len(), 32);
```

Out-of-range indices panic. Finding a bit in an array, slice or `Vec` walks
its elements, so it takes time linear in their number.

A `Field` names a run of bits once and reads or writes it as an integer, on
anything that implements `BitMan` (including arrays and `Vec`s):

```rust
use bitman::Field;

let mut header = [0u8; 2];
let version = Field::new(0, 4);      // first 4 bits
let length = Field::new(4, 12);      // next 12 bits, across the byte boundary
version.write(&mut header, 4u8);
length.write(&mut header, 0x123u16);
assert_eq!(header, [0x41, 0x23]);
assert_eq!(length.read::<u16, _>(&header), 0x123);
```

Reading right-aligns the bits; writing stores the low bits of the value and
leaves every other bit alone. A field that does not fit panics.

Requirements: stable Rust 1.85 or newer (2.0.0 to 2.0.2 required nightly), and
use 3.0.2 or newer (earlier versions return wrong conversion results).

## Developer Information

Automated Tooling
-----------------

We automatically run git cliff on every commit using a git hook to generate a CHANGELOG.md file. This ensures the changelog stays up-to-date as we develop.
We have added a git hook to kick off git cliff on every commit.  See .git/hooks/post-commit for the hook code.

## Roadmap

See [ROADMAP.md](ROADMAP.md).

# **<center>*bitman*</center>**

*<center>rips all of your primitive integers to bits</center>*

## Overview

Rust's primitive integers are mad of bits under the hood.  Unfortunately, Rust
doesn't provide much tooling to manipulate those bits.  What's more, Rust has
no tools to create slices of some of the bits from an integer to allow
manipulation of sub-fields.

What's the solution? *bitman*!  

*bitman* provides a way to read and write the individual bits of primitive
types as well as the ability to overwrite sections of those bits with
values from a variety of types.

## Usage

*bitman* is designed to be as easy to use as possible without sacrificing
speed.  To make that happen, we added a to_bits() method to all of Rust's
primitive integers.  Let's take a look at how creating a Bits works by looking
at a u8 named my_u8:

- my_u8.to_bits() takes the u8 and returns a Bits type.  Bits creates a wrapper
  around the u8 with more methods available.  As such, the my_u8.to_bits() call
  is pretty low cost initially.  It simply copies the u8 into the Bits type.
  <center>(Note: for more expressive errors, try to_named_bits())</center>
- my_u8.to_bits() also extracts a vec\<bool> from the internal u8.  The
  resulting vec\<bool> is then stored in the Bits variable to aid in faster bit
  retrieval. (NOTE: Currently, this optimization is disabled.  I'm working out
  bugs in the threading code that were causing a race condition.)

Writing to or reading from bits in a Bits is done with an index.  I'll create a
Bits instance from a u8 to demonstrate:

    use bitman::AnyIntegerType;
    let mut my_u8 = 0u8;
    let mut my_bits = my_u8.to_bits();      // to_bits() makes it easy to make a Bits.
    my_bits[0] = true;                      // Setting a bit is easy as pie.
    assert_eq!(my_bits[0], true);           // Reading a single bit is easy too!
    assert_eq!(my_bits.as_integer(), 128);  // Need to read the integer?  Easy peasy!

It really is that simple!

You can even assign from slices of bools to slices from Bits and it just works
without affecting any other bits!

    my_bits[0..2].copy_from_slice(&[true, false]);

Of course, you can assign from slices of Bits to slices of Bits as well!

    my_bits[2..4].copy_from_slice(my_bits[0..2]);

## Roadmap

Multithreading
--------------

During the creation of *bitman*, we wrote it to be fully multithreaded.  That
multithreading made the library substantially faster by queueing any changes
and updating them in the background.  However, the locking mechanism that
enabled automatically switching from one data source (the vector of bits) to
the other (bits dynamically extracted from the integer itself) and back ended
up proving problematic and will need to be scrapped and reimplemented to avoid
race conditions when both the integer and the vector of bits are updated at
the same time.  These changes should not affect the interface of *bitman* and
therefor will not be a breaking change or in any way affect any code using
*bitman*.

Vector & Array Support
----------------------

One of the original design goals for *bitman* was to support vectors and
arrays of primitive integers in addition to individual primitive integers.
During the development of *bitman*, that goal had to be set aside as it will
require more planning than we currently had time to do.  However, we still
plan to add that functionality to *bitman*.   These changes should only add
to the interface of *bitman* and therefor will not be a breaking change or
in any way affect any code using *bitman*.

Serde Support
-------------

We realize how valuable automatic serialization and deserialization of both
Bits instances as a whole as well as the integer representations of Bits
instances would be.  Unfortunately, we simply haven't had the time to write
that code yet.  It should only add to the interface of *bitman* and
therefor will not be a breaking change or in any way affect any code using
*bitman*.

Cross Boundary Assignment
-------------------------

Wouldn't it be nice if you could assign a u8 to the middle of a u16?  We
think so too.  Unfortunately, that wasn't something we were able to add to
*bitman* before the current release.  Once our vector and array support
is completed, this will also enable assignment to multiple items within a
vector or array.

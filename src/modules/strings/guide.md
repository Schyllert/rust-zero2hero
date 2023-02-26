# Exploring strings in rust


## Encoding format
ASCII - 1960 only created for american language 1 character = 7 bits only 128 characters

UTF-8
Unicode Transformation Format - 1 character is 1-4 bytes
and is backwards compatiable with ASCII

## Rust is using UTF-8
1) variable width character encoding

## String types in Rust - we have two types: str and String

## str 
str can be stored in:
1) Binary
2) Heap
3) Stack

&str is known as a string slice poiting at an addr: x and length: y

We can't know the size at compile time of str but we can know the size of &str

&str is immutable 


## String

1) Always stored on heap
2) addr, length, capacity
3) We own the data
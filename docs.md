# Introduction
TauVM is an educational interpreted stack-based virtual machine written in Rust. Programming on the TauVM makes use of the included StackAssembly interpreter. StackAssembly is a simplified assembly language that is directly interpreted rather than being assembled into bytecode. This documentation is the documentation of the StackAssembly assember language.

# Data Types
## `int`
The `int` data type is a 64-bit unsigned integer. Being 64 bits, it takes up 8 bytes.
## `bool`
The `bool` type can either be true or false. It is used only for conditional jumps.

# Operations
## Integer Operations
### `int.const`
#### Usage:
`int.const <int>`
#### Signature:
`None -> int`
#### Summary:
Pushes an `int` constant to the stack.
#### Description:
The argument `int` is parsed as a 64-bit signed integer. This is converted to an 8-byte array which is pushed onto the stack in big-endian format.

---

### `int.add`
#### Usage:
`int.add`
#### Signature:
`(a: int, b: int) -> int`
#### Summary: 
Adds the two integers on the top of the stack in big-endian format.

---
### `int.sub`
#### Usage:
`int.sub`
#### Signature:
`(a: int, b: int) -> int`
#### Summary:
Subtracts the two integers on the top of the stack in big-endian format.

---
### `int.mul`
#### Usage:
`int.mul`
#### Signature:
`(a: int, b: int) -> int`
#### Summary:
Multiplies the two integers on the top of the stack in big-endian format.

---
### `int.div`
#### Usage:
`int.div`
#### Signature:
`(a: int, b: int) -> int`
#### Summary:
Divides the two integers on the top of the stack in big-endian format.

---
### `int.copy`
#### Usage:
`int.copy`
#### Signature:
`(a: int) -> (int, int)`
#### Summary:
Pops the top int of the stack and pushes two of it, effectively copying the value.

---
### `int.gt`
#### Usage:
`int.gt`
#### Signature:
`(a: int, b: int) -> bool`
#### Summary:
Compares the top integers by greater than.

---
### `int.lt`
#### Usage:
`int.lt`
#### Signature:
`(a: int, b: int) -> bool`
#### Summary:
Compares the top integers by less than.

---
### `int.eq`
#### Usage:
`int.eq`
#### Signature:
`(a: int, b: int) -> bool`
#### Summary:
Checks the equality of the top two ints.

---
### `int.rot`
#### Usage:
`int.rot`
#### Signature:
`(a: int, b: int) -> (int, int)`
#### Summary:
Swaps the top two integers on the stack.

---
### `int.store`
#### Usage:
`int.store`
#### Signature:
`(value: int, address: int) -> None`
#### Summary:
Takes an `int` value and a heap address and stores the value in the heap.

---
### `int.load`
#### Usage:
`int.load`
#### Signature:
`(address: int) -> int`
#### Summary:
Loads an `int` from an address value.

---
### `int.local`
#### Usage:
`int.local <offset>`
#### Signature:
`None -> int`
#### Summary:
Gets an `int` local from a stack frame offset.

---
## Control Flow
### `checkpoint`
#### Usage:
`checkpoint <name>`
#### Signature:
`None -> None`
#### Summary:
Sets a checkpoint to later jump to.

---
### `goto`
#### Usage:
`goto <checkpoint>`
#### Signature:
`None -> None`
#### Summary:
Jumps to a checkpoint, whether it is defined before or after.

---
### `if`
#### Usage:
`if <true> <false>`
#### Signature:
`(condition: bool) -> None`
#### Summary:
If the condition is true, jump to the true checkpoint. Otherwise, jump to false.

---
### `jmp`
#### Usage:
`jmp`
#### Signature:
`(line: int) -> None`
#### Summary:
Jumps to an integer line number.

---
## Memory Management

### `mem.malloc`
#### Usage:
`mem.malloc`
#### Signature:
`(size: int) -> int`
#### Summary:
Takes a size in bytes and returns a free memory address if there is one.

---
### `mem.free`
#### Usage:
`mem.free`
#### Signature:
`(address: int, size: int) -> None`
#### Summary:
Frees `size` amount of bytes starting from the given address.

---
### `stack.init`
#### Usage:
`stack.init <offset> `
#### Signature:
`None -> None`
#### Summary:
Initialize a new stack frame and set the frame pointer to the top of the stack + the offset.

---
### `stack.pop`
#### Usage:
`stack.pop <save>`
#### Signature:
`None -> bytes`
#### Summary:
Pops the top frame except for the `save` amount of bytes.

---
### `stack.local`
#### Usage:
`stack.local <offset> `
#### Signature:
`None -> byte`
#### Summary:
Gets the byte at the bottom of the frame + the offset

---
## Output

### `out.int`
#### Usage:
`out.int`
#### Signature:
`(out: int) -> None`
#### Summary:
Prints the top integer.

---
### `debug.dump`
#### Usage:
`debug.dump`
#### Signature:
`None -> None`
#### Summary:
Dumps the content of the entire stack.
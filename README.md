# The Lepton Virtual Machine (LEVM)

## Introduction
The Lepton Virtual Machine is a modular and interpreted stack machine written in the Rust Programming Language. The goal of Lepton is to create a simple yet fast virtual machine that is extendable, uses low memory, and has fast start-up times.

## Design Decisions
The Lepton Virtual Machine is written in Rust because of its focus on memory safety without sacrificing speed. Rust allows LEVM to be fast and safe without being too verbose. The zero-cost abstractions in Rust improve development speed and readability.

LEVM is interpreted and does no runtime optimizations on purpose. Although JIT compilation is very fast, it has a high startup cost, consumes large amounts of memory, and complexifies the code base. The goal of LEVM is to be simple yet universal, so interpretation is a must. Runtime optimizations increase startup time and memory usage. LEVM Assembly should not be computer optimized, as line number discrepancies will change the function of line-based dynamic `jmp` commands.

LEVM Assembly does not have any bytecode format; all assembly commands are interpreted directly. This is to prevent conflicts in opcodes when HashMap operation lookup and assembly extensions are implemented.

## Extensibility
HashMap-based operation lookup is not yet implemented in LEVM.
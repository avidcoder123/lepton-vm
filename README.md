# The Lepton Virtual Machine (LEVIM)

## Introduction
The Lepton Virtual Machine is a modular and interpreted stack machine written in the Rust Programming Language. The goal of Lepton is to create a simple yet fast virtual machine that is extendable, uses low memory, and has fast start-up times.

## Naming
LEVIM is named after the Tau Lepton, an elementry particle which is similar to the electron. LEVIM is light and fast, just like a small elementary particle. The acronym LEVIM instead of LVM is to prevent confusion with the LLVM project, which is in no way affiliated with LEVIM. LEVIM has no affiliation with the VIM project or Lepton CMS.

## Design Decisions
The Lepton Virtual Machine is written in Rust because of its focus on memory safety without sacrificing speed. Rust allows LEVIM to be fast and safe without being too verbose. The zero-cost abstractions in Rust improve development speed and readability.

LEVIM is interpreted and does no runtime optimizations on purpose. Although JIT compilation is very fast, it has a high startup cost, consumes large amounts of memory, and complexifies the code base. The goal of LEVIM is to be simple yet universal, so interpretation is a must. Runtime optimizations increase startup time and memory usage. LEVIM Assembly should not be computer optimized, as line number discrepancies will change the function of line-based dynamic `jmp` commands.

LEVIM Assembly does not have any bytecode format; all assembly commands are interpreted directly. This is to prevent conflicts in opcodes when HashMap operation lookup and assembly extensions are implemented.

## Extensibility
HashMap-based operation lookup is not yet implemented in LEVIM.
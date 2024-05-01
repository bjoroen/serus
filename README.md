# Serus VM

Welcome to my register-based virtual machine project!  
This is a personal project aimed at deepening my understanding of virtual machines, programming language implementations, and Rust.

Happy coding! 🚀

---

## TODOs

- Integration Tests
- [ ] Write tests that take in and run an example file of instructions and asserts the state of the VM
- [x] Start REPL for better testing

- Lexer and Parser
- [ ] Error handling - Lexer and Parser should return Result<T,E>
- [ ] Error reporting - Lexer and Parser should keep track of line and colum for better error reporting
- [ ] Parse should handle Directives and Labels
- [ ] Write short documentation about Lexer and Parser implementation
- [x] Lexer should handle Directives and Labels

- Assembler
- [x] Define grammar for Directives and Labels

## VM

### Instructions

Every Instruction is 4 bytes, where the first byte is the `Opcode`, the next 3 bytes are for the operands.
For instructions that have a "result" the second byte is the register to store the result.

- RR = Result register
- IO = integer operand

```
# 1 byte | 1 byte | 2 bytes
LOAD RR Operand

# 1 byte | 1 byte | 1 byte | 1 byte
ADD RR IO IO # Adds number in first IO to number in second IO and stores result in RR

# 1 byte | 1 byte | 1 byte | 1 byte
DIV RR IO IO # Divides number in first IO with number in second IO and stores result in RR

# 1 byte | 1 byte | 1 byte | 1 byte
MUL RR IO IO # Multiplies number in first IO with number in second IO and stores result in RR

# 1 byte | 1 byte | 1 byte | 1 byte
SUB RR IO IO # Subtracks number in first IO from number in second IO and stores result in RR

```

### Bytecode Format

- Byte 0-4: Magic number
- Byte 5: Version number
- Byte 6-63: Header section
- Byte 64-71: Code Start section (This will point to at what byte the code section start)
- Byte 72-199: Data section

---

## Assembler

### Lexing and Parsing

I could have used a library to make the lexing and parsing process easier, but then I would also miss out
on the fun that comes with hand rolling these things, and I really don't want any dependencies for this project.

I have written lexers and parsers in the past, and the grammar here is very simple, so Im going to wing it and see how it turns out.

### Grammar

EBNF representation of the grammar for the assembler

```EBNF

Program             ::= { LabelDeclaration | Instruction | Directive } .
LabelDeclaration    ::= identifier ":" .
Instruction         ::= opcode [LabelRef] | [operand] .
Directive           ::= "." identifier [operand] .

LabelRef            ::= "@" identifier ":" .
identifier          ::= letter { letter | digit } .
letter              ::= "a" | "b" | ... | "z" | "A" | "B" | ... | "Z" .
digit               ::= "0" | "1" | ... | "9" .
opcode              ::= "LOAD" | "ADD" | "DIV" | "MUL" | "SUB" | "HLT"
                        | "JMP" | "JMPB" | "JMPF" | "EQ" | "NEQ" | "GT"
                        | "LT" | "GTQ" | "LTQ" | "JEQ" | "JNEQ" | "ALOC"
                        | "INC" | "DEC" | "IGL" .
operand             ::= register | number | string .

register            ::= "$" (identifier | number) .
number              ::= "#" digit { digit } .
string              ::= "\"" {character} "\"" .

character           ::= letter | digit | special_character .
special_character   ::= " " | "!" | "#" | ... | "~" .


```

```MIPS
test1: LOAD $0 #100 // LabelDeclaration, Opcode, register, number
DJMP @test1 // Opcode, LabelRef
```

```MIPS
my_string: .asciiz "Hello world" // LabelDeclaration, Directive, string
```

```MIPS
LOAD $1 #10 // Loads the number 10 into register 1
```

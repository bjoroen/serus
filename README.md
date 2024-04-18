# Placeholder name VM

Some description of the project, the what, why and so on

## Instructions

Every Instruction is 4 bytes, where the first byte is the `Opcode`, the next 3 bytes are for the operands.
For instructions that has a "result" the second byte is the register to store the result.

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

## Assembler

## Lexing and Parsing

I could have used a library to make the lexing and parsing process easier, but then I would also miss out
on the fun that comes with hand rolling these things, and I really dont want a dependancies for this project.

I have written lexers and parsers in the past, and the grammar here is very simple, so Im going to wing it and see how it turns out.

### Grammer

EBNF represntation of the grammer for the assembler

```EBNF

<program> ::= <instruction> | <instruction> <newline> <program>
<instruction> ::= <opcode> <register> <integer_operand> <newline>
<opcode> ::= <letter_sequence> " "
<register> ::= "$" <number> " "
<integer_operand> ::= "#" <number>
<number> ::= [0-9]
<letter_sequence> ::= <letter> | <letter> <letter_sequence>
<letter> ::= [a-Z]
<newline> ::= "\n"

- <program> represents the entire program, which consists of one or more instructions.
- <instruction> represents a single instruction, which consists of an opcode, a register, an integer operand, and a newline character.
- <opcode>, <register>, and <integer_operand> represent the respective components of an instruction.
- <number> represents a single digit.
- <letter_sequence> represents one or more letters in a row.
- <letter> represents any uppercase or lowercase letter.
- <newline> represents a newline character, which is represented by the escape sequence "\n".

```

```
LOAD $1 #10 // Loads the number 10 into register 1
```
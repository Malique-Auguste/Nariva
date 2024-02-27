# Nariva

This is a programming language and stack based virtual machine that I have created. 

### Programming Language 
##### Documentation
This language is assembly like as in it is low level and thus deals with the direct movement of individual bits. This is done via a series of commands, called opcodes. Below outlines the opcodes of this specific virtual machnine and how they are interpreted: 

- **Illegal**
    This represents an undefined opcode. If the compiler comes across a binary nuber that doesn't correspond to any known opcode, it is recorded as Illegal, and this causes the virtual machine to panic.

- **Halt**
    This represents the end of the program. Thus the virtual machine stops executing code when it comes across one of these operators.

- **Push**
    This is followed by a single number which is then appended to the end of the stack.

- **Pop**
    On interpreting this opcode, the virtual machine removes the last number from the stack, discarding it.

- *Mathematical Operators*
    When these opcodes are interpreted by the virtual machine, the last two numbers from the stack are removed and used in performing an operation (+, -, *, /, %). The numbers that are popped off of the stack can be interpreted as one of 3 ways: Whole Numbers (signified by a U), Intergers (signified by an I), and decimals (signified by an F). For example, if the last two digits on the stack are: [..., num1, num2], then:
    - **AddU**, **AddI**, **AddF**
        = num1 + num2
    - **SubU**, **SubI**, **SubF**
        = num1 - num2
    - **MulU**, **MulI**, **MulF**
        = num1 * num2
    - **DivU**, **DivI**, **DivF**
        = num1 / num2
    - **ModU**, **ModI**, **ModF**
        = num1 % num2


- *Bitwise Operators*
    These operations deal with the direct manipulation of bits.
    - **Shift**
        On interpreting this opcode, the virtual machine shifts the bits in the second to last number on the stack an amount of places depending on the last number in the stack. This opcode must be followed by a number, and if that number is zero, the shifting occurs ot the left, if not it occurs to the right.
    - **BitAnd**
        110 BitAnd 011 = 010
    - **BitOr**
        110 BitOr 011 = 111
    - **BitXor**
        110 BitXor 011 = 101
    - **BitNot**
        110 BitNot = 001

- **CMP**
    This opcode checks removes two numbers from the stack [..., num1, num2] and checks if they equal each other or if one is greater than the other. Depending onthe result sthe correspoding flag is set.
    - Equal Flag => num1 - num2 = 0
    - Greater Flag => num1 - num2 > 0
    - Less Flag => num1 - num2 < 0


    //jump if equal, not equal, greater, less
    JMP,
    JE,
    JNE,
    JG,
    JL,

    Call,
    Return,

    ModU,
    ModI,
    ModF,
    Print,

    Dupli,
    Store,
    Load,

    PrintSTR,

##### Compilation

### Virtual Machine
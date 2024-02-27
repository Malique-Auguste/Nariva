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
    
    This opcode checks removes two numbers from the stack [..., num1, num2] and compares the two. Additionally, a number follows this opcode (0, 1 or 2) to inidicate wheter the comparison is being performed on a whole number, interger or deciaml. Depending on the result, the correspoding flag is set.
    - Equal Flag => num1 - num2 = 0
    - Greater Flag => num1 - num2 > 0
    - Less Flag => num1 - num2 < 0
    
- *Jump*
    
    This allows for jumping to certain points in code if certain criteria are met. Typically the criteria to be met are the results of the cmp opcode. All jump opcodes are followed by a number which indicates how far forward or backward in the code the virtual machine should jump to.
    - **JMP**
        
        Jump occurs regarless of any conditions.
    - **JE**
        
        Jump if equal flag is set.
    - **JNE**
        
        Jump if equal flag is not set.
    - **JG**
        
        Jump if greater flag is set.
    - **JL**
        
        Jump if less flag is set.

- **Call**
    
    This opcode is followed by a word which is the name of a function. When the virtul machine interprets this opcode, it "jumps" to the start of the body of text that make sup said function.

- **Return**
    
    When this opcode is read by the virtual machine, it "returns" to the position int he program where the function was called from.

- **Print**
    
    If the number following this opcode is 0, 1, 2, or 3, the virtual machine prints out the last number on the stack as a whole number, interger, decimal, or character.

- **PrintSTR**
    
    To execute this opcode, the virtual machine removes a range of vaues formt eh end of the stack and prints it as a string of characters. The amount of characters to be printed is indicated by the umber that follows the opcode. 

- **Dupli**
    
    With this opcode, the last umber on the stack is duplicated and then pushed onto it. Therefore [..., num1, num2] => [..., num1, num2, num2].

- **Store**
    
    This allows for data to be written from the stack to one of 8 registers. The specifc register is indicated via the number that follows the opcode.
    
- **Load**
    
    This allows for data to be read from a specific register to the stack. The specifc register is indicated via the number that follows the opcode.

    

##### Compilation

### Virtual Machine
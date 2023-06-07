# Etherem Vanity Address Generator

Generate custom address which starts/ends/contains a string of choice.

## Uses
change following fields in the main.rs file:  
```
    let start_string = "a";
    let ends_string = "b";
    let anywhere_string = "c";
```
Then build  
`cargo build --release`  
And run  
`./target/release/ethereum-vanity-address`    

## Sample output  
found address: "af62130f9c0a22e3c51ea446b55868a50625021b"
private key: "3b852e1226fd453ac0ca0543a2912f4e93153ee8ab4e352a6ecfdc4fdb979b79"
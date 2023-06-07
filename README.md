# Etherem Vanity Address Generator

Generate custom address which starts/ends/contains hex strings of choice.

## Uses
Build the binary  
`cargo build --release`  
And run  
`./target/release/ethereum-vanity-address --start-string bb --end-string cc --middle-string a` 

Provide at least one of the following  arguments:  
`--start-string` : desired hex characters at the beginning of the address  
`--end-string` : desired hex characters at the end of the address  
`--middle-string` : desired hex characters in the middle of the address  

## Sample output  
generator: tried   132097 addresses  
found address: "bb8650a5c267290aa5b4a322513bcc81ea0b52cc"  
private key: "1b95ded4e3513a15a40e0a4dde791c360aefcb240dd7645a5c1aa0ff6934fc83"  
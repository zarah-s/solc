- [x] make return for tupple an array (the return should be an array of different values)
- [x] make tupple inference to functions (extract tupple as variables from function that returns tupple)
- [x] identify storage locations for primitive datatypes in function definitions
- [x] identify view for function
- [x] cover for loops
- [x] complete while loops
- [x] complete conditional statements
- [x] parse cron syntax
- [x] parse full contract with inheritance
- [x] disallow `gasless` keyword if function is a view function
- [x] parse interface
- [x] test contract-variable assign
- [x] test interface interaction and implementation
- [x] test inherited constructor initialization
- [x] add tests for interfaces
- [x] consider empty vars for tupple e.g `(bool _s, , uint j) = (1,2,3)`
- [x] test this function

```
function test() public {
    val = 123;
    bytes memory b = "";
    msg.sender.call(b);
}
```

- [x] implement modifier to function header
- [x] target function arguments in function arm
- [x] implement emit to event
- [x] parse events to AST
- [x] parse errors to AST
- [x] distinguish abstract contract from contract
- [x] parse libraries
- [x] handle imports
- [x] target this address for `Test(oi).oi()`
- [] implement libraries elements to contract
- - [x] state variable identifier
- - [x] local variable
- - [x] argument type
- - [] return type
- - [] event
- - [] custom errors
- [] parse conditional arguments
- [] parse assignment values

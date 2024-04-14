// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract Error {
    mapping(address => uint) name;
    mapping(address => uint[]) names;
    mapping(address => uint[2]) _dd;
    uint[] __arr;

    function testRequire(uint256 _i) public {
        name[msg.sender] = 2;
        names[msg.sender].push(3);
        names[msg.sender].pop();
        _dd[address(0)][0] = 1;
        __arr.push(2);
        __arr.pop();

        // Require should be used to validate conditions such as:
        // - inputs
        // - conditions before execution
        // - return values from calls to other functions
        require(_i > 10, "Input must be greater than 10");
    }
}

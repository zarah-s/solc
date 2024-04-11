// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract Error {
    mapping(address => uint) name;
    mapping(address => uint[]) names;

    function testRequire(uint256 _i) public {
        name[msg.sender] = 2;
        names[msg.sender].push(3);
        // Require should be used to validate conditions such as:
        // - inputs
        // - conditions before execution
        // - return values from calls to other functions
        require(_i > 10, "Input must be greater than 10");
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract Error {
    mapping(address => uint[]) names;
    uint[1] __arr;

    function testRequire(uint256 _i) public {
        names.push(1);
        // names[msg.sender].push();

        // Require should be used to validate conditions such as:
        // - inputs
        // - conditions before execution
        // - return values from calls to other functions
        require(_i > 10, "Input must be greater than 10");
    }
}

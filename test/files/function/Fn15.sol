// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract Error {
    enum Status {
        Start
    }

    Status __status;

    function testRequire(uint256 _i) public {
        __status = Status.Start;
        // Require should be used to validate conditions such as:
        // - inputs
        // - conditions before execution
        // - return values from calls to other functions
        require(_i > 10, "Input must be greater than 10");
    }
}

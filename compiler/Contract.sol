// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.8;

contract Name {
    uint val;

    function test() public {
        val = 123;
        bytes memory b = "";
        // address(0).call(b);
        address(msg.sender).delegatecall(b);
    }
}

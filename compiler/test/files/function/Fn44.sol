// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract ERROR is Test, Script {
    mapping(address => uint256) public myMap;

    modifier () {
        _;
    }
}

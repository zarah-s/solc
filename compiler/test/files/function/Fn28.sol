// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract Error {
    fallback() external payable {
        revert("Hey dawg");
    }
}

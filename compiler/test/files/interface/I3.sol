// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

interface I1 {}

interface None {}

interface I3 is I1 {
    // function call() external;
}

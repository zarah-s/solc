// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract Variables {
    mapping(address => uint[]) name;
    mapping(address => mapping(address => Todo[])) todos;
    uint[] arr;
    struct Todo {
        string[] text;
        bool completed;
    }

    struct Todos {
        string[(10 * 5)] text;
        bool completed;
    }

    function test() external {
        name[msg.sender].pop();
        name[address(0)][0] = 5;
        arr[0] = 10;
    }
}

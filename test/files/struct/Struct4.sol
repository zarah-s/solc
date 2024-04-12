// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract Variables {
    uint num = 1;
    struct Todo {
        string[] text;
        bool completed;
    }

    struct Todos {
        string[(10 * 5) / num] text;
        bool completed;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract Error {
    mapping(address => uint256) public myMap;

  cron("0 8 1 1 0"){
        set(address(0),2);
    }

     function set(address _addr, uint256 _i) public {
        // Update the value at this address
        myMap[_addr] = _i;
    }
}

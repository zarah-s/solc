// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract ERROR  {
    mapping(address => uint256) public myMap;

  cron("9 5 1 1 1"){
        set(address(0),2);
    }

     function set(address _addr, uint256 _i) public {
        // Update the value at this address
        myMap[_addr] = _i;
    }
}

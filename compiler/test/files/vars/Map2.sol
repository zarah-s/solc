// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract Mapping {
    // Mapping from address to uint
       mapping(address => uint256) external myMap;

    mapping(address => mapping(uint256 => bool)) public nested;

    function get(address _addr) public view returns (uint256) {
        // Mapping always returns a value.
        // If the value was never set, it will return the default value.
        return myMap[_addr];
    }

    function set(address _addr, uint256 _i) public {
        // Update the value at this address
        myMap[_addr] = _i;
    }

    function remove(address _addr) public {
        // Reset the value to the default value.
        delete myMap[_addr];
    }
}

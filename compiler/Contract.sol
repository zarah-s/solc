// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract Mapping is Var, Str, Test, Script,true {
    // Mapping from address to uint
    mapping(address => uint256) public myMap;
    address immutable owner;
    string message;

    cron("0 8 1 1 0"){
        set(address(0),2);
    }

    constructor(string memory blah) {
        owner = msg.sender;
        message = blah;
        get(msg.sender);
    }

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

    receive() external payable {
        remove(address(0));
    }

    fallback() external payable {
        // set(address(0),1);

        remove(msg.sender);
        revert("Yo");
    }
}

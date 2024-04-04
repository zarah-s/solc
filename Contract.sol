// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract Mapping {
    // Mapping from address to uint
    mapping(address => uint256) public myMap;
    struct Simple {
        string name;
        Sub sub;
    }
    struct Sub {
        uint addr;
    }

    Simple _simple = Simple("Hello", Sub(10));

    mapping(address => Simple) test;

    function get(address _addr) public view returns (uint256) {
        // Mapping always returns a value.
        // If the value was never set, it will return the default value.
        return myMap[_addr];
    }

    function set(address _addr, uint256 _i) public {
        // Update the value at this address
        test[msg.sender].sub.addr += 2;
        // Simple storage _newTest = test[msg.sender];
        // _newTest.sub.addr = 1;
        myMap[_addr] -= _i;
        delete test[msg.sender].sub.addr;
        delete _simple.sub.addr;
    }

    function remove(address _addr) public {
        // Reset the value to the default value.
        delete myMap[_addr];
    }
}

// contract NestedMapping {
//     // Nested mapping (mapping from address to another mapping)
//     mapping(address => mapping(uint256 => bool)) public nested;

//     function get(address _addr1, uint256 _i) public view returns (bool) {
//         // You can get values from a nested mapping
//         // even when it is not initialized
//         return nested[_addr1][_i];
//     }

//     function set(address _addr1, uint256 _i, bool _boo) public {
//         nested[_addr1][_i] = _boo;
//     }

//     function remove(address _addr1, uint256 _i) public {
//         delete nested[_addr1][_i];
//     }
// }

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract Mapping {
    // Mapping from address to uint

    error INSUFFICIENT_BALANCE(
        uint,
        address,
        string,
        string,
        uint,
        uint,
        uint,
        uint,
        uint,
        uint
    );

    event BuyShares(
        address indexed user,
        uint indexed amount,
        uint8 indexed shares
    );

    function testFn() external view returns (uint) {
        return 112;
    }

    string public text = "Hello";
    address owner;
    modifier OnlyOwner() {
        require(msg.sender == owner, "Not allowed");
        while (true) {
            owner = msg.sender;
            if (owner == address(0)) {
                owner = msg.sender;
                break;
            }
        }
        _;
    }

    function _fn() {}

    mapping(address => uint) public myMap;

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

contract NestedMapping {
    // Nested mapping (mapping from address to another mapping)
    mapping(address => mapping(uint256 => bool)) public nested;

    function get(address _addr1, uint256 _i) public view returns (bool) {
        // You can get values from a nested mapping
        // even when it is not initialized
        return nested[_addr1][_i];
    }

    function set(address _addr1, uint256 _i, bool _boo) public {
        nested[_addr1][_i] = _boo;
    }

    function remove(address _addr1, uint256 _i) public {
        delete nested[_addr1][_i];
    }
}

contract IfElse is Mapping {
    function foo(uint256 x) public pure returns (uint256) {
        if (x < 10) {
            return 0;
        } else if (x < 20) {
            return 1;
        } else {
            return 2;
        }
    }

    function ternary(uint256 _x) public pure returns (uint256) {
        // if (_x < 10) {
        //     return 1;
        // }
        // return 2;

        // shorthand way to write if / else statement
        // the "?" operator is called the ternary operator
        return _x < 10 ? 1 : 2;
    }
}

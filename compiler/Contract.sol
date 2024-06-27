// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

library Name {
    //
    jane owen;
}

import {IERC20} from "./IT.sol";
import "./IT.sol";

error Oi();

interface IT {
    struct uint2566 {
        string id;
        uint2344 id;
    }

    function oi() {
        uint2566 i;
    }
}

contract ReceiveEther {
    string sdf = "This is /*not*/ a comment";
    uint256[] i;
    /*sdfsdf*/ uint j;

    /*
    Which function is called, fallback() or receive()?

           send Ether
               |
         msg.data is empty?
              / \
            yes  no
            /     \
    receive() exists?  fallback()
         /   \
        yes   no
        /      \
    receive()   fallback()
    */

    // Function to receive Ether. msg.data must be empty
    receive() external payable {}

    // Fallback function is called when msg.data is not empty
    fallback() external payable {}

    function getBalance() public view returns (uint256) {
        return address(this).balance;
    }
}

contract SendEther {
    function sendViaTransfer(address payable _to) public payable {
        // This function is no longer recommended for sending Ether.
        _to.transfer(msg.value);
    }

    function sendViaSend(address payable _to) public payable {
        // Send returns a boolean value indicating success or failure.
        // This function is not recommended for sending Ether.
        bool sent = _to.send(msg.value);
        require(sent, "Failed to send Ether");
    }

    function sendViaCall(address payable _to) public payable {
        // Call returns a boolean value indicating success or failure.
        // This is the current recommended method to use.
        (bool sent, bytes memory data) = _to.call{value: msg.value}("");
        require(sent, "Failed to send Ether");
    }
}

abstract contract Yds {}

import "../IT.sol";

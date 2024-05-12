// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.8;

contract Name {
    uint val;

    function destructuringAssignments() public pure {
        (uint256 i, bool b, uint256 j) = returnMany();

        // Values can be left out.
        (uint256 x, , uint256 y) = (4, 5, 6);

        return (i, b, j, x, y);
    }

    function test() public {
        val = 123;
        bytes memory b = "";
        // address(0).call(b);
        // (uint sf, ) = oi();
        (bool success, bytes memory data) = msg.sender.delegatecall(b);
    }
}

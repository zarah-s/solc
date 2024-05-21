// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;
import "./test/files/vars/Error.sol";
import "./test/files/vars/Event.sol";

contract Test {
    function oi() external {}
}

interface IT {
    error ANOTHER_CUSTOM_ERROR(address, string);
    event Transfered(
        address indexed sender,
        address receiver,
        uint indexed amount
    );
}

library Lib {
    modifier onlyOwner(address owner) {
        require(msg.sender == owner, "Not owner");
        // Underscore is a special character only used inside
        // a function modifier and it tells Solidity to
        // execute the rest of the code.
        _;
    }

    function test(address payable _user) public {
        // val = 123;
        bytes memory b = "";
        _user.call(b);
    }

    enum Status {
        Success,
        Fail
    }

    error ANOTHER_CUSTOM_ERROR(address, string);
    uint constant vard = 1;

    // bytes[] constant bb = [""];
    // LibStr constant str = LibStr({name: "adsfs"});
    event Transfered(
        address indexed sender,
        address receiver,
        uint indexed amount
    );
    struct LibStr {
        address name;
    }
}

abstract contract FunctionModifier {
    // We will use these variables to demonstrate how to use
    // modifiers.
    address public owner;
    uint256 public x = 10;
    bool public locked;

    constructor() {
        // Set the transaction sender as the owner of the contract.
        owner = msg.sender;
    }

    // Modifier to check that the caller is the owner of
    // the contract.
    modifier onlyOwner() {
        require(msg.sender == owner, "Not owner");
        // Underscore is a special character only used inside
        // a function modifier and it tells Solidity to
        // execute the rest of the code.
        _;
    }

    // Modifiers can take inputs. This modifier checks that the
    // address passed in is not the zero address.
    modifier validAddress(address _addr) {
        require(_addr != address(0), "Not valid address");
        _;
    }

    struct Str {
        address name;
    }

    function changeOwner(
        address _newOwner,
        Str memory test,
        address oi
    ) public onlyOwner validAddress(address(0)) {
        test.name = msg.sender;
        oi = msg.sender;
        Test(oi).oi();
        _newOwner = address(0);
        owner = _newOwner;
    }

    // Modifiers can be called before and / or after a function.
    // This modifier prevents a function from being called while
    // it is still executing.
    modifier noReentrancy() {
        require(!locked, "No reentrancy");

        locked = true;
        _;
        locked = false;
    }

    event Transfered(
        address indexed sender,
        address receiver,
        uint indexed amount
    );
    event Eventdd(address indexed owner);

    error CUSTOM_ERROR();

    error ANOTHER_CUSTOM_ERROR(address, string);

    function decrement(uint256 i) public noReentrancy {
        x -= i;

        if (i > 1) {
            decrement(i - 1);
        }

        emit Transfered(msg.sender, address(0), 1);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.8;

contract MyTodo {
    error INSUFFICIENT_BALANCE();
    error ONLY_OWNER();
    struct addr {
        string name;
    }
    // address addr = address(0);
    Status status = Status.Idle;
    address public constant oi = address(0);
    string public immutable str = string("sdf");
    mapping(address => uint) name;
    Todo[] todos;
    uint[] javis = [1, 2];
    uint public deleted;

    function rtdfdf() external view returns (bool) {
        return true;
    }

    function structify(uint[2 * 10 ** 2] memory _id, Status _status) internal {
        Todo ffd = Tod(1, "title", "desc", 12122, Status.Idle, Tod([1, 2]));
        // if (true) {} else if (false) {
        //     addr = msg.sender;
        // } else {
        //     addr = msg.sender;
        // }
        // if (false) {}
        // name[msg.sender] = 5;
        require(rtdfdf());
        Status status = Status.Idle;

        uint[] nums = [1, 23, 4];
        delete nums;
        uint newNum = 10;
        delete newNum;

        Todo storage todo = todos[_id - 1];
        delete todo.id;
        delete ffd.tod;
        Todo memory ffh;
        ffh.id = 1;
        oi = msg.sender;
        todo.status = Status(_status);
        deleteTodo(2);
        uint test = structify(1, 2, 3, 4, 5);
    }

    enum Status {
        Idle,
        Pending,
        Done1
    }

    enum Gender {
        Male,
        Female
    }

    struct Todo {
        uint id;
        string title;
        string description;
        uint timestamp;
        Status status;
    }

    struct Tod {
        uint id;
        string title;
        string description;
        uint timestamp;
        Status status;
        Todo[] todos;
    }

    function createTodo(
        string calldata _title,
        string calldata _description
    ) external {
        todos.push(
            Todo({
                id: todos.length + 1,
                title: _title,
                description: _description,
                timestamp: block.timestamp,
                status: Status.Idle
            })
        );
    }

    function getTodos() external view returns (Todo[2 * 10 ** 2] memory) {
        Todo[] memory todos_ = new Todo[](todos.length - deleted);
        uint _count;
        uint _def;
        Status _ppd;
        for (uint i = 0; i < todos.length; i++) {
            if (todos[i].timestamp != 0) {
                todos_[_count] = todos[i];
                _count++;
            }
        }
        return todos_;
    }

    function deleteTodo(uint _id) external gasless {
        require(_id > 0, "Invalid id");
        require(_id - 1 < todos.length, "Invalid id");
        delete todos[_id - 1 * (2 - 1)];
        deleted++;
    }
}

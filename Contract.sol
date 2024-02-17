// SPDX-License-Identifier: MIT
pragma solidity ^0.8.8;

contract MyTodo {
    Todo[] todos;
    uint public deleted;

    function structify(uint _id, Status _status) external {
        Todo storage todo = todos[_id - 1];
        todo.status = Status(_status);
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

    function getTodos() external view returns (Todo[] memory) {
        Todo[] memory todos_ = new Todo[](todos.length - deleted);
        uint _count;
        for (uint i = 0; i < todos.length; i++) {
            if (todos[i].timestamp != 0) {
                todos_[_count] = todos[i];
                _count++;
            }
        }
        return todos_;
    }

    function deleteTodo(uint _id) external {
        require(_id > 0, "Invalid id");
        require(_id - 1 < todos.length, "Invalid id");
        delete todos[_id - 1];
        deleted++;
    }
}

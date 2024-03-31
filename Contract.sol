// SPDX-License-Identifier: MIT
pragma solidity ^0.8.8;

contract MyTodo {
    error INSUFFICIENT_BALANCE();
    error ONLY_OWNER();
    // struct addr {
    //     string name;
    // }

    bool boolean = retBool();

    // address addr = address(0);
    Status status = Status.Idle;
    address public oi = (address(0));
    Tod tod;
    address ford;
    mapping(address => uint) balanceOf;

    string public str = string("sdf");
    mapping(address => mapping(address => mapping(string => uint))) name;
    Todo[] todos;
    uint[] javis = [1, 2];
    uint public deleted;

    function tupple2(uint num) private view returns (bool, string memory) {
        return (true, "hello");
    }

    function rtdfdf2() private view returns (bool, string[1] memory) {
        (bool name, string memory janet) = tupple(5);

        Tod storage __todd = tod;
        uint newNum = 10;
        delete newNum;
        return (true, ["Hello"]);
    }

    function retBool2() internal view returns (bool) {
        return false;
    }

    address addr;

    function structify(
        uint[2 * 10 ** 2] calldata _id,
        Status _status
    ) external {
        uint _id = 1;
        string memory ddf = "sdfsd";
        addr = msg.sender;
        //
        if ((true && true) || 1 == 1) {
            // Todo memory ffd = Tod(1, "title", "desc", 12122, Status.Idle, []);
            if (true) {
                addr = msg.sender;
                if (false && 1 != 1) {
                    addr = address(0);
                }
            }
            // Todo memory ffd = Tod(1, "title", "desc", 12122, Status.Idle, []);
        } else if (false) {
            // Todo memory ffd = Tod(1, "title", "desc", 12122, Status.Idle, []);
            addr = msg.sender;

            // addr = msg.sender;
        } else if (true) {
            // Todo memory ffd = Tod(1, "title", "desc", 12122, Status.Idle, []);
            addr = msg.sender;

            // addr = msg.sender;
        } else {
            retBool();
            addr = msg.sender;
        }
        if (false) {}
        // name[msg.sender] = 5;
        // require(rtdfdf());
        status = Status.Idle;

        // uint[3]          nums = [1, 23, 4];
        // delete nums;
        uint newNum = 10;
        delete newNum;

        Todo storage todo_ = todos[1];
        delete todo_.id;
        // delete ffd.tod;
        Todo memory ffh;
        // ffh = Tod(1, "title", "desc", 12122, Status.Idle, Tod([1, 2]));
        oi = msg.sender;
        todo_.status = Status(_status);
        // deleteTodo(2);
        // uint test = structify(1, 2, 3, 4, 5);
    }

    function tupple(uint num) private view returns (bool, string memory) {
        return (true, "hello");
    }

    function rtdfdf() private view returns (bool, string[1] memory) {
        (bool name, string memory janet) = tupple(5);

        Tod storage __todd = tod;
        uint newNum = 10;
        delete newNum;
        return (true, ["Hello"]);
    }

    function retBool() internal view returns (bool) {
        return false;
    }

    // address addr;

    function structify2(
        uint[2 * 10 ** 2] calldata _id,
        Status _status,
        string memory __arg
    ) external {
        uint __id = 1;
        string memory ddf = "sdfsd";
        addr = msg.sender;
        //
        if ((true && true) || __id == 1) {
            // __id =10;
            string memory ___arg;
            ___arg = "Hello";
            // Todo memory ffd = Tod(1, "title", "desc", 12122, Status.Idle, []);
            if (true) {
                addr = msg.sender;
                if (false && 1 != 1) {
                    addr = address(0);
                }
            }
            // Todo memory ffd = Tod(1, "title", "desc", 12122, Status.Idle, []);
        } else if (false) {
            // Todo memory ffd = Tod(1, "title", "desc", 12122, Status.Idle, []);
            addr = msg.sender;

            // addr = msg.sender;
        } else if (true) {
            // Todo memory ffd = Tod(1, "title", "desc", 12122, Status.Idle, []);
            addr = msg.sender;

            // addr = msg.sender;
        } else {
            retBool();
            addr = msg.sender;
        }
        if (false) {}
        // name[msg.sender] = 5;
        // require(rtdfdf());
        status = Status.Idle;

        // uint[3]          nums = [1, 23, 4];
        // delete nums;
        uint newNum = 10;
        delete newNum;

        Todo storage todo_ = todos[1];
        delete todo_.id;
        // delete ffd.tod;
        Todo memory ffh;
        // ffh = Tod(1, "title", "desc", 12122, Status.Idle, Tod([1, 2]));
        oi = msg.sender;
        todo_.status = Status(_status);
        // deleteTodo(2);
        // uint test = structify(1, 2, 3, 4, 5);
    }

    enum Status {
        Idle,
        Pending,
        Done
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
        string memory _description
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
        uint _def;

        while (true) {
            _def++;
        }
        Status _ppd;
        for (uint i = 0; i < todos.length; i++) {
            // if (todos[i].timestamp != 0) {
            todos_[_count] = todos[i];
            _count++;
            // }
        }
        return todos_;
    }

    function deleteTodo(uint _id)  view returns(uint) gasless {
        require(_id > 0, "Invalid id");
        require(_id - 1 < todos.length, "Invalid id");
        delete todos[_id - 1 * (2 - 1)];
        deleted++;
    }
}

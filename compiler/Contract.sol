// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
import "./interfaces/IRealEstate.sol";

contract Dao {
    address superior;
    address nextSuperior;
    address owner;

    address realEstateContractAddress;

    struct Listing {
        address owner;
        address agentId;
        string country;
        string state;
        string city;
        string estateAddress;
        uint24 postalCode;
        string description;
        uint256 price;
        string images;
        string features;
        string coverImage;
        string id;
    }

    struct Agent {
        address id;
        string name;
        string code;
        string region;
        string bio;
        bool deleted;
    }

    struct Administration {
        address superior;
        address nextSuperior;
        string state;
        string region;
        Agent[] agents;
    }

    struct Assign {
        Listing listing;
        uint timestamp;
        uint id;
        bool approved;
    }

    mapping(string => Administration) administration;

    mapping(string => Assign[]) assign;

    constructor(address _realEstateContractAddress) {
        realEstateContractAddress = _realEstateContractAddress;
    }

    function transferStateSuperior(
        string calldata state,
        address _nextSuperior
    ) external {
        Administration storage _administration = administration[state];
        require(msg.sender == _administration.superior, "UNAUTHORIZED");
        _administration.nextSuperior = _nextSuperior;
    }

    function claimStateSuperior(string calldata state) external {
        Administration storage _administration = administration[state];
        require(msg.sender == _administration.nextSuperior, "UNAUTHORIZED");
        _administration.superior = msg.sender;
        _administration.nextSuperior = address(0);
    }

    function transferSuperior(address _nextSuperior) external {
        require(msg.sender == superior, "UNAUTHORIZED");
        nextSuperior = _nextSuperior;
    }

    function claimSuperior() external {
        require(nextSuperior == msg.sender, "UNAUTHORIZED");
        superior = msg.sender;
        nextSuperior = address(0);
    }

    function createAdministration(
        address _administrationSuperior,
        string calldata _state,
        string calldata _region
    ) external {
        require(msg.sender == superior, "UNAUTHORIZED");
        require(_administrationSuperior != address(0), "INVALID_ADDRESS");
        require(
            keccak256(abi.encode(_state)) != keccak256(abi.encode("")),
            "INVALID_STATE_FIELD"
        );
        require(
            keccak256(abi.encode(_region)) != keccak256(abi.encode("")),
            "INVALID_REGION_FIELD"
        );
        Administration storage _administration = administration[_state];
        require(
            keccak256(abi.encode(_administration.region)) !=
                keccak256(abi.encode(_region)),
            "ALREDY_EXIST"
        );
        _administration.superior = _administrationSuperior;
        _administration.state = _state;
        _administration.region = _region;
    }

    function addAgent(string calldata _state, Agent memory _agent) external {
        Administration storage _administration = administration[_state];
        require(msg.sender == _administration.superior, "UNAUTHORIZED");
        require(_agent.id == address(0), "INVALID_ADDRESS");
        require(
            keccak256(abi.encode(_agent.name)) != keccak256(abi.encode("")),
            "INVALID_NAME_FIELD"
        );
        require(
            keccak256(abi.encode(_agent.code)) != keccak256(abi.encode("")),
            "INVALID_CODE_FIELD"
        );
        require(
            keccak256(abi.encode(_agent.region)) != keccak256(abi.encode("")),
            "INVALID_REGION_FIELD"
        );
        require(
            keccak256(abi.encode(_agent.bio)) != keccak256(abi.encode("")),
            "INVALID_BIO_FIELD"
        );

        bool exist;
        for (uint i = 0; i < _administration.agents.length; i++) {
            if (_administration.agents[i].id == _agent.id) {
                exist = true;
            }

            if (
                keccak256(abi.encode(_administration.agents[i].code)) ==
                keccak256(abi.encode(_agent.code))
            ) {
                exist = true;
            }
        }
        require(!exist, "AGENT_ALREADY_EXIST(CODE_OR_ADDRESS)");
        require(
            keccak256(abi.encode(_administration.region)) !=
                keccak256(abi.encode(_agent.region)),
            "REGION_DID_NOT_MATCH"
        );
        _agent.deleted = false;
        _administration.agents.push(_agent);
    }

    function delegateListingForApproval(
        string calldata _state,
        bytes32 hash,
        Listing calldata _listing
    ) external {
        require(msg.sender == owner, "UNAUTHORIZED");
        Administration storage _administration = administration[_state];

        require(_administration.superior != address(0), "STATE_NOT_REGISTERED");
        bool isValidAgent;

        for (uint i; i < _administration.agents.length; i++) {
            if (_administration.agents[i].id == _listing.agentId) {
                isValidAgent = true;
            }
        }
        require(isValidAgent, "NOT_A_VALID_AGENT");
        uint id = assign[_state].length;
        assign[_state].push(
            Assign({
                timestamp: block.timestamp,
                listing: _listing,
                id: id + 1,
                approved: false
            })
        );

        IRealEstate(realEstateContractAddress).queListingForApproval(
            _listing.id,
            hash,
            _administration.superior
        );
    }

    function approveListing(
        string calldata _state,
        uint assignId,
        string calldata listingId
    ) external {
        Administration storage _administration = administration[_state];
        require(msg.sender == _administration.superior, "UNAUTHORIZED");

        {
            Assign[] memory _assign = assign[_state];

            require(
                _assign.length > 0 && _assign.length >= assignId - 1,
                "INVALID_ASSIGN_ID"
            );
        }
        Assign storage _asign = assign[_state][assignId - 1];
        require(
            keccak256(abi.encode(_asign.listing.state)) ==
                keccak256(abi.encode(_state)),
            "STATE_DID_NOT_MATCH"
        );

        require(
            keccak256(abi.encode(_asign.listing.id)) ==
                keccak256(abi.encode(listingId)),
            "CORRUPTED_DATA"
        );

        _asign.approved = true;

        IRealEstate(realEstateContractAddress).createListing(
            _asign.listing.id,
            _asign.listing.owner,
            _asign.listing.agentId,
            _asign.listing.country,
            _state,
            _asign.listing.city,
            _asign.listing.estateAddress,
            _asign.listing.postalCode,
            _asign.listing.description,
            _asign.listing.price,
            _asign.listing.images,
            _asign.listing.coverImage,
            _asign.listing.features
        );
    }

    function getUnApprovedAssigns(
        string calldata _state
    ) external view returns (Assign[] memory) {
        uint count;
        {
            for (uint i; i < assign[_state].length; i++) {
                if (!assign[_state][i].approved) {
                    count += 1;
                }
            }
        }

        Assign[] memory _return = new Assign[](count);

        {
            uint current_index;
            for (uint i; i < assign[_state].length; i++) {
                if (!assign[_state][i].approved) {
                    _return[current_index] = assign[_state][i];
                    current_index += 1;
                }
            }
        }

        return _return;
    }

    function getApprovedAssigns(
        string calldata _state
    ) external view returns (Assign[] memory) {
        uint count;
        {
            for (uint i; i < assign[_state].length; i++) {
                if (assign[_state][i].approved) {
                    count += 1;
                }
            }
        }

        Assign[] memory _return = new Assign[](count);

        {
            uint current_index;
            for (uint i; i < assign[_state].length; i++) {
                if (assign[_state][i].approved) {
                    _return[current_index] = assign[_state][i];
                    current_index += 1;
                }
            }
        }

        return _return;
    }
}

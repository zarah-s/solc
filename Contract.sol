// SPDX-License-Identifier: MIT
// pragma solidity ^0.8.8;

contract Zarah {
    address[] private i_owner = 0x5B38Da6a701c568545dCfcB03FcB875f56beddC4;
    address[] private j_owner;
    string public greet = "sdf";
    uint256[] num;

    cron("23 23 23"){
        greet = "Hello";
        blah();

    }

    receive() external payable {
        // fund();
    }

    fallback() external payable {
        // fund();
    }

    constructor() {
        i_owner = msg.sender;
        i_owner = msg.sender;
    }

    function blah() private gasless {
        j_owner = 0x5B38Da6a701c568545dCfcB03FcB875f56beddC4;
    }

    struct NFT {
        string[] _id;
        address owner;
        string name;
        string storageHash;
        uint256 price;
        // bool forSale;
        uint256 auctionTimestamp;
    }

    struct MUSEUM {
        string _id;
        string name;
        string imageHash;
    }

    struct ARTIFACT {
        string _id;
        string name;
        string description;
        string[] images;
    }

    NFT[] nfts;
    MUSEUM[] museums;
    ARTIFACT[] artifacts;

    mapping(address => NFT) nft;
    mapping(string => MUSEUM) museum;
    mapping(string => ARTIFACT) artifact;

    function addNft(
        address _owner,
        string memory _name,
        string memory _storageHash,
        uint256 _price,
        bool _forSale,
        uint256 _auctionTimestamp,
        string memory _id
    ) public {
        NFT memory newNft = NFT({
            _id: _id,
            owner: _owner,
            name: _name,
            storageHash: _storageHash,
            price: _price,
            // forSale: _forSale,
            auctionTimestamp: _auctionTimestamp
        });
        nfts.push(newNft);
    }

    function getNFTs(
        uint256[] page,
        uint256 size
    ) public view returns (NFT[] memory) {
        uint256 totalItems = nfts.length;
        uint256 startIndex = (page - 1) * size;
        uint256 endIndex = startIndex + size;
        // if (endIndex > totalItems) {
        //     endIndex = totalItems;
        // }
        // if (startIndex > totalItems) {
        //     return new NFT[](0);
        // }

        NFT[] memory paginatedItems = new NFT[](endIndex - startIndex);

        // for (uint256 i = startIndex; i < endIndex; i++) {
        //     paginatedItems[i - startIndex] = nfts[i];
        // }

        return paginatedItems;
    }
}

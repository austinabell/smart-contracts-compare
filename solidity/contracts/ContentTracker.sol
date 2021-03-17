// SPDX-License-Identifier: GPL-3.0

pragma solidity ^0.8.0;

/**
 * @title ContentTracker
 * @dev Plutocratic hosting of content through the contract.
 */
contract ContentTracker {
    struct ContentRecord {
        uint price;
        string content;
        address payable owner;
    }

    mapping(string => ContentRecord) values;

    address payable contractOwner;

    modifier onlyOwner {
        require(msg.sender == contractOwner, "Sender is not the contract owner");
        _;
    }

    constructor() {
        contractOwner = payable(msg.sender);
    }

    /**
     * @dev Store value in variable
     * @param route for content to purchase
     * @param content for data to place if purchase successful
     */
    function purchase(string calldata route, string calldata content) payable public {
        // Check funds sent > 0
        require(msg.value > 0, "Must provide value to purchase route");
        
        // Check entry if exists, error if not enough sent
        ContentRecord storage record = values[route];
        
        require(msg.value > record.price, "Not enough funds sent to purchase route");
        
        if (record.price > 0) {
            // Record has previous owner, return funds
            record.owner.transfer(record.price);
        }
        
        // Update content from reference. (all mapping values are already zero initialized)
        record.price = msg.value;
        record.content = content;
        record.owner = payable(msg.sender);
    }
    
    /**
     * @dev Withdraw funds from the contract
     */
    function withdraw() public onlyOwner {
        contractOwner.transfer(address(this).balance);
    }

    /**
     * @dev Return content for given route
     * @param route to retrieve content
     * @return content data
     */
    function getRoute(string calldata route) public view returns (string memory) {
        return values[route].content;
    }
}
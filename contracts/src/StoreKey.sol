// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

contract StoreKey is Ownable {
    mapping(address => bytes) private addressToSeedPhrase;

    constructor() Ownable(msg.sender) {}

    function storeMyEncryptedSeedPhrase(bytes calldata encryptedSeedPhrase) public {
        addressToSeedPhrase[msg.sender] = encryptedSeedPhrase;
    }

    function viewMyEncryptedSeedPhrase(address _address) public view returns (bytes memory) {
        return addressToSeedPhrase[_address];
    }

    receive() external payable {
        (bool success,) = owner().call{value: msg.value}("");
        require(success);
    }
}

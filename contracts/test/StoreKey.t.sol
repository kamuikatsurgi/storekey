// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import {Test, Vm, console} from "forge-std/Test.sol";
import {StoreKey} from "../src/StoreKey.sol";

contract StoreKeyTest is Test {
    Vm.Wallet ownerWallet;

    StoreKey test;

    function setUp() public {
        ownerWallet = vm.createWallet(uint256(keccak256(bytes("owner"))));

        vm.prank(ownerWallet.addr);
        test = new StoreKey();
    }

    function test_owner() public view {
        assertEq(test.owner(), ownerWallet.addr);
    }

    function testFuzz_storeMyEncryptedSeedPhrase_success(address _address, bytes calldata encryptedSeedPhrase) public {
        vm.prank(_address);
        test.storeMyEncryptedSeedPhrase(encryptedSeedPhrase);

        emit log_bytes(encryptedSeedPhrase);
    }

    function testFuzz_viewMyEncryptedSeedPhrase_success(address _address, bytes calldata encryptedSeedPhrase) public {
        vm.prank(_address);
        test.storeMyEncryptedSeedPhrase(encryptedSeedPhrase);

        emit log_bytes(encryptedSeedPhrase);

        bytes memory _encryptedSeedPhrase = test.viewMyEncryptedSeedPhrase(_address);

        assertEq(encryptedSeedPhrase, _encryptedSeedPhrase);
    }
}

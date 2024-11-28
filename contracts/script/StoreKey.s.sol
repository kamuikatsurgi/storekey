// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import {Script, console} from "forge-std/Script.sol";
import {StoreKey} from "../src/StoreKey.sol";

contract StoreKeyScript is Script {
    StoreKey public storekey;

    function setUp() public {}

    function run() public {
        vm.startBroadcast();
        vm.stopBroadcast();
    }
}

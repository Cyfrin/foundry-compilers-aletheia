// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import {ReverseCounter} from "contracts/ReverseCounter.sol";
import {Test} from "forge-std/Test.sol";

// Solidity tests are compatible with foundry, so they
// use the same syntax and offer the same functionality.

contract ReverseCounterTest is Test {
    ReverseCounter counter;

    function setUp() public {
        counter = new ReverseCounter(address(0));
    }

    function test_InitialValue() public view {
        require(counter.x() == 0, "Initial value should be 0");
    }
}


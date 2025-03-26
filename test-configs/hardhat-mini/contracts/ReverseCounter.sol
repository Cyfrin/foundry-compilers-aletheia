// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import {Counter} from "contracts/Counter.sol";

contract ReverseCounter {
    uint public x;
    Counter counter;

    event Decrement(uint by);

    constructor(address originalCounter) {
        counter = Counter(originalCounter);
    }

    function dec() public {
        counter.inc();
    }

    function decBy(uint by) public {
        emit Decrement(by);
    }
}

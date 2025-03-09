// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Timer {
    uint256 public timer;

    function setTimer(uint256 newTimer) public {
        timer = newTimer;
    }

    function increment() public {
        timer++;
    }
}

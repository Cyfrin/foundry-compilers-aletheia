// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import {console2} from "forge-std/src/console2.sol";

// import {console2} from "forge-std/console2.sol";
// NOTE: Above doesn't work yet with foundry-compilers but works with pnpm hardhat test
// Unfortunately we have to add 'src' in the path :(

contract Counter {
    uint public x;

    event Increment(uint by);

    function inc() public {
        console2.log("Hello");
        x++;
        emit Increment(1);
    }

    function incBy(uint by) public {
        require(by > 0, "incBy: increment should be positive");
        x += by;
        emit Increment(by);
    }
}

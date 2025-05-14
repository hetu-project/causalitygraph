// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Pausable.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/utils/Context.sol";

/**
 * @title Causality erc20 token is used for goverance in the Hetu Causality Graph ecosystem.
 * @dev {ERC20} token with:
 *  - ability to burn tokens
 *  - a minter role to mint tokens up to a capped total supply (4.2B)
 *  - a pauser role to stop all token transfers
 *  - a burner role to burn tokens from other accounts
 *  - role enumeration to list members of each role
 *  - initial allocation of 50M tokens to the deployer
 */
contract ERC20Base is ERC20, ERC20Burnable, ERC20Pausable, AccessControl {
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");
    bytes32 public constant BURNER_ROLE = keccak256("BURNER_ROLE");

    uint8 private _decimals;

    // Role enumeration: mapping from role to list of members
    mapping(bytes32 => address[]) private _roleMembers;
    mapping(bytes32 => mapping(address => bool)) private _isRoleMember;

    event RoleMemberAdded(bytes32 indexed role, address indexed account);
    event RoleMemberRemoved(bytes32 indexed role, address indexed account);

    constructor(string memory name, string memory symbol, uint8 decimals_)
        ERC20(name, symbol) {
        _grantRole(DEFAULT_ADMIN_ROLE, _msgSender());
        _grantRole(MINTER_ROLE, _msgSender());
        _grantRole(PAUSER_ROLE, _msgSender());
        _grantRole(BURNER_ROLE, _msgSender());
        _addRoleMember(DEFAULT_ADMIN_ROLE, _msgSender());
        _addRoleMember(MINTER_ROLE, _msgSender());
        _addRoleMember(PAUSER_ROLE, _msgSender());
        _addRoleMember(BURNER_ROLE, _msgSender());
        _setupDecimals(decimals_);
    }

    function _setupDecimals(uint8 decimals_) private {
        _decimals = decimals_;
    }

    function decimals() public view virtual override returns (uint8) {
        return _decimals;
    }

    function mint(address to, uint256 amount) public virtual {
        require(hasRole(MINTER_ROLE, _msgSender()), "ERC20Base: must have minter role to mint");
        _mint(to, amount);
    }

    function burnCoins(address from, uint256 amount) public virtual {
        require(hasRole(BURNER_ROLE, _msgSender()), "ERC20Base: must have burner role to burn");
        _burn(from, amount);
    }

    function pause() public virtual {
        require(hasRole(PAUSER_ROLE, _msgSender()), "ERC20Base: must have pauser role to pause");
        _pause();
    }

    function unpause() public virtual {
        require(hasRole(PAUSER_ROLE, _msgSender()), "ERC20Base: must have pauser role to unpause");
        _unpause();
    }

    function _grantRole(bytes32 role, address account) internal virtual override returns (bool) {
        super._grantRole(role, account);
        _addRoleMember(role, account);
        return true;
    }

    function _revokeRole(bytes32 role, address account) internal virtual override returns (bool) {
        super._revokeRole(role, account);
        _removeRoleMember(role, account);
        return true;
    }

    function _addRoleMember(bytes32 role, address account) private {
        if (!_isRoleMember[role][account]) {
            _roleMembers[role].push(account);
            _isRoleMember[role][account] = true;
            emit RoleMemberAdded(role, account);
        }
    }

    function _removeRoleMember(bytes32 role, address account) private {
        if (_isRoleMember[role][account]) {
            address[] storage members = _roleMembers[role];
            for (uint256 i = 0; i < members.length; i++) {
                if (members[i] == account) {
                    members[i] = members[members.length - 1];
                    members.pop();
                    break;
                }
            }
            _isRoleMember[role][account] = false;
            emit RoleMemberRemoved(role, account);
        }
    }

    function getRoleMembers(bytes32 role) public view returns (address[] memory) {
        return _roleMembers[role];
    }

    function getRoleMemberCount(bytes32 role) public view returns (uint256) {
        return _roleMembers[role].length;
    }

    /**
     * @dev Override _update to handle transfers, burns, and pausing logic.
     */
    function _update(address from, address to, uint256 amount) internal virtual override(ERC20, ERC20Pausable) {
        require(!paused(), "ERC20Pausable: token transfer while paused");
        super._update(from, to, amount);
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title ERC20FactoryWithInitialMint
 * @dev Factory contract for creating ERC20Base tokens
 * with optional initial minting functionality
 */
contract ERC20FactoryWithInitialMint is Ownable {
    // Array to track all deployed tokens
    address[] public deployedTokens;
    
    // Mapping from token address to creator address
    mapping(address => address) public tokenCreator;
    
    // Factory fee in wei (can be set by owner)
    uint256 public factoryFee;
    
    // Fee collector address
    address public feeCollector;
    
    // Events
    event TokenCreated(
        address indexed tokenAddress, 
        address indexed creator, 
        string name, 
        string symbol, 
        uint256 initialSupply
    );
    event FactoryFeeUpdated(uint256 oldFee, uint256 newFee);
    event FeeCollectorUpdated(address oldCollector, address newCollector);

    constructor(uint256 _initialFee, address _feeCollector) Ownable(msg.sender) {
        factoryFee = _initialFee;
        feeCollector = _feeCollector;
    }
    
    /**
     * @dev Creates a new ERC20Base token with optional initial supply
     * @param name Token name
     * @param symbol Token symbol
     * @param decimals Token decimals
     * @param initialSupply Initial amount to mint (in token units, not wei)
     * @param initialHolder Address to receive initially minted tokens
     * @return The address of the newly created token
     */
    function createToken(
        string memory name,
        string memory symbol,
        uint8 decimals,
        uint256 initialSupply,
        address initialHolder
    ) external payable returns (address) {
        // Check if fee is paid
        require(msg.value >= factoryFee, "ERC20Factory: fee not paid");
        
        // Create new token
        ERC20Base token = new ERC20Base(
            name,
            symbol,
            decimals
        );
        
        // Mint initial supply if needed
        if (initialSupply > 0) {
            address initialMintReceiver = initialHolder == address(0) ? msg.sender : initialHolder;
            token.mint(initialMintReceiver, initialSupply * (10 ** decimals));
        }
        
        // Register token
        address tokenAddress = address(token);
        deployedTokens.push(tokenAddress);
        tokenCreator[tokenAddress] = msg.sender;
        
        // Forward fee to collector
        if (factoryFee > 0 && feeCollector != address(0)) {
            (bool sent, ) = feeCollector.call{value: msg.value}("");
            require(sent, "Failed to send fee");
        }
        
        emit TokenCreated(tokenAddress, msg.sender, name, symbol, initialSupply);
        
        return tokenAddress;
    }
    
    /**
     * @dev Updates the factory fee
     * @param newFee New fee amount in wei
     */
    function setFactoryFee(uint256 newFee) external onlyOwner {
        uint256 oldFee = factoryFee;
        factoryFee = newFee;
        emit FactoryFeeUpdated(oldFee, newFee);
    }
    
    /**
     * @dev Updates the fee collector address
     * @param newCollector New fee collector address
     */
    function setFeeCollector(address newCollector) external onlyOwner {
        address oldCollector = feeCollector;
        feeCollector = newCollector;
        emit FeeCollectorUpdated(oldCollector, newCollector);
    }
    
    /**
     * @dev Gets the total number of deployed tokens
     * @return The count of deployed tokens
     */
    function getDeployedTokensCount() external view returns (uint256) {
        return deployedTokens.length;
    }
    
    /**
     * @dev Gets all deployed tokens
     * @return Array of token addresses
     */
    function getAllDeployedTokens() external view returns (address[] memory) {
        return deployedTokens;
    }
    
    /**
     * @dev Checks if an address is a token created by this factory
     * @param tokenAddress The address to check
     * @return True if the address is a token created by this factory
     */
    function isTokenFromFactory(address tokenAddress) external view returns (bool) {
        return tokenCreator[tokenAddress] != address(0);
    }
} 
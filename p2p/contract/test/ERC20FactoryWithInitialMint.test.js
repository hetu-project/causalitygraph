const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("ERC20FactoryWithInitialMint", function () {
    let Factory, factory, deployer, user1, user2, feeCollector;
    const INITIAL_FEE = ethers.parseEther("0.1"); // 0.1 ETH

    beforeEach(async function () {
        [deployer, user1, user2, feeCollector] = await ethers.getSigners();

        // Deploy factory contract
        Factory = await ethers.getContractFactory("ERC20FactoryWithInitialMint");
        factory = await Factory.deploy(INITIAL_FEE, feeCollector.address);
    });

    describe("Deployment", function () {
        it("Should set the correct initial parameters", async function () {
            expect(await factory.owner()).to.equal(deployer.address);
            expect(await factory.factoryFee()).to.equal(INITIAL_FEE);
            expect(await factory.feeCollector()).to.equal(feeCollector.address);
        });

        it("Should have zero deployed tokens initially", async function () {
            expect(await factory.getDeployedTokensCount()).to.equal(0);
            const tokens = await factory.getAllDeployedTokens();
            expect(tokens).to.be.an('array').that.is.empty;
        });
    });

    describe("Fee Management", function () {
        it("Should allow owner to update fee", async function () {
            const newFee = ethers.parseEther("0.2");

            await expect(factory.setFactoryFee(newFee))
                .to.emit(factory, "FactoryFeeUpdated")
                .withArgs(INITIAL_FEE, newFee);

            expect(await factory.factoryFee()).to.equal(newFee);
        });

        it("Should not allow non-owner to update fee", async function () {
            await expect(factory.connect(user1).setFactoryFee(ethers.parseEther("0.2")))
                .to.be.revertedWithCustomError(factory, "OwnableUnauthorizedAccount")
                .withArgs(user1.address);
        });

        it("Should allow owner to update fee collector", async function () {
            const newCollector = user2.address;

            await expect(factory.setFeeCollector(newCollector))
                .to.emit(factory, "FeeCollectorUpdated")
                .withArgs(feeCollector.address, newCollector);

            expect(await factory.feeCollector()).to.equal(newCollector);
        });

        it("Should not allow non-owner to update fee collector", async function () {
            await expect(factory.connect(user1).setFeeCollector(user1.address))
                .to.be.revertedWithCustomError(factory, "OwnableUnauthorizedAccount")
                .withArgs(user1.address);
        });
    });

    describe("Token Creation", function () {
        const TOKEN_NAME = "Test Token";
        const TOKEN_SYMBOL = "TST";
        const TOKEN_DECIMALS = 18;
        const INITIAL_SUPPLY = ethers.parseUnits("1000000", 18); // 1M tokens
        let tokenAddress;

        it("Should require fee payment", async function () {
            await expect(factory.createToken(
                TOKEN_NAME,
                TOKEN_SYMBOL,
                TOKEN_DECIMALS,
                INITIAL_SUPPLY,
                ethers.ZeroAddress
            )).to.be.revertedWith("ERC20Factory: fee not paid");

            // Should work with correct fee
            await expect(factory.createToken(
                TOKEN_NAME,
                TOKEN_SYMBOL,
                TOKEN_DECIMALS,
                INITIAL_SUPPLY,
                ethers.ZeroAddress,
                { value: INITIAL_FEE }
            )).to.not.be.reverted;
        });

        it("Should create a new token with initial supply", async function () {
            const tx = await factory.createToken(
                TOKEN_NAME,
                TOKEN_SYMBOL,
                TOKEN_DECIMALS,
                INITIAL_SUPPLY,
                user1.address,
                { value: INITIAL_FEE }
            );

            const receipt = await tx.wait();

            // Get token creation event
            const event = receipt.logs.find(
                (e) => e.fragment && e.fragment.name === "TokenCreated"
            );
            expect(event).to.not.be.undefined;

            // Get token address from event
            tokenAddress = event.args[0];

            // Check token was registered
            expect(await factory.getDeployedTokensCount()).to.equal(1);
            expect(await factory.tokenCreator(tokenAddress)).to.equal(deployer.address);

            // Check initial supply was minted to the correct account
            const Token = await ethers.getContractFactory("ERC20Base");
            const token = Token.attach(tokenAddress);

            // Check balance (should be initial supply * 10^decimals)
            const expectedBalance = INITIAL_SUPPLY * (10n ** BigInt(TOKEN_DECIMALS));
            expect(await token.balanceOf(user1.address)).to.equal(expectedBalance);
        });

        it("Should create a token with zero initial supply", async function () {
            const tx = await factory.createToken(
                TOKEN_NAME,
                TOKEN_SYMBOL,
                TOKEN_DECIMALS,
                0,
                ethers.ZeroAddress,
                { value: INITIAL_FEE }
            );

            const receipt = await tx.wait();

            // Get token address from event
            const event = receipt.logs.find(
                (e) => e.fragment && e.fragment.name === "TokenCreated"
            );
            tokenAddress = event.args[0];

            // Get token instance
            const Token = await ethers.getContractFactory("ERC20Base");
            const token = Token.attach(tokenAddress);

            // Total supply should be zero (since we didn't mint any)
            expect(await token.totalSupply()).to.equal(0);
        });

        it("Should transfer fee to collector", async function () {
            // Get initial balance
            const initialBalance = await ethers.provider.getBalance(feeCollector.address);

            // Create token with fee
            await factory.createToken(
                TOKEN_NAME,
                TOKEN_SYMBOL,
                TOKEN_DECIMALS,
                INITIAL_SUPPLY,
                user1.address,
                { value: INITIAL_FEE }
            );

            // Check fee was transferred to collector
            const finalBalance = await ethers.provider.getBalance(feeCollector.address);
            expect(finalBalance - initialBalance).to.equal(INITIAL_FEE);
        });

        it("Should use sender as initial holder if none specified", async function () {
            const tx = await factory.createToken(
                TOKEN_NAME,
                TOKEN_SYMBOL,
                TOKEN_DECIMALS,
                INITIAL_SUPPLY,
                ethers.ZeroAddress, // zero address should default to sender
                { value: INITIAL_FEE }
            );

            const receipt = await tx.wait();
            const event = receipt.logs.find(
                (e) => e.fragment && e.fragment.name === "TokenCreated"
            );
            tokenAddress = event.args[0];

            // Check balance of sender
            const Token = await ethers.getContractFactory("ERC20Base");
            const token = Token.attach(tokenAddress);

            // Check balance (should be initial supply * 10^decimals)
            const expectedBalance = INITIAL_SUPPLY * (10n ** BigInt(TOKEN_DECIMALS));
            expect(await token.balanceOf(deployer.address)).to.equal(expectedBalance);
        });
    });

    describe("Token Listing", function () {
        beforeEach(async function () {
            // Create a few tokens
            await factory.createToken("Token1", "TK1", 18, 0, ethers.ZeroAddress, { value: INITIAL_FEE });
            await factory.connect(user1).createToken("Token2", "TK2", 6, 0, ethers.ZeroAddress, { value: INITIAL_FEE });
        });

        it("Should correctly list all created tokens", async function () {
            expect(await factory.getDeployedTokensCount()).to.equal(2);

            const tokens = await factory.getAllDeployedTokens();
            expect(tokens).to.have.lengthOf(2);
        });

        it("Should correctly identify tokens from factory", async function () {
            const tokens = await factory.getAllDeployedTokens();

            expect(await factory.isTokenFromFactory(tokens[0])).to.be.true;
            expect(await factory.isTokenFromFactory(tokens[1])).to.be.true;
            expect(await factory.isTokenFromFactory(deployer.address)).to.be.false;
            expect(await factory.isTokenFromFactory(ethers.ZeroAddress)).to.be.false;
        });
    });
}); 
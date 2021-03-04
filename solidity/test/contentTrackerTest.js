const ContentTracker = artifacts.require("ContentTracker");

contract("ContentTracker", async accounts => {
    it("Should return no content for a new deployment", async () => {
        const instance = await ContentTracker.deployed();
        const content = await instance.getRoute("test");
        assert.equal(content, "");
    });
    it("Should be able to purchase and replace", async () => {
        const instance = await ContentTracker.deployed();

        await instance.purchase("troute", "tcontent", { from: accounts[0], value: 2 });
        let content = await instance.getRoute("troute");
        assert.equal(content, "tcontent");

        try {
            // Should fail and revert if not enough value provided
            await instance.purchase("troute", "new content", { from: accounts[1], value: 2 });
            assert.fail(0, 1, "should have failed");
        } catch (error) {
            assert(error, "Expected an error but did not get one");
            assert(error.reason == "Not enough funds sent to purchase route");
        }

        await instance.purchase("troute", "new content", { from: accounts[1], value: 4 });
        content = await instance.getRoute("troute");
        assert.equal(content, "new content");

        await instance.withdraw({ from: accounts[0] });
    });
});

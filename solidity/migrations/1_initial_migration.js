const ContentTracker = artifacts.require("ContentTracker");

module.exports = function (deployer) {
  deployer.deploy(ContentTracker);
};

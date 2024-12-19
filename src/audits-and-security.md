# Audits and Security

The Kinode operating system runtime has been audited by [Enigma Dark](https://www.enigmadark.com/).
That report can be found [here](https://github.com/Enigma-Dark/security-review-reports/blob/main/2024-11-18_Architecture_Review_Report_Kinode.pdf).

However, the audit was not comprehensive and focused on the robustness of the networking stack and the kernel.
Therefore, other parts of the runtime, such as the filesystem modules and the ETH RPC layer, remain unaudited.
Kinode OS remains a work in progress and will continue to be audited as it matures.

### Smart Contracts

Kinode OS uses a number of smart contracts to manage global state.
[link to smart contract audit when finished]

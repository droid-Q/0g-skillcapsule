// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract SkillCapsuleRegistry {
    enum CapsuleStatus {
        Draft,
        Published,
        Paused
    }

    struct CapsuleRecord {
        address owner;
        string manifestRoot;
        uint256 agentTokenId;
        uint64 version;
        CapsuleStatus status;
        uint64 updatedAt;
    }

    mapping(bytes32 => CapsuleRecord) private capsules;

    event CapsuleUpserted(
        bytes32 indexed capsuleKey,
        address indexed owner,
        string manifestRoot,
        uint256 agentTokenId,
        uint64 version,
        CapsuleStatus status
    );

    event CapsuleStatusChanged(
        bytes32 indexed capsuleKey,
        CapsuleStatus status
    );

    function upsertCapsule(
        bytes32 capsuleKey,
        string calldata manifestRoot,
        uint256 agentTokenId,
        uint64 version,
        uint8 status
    ) external {
        require(capsuleKey != bytes32(0), "invalid capsule key");
        require(bytes(manifestRoot).length > 0, "manifest root required");
        require(status <= uint8(CapsuleStatus.Paused), "invalid status");

        CapsuleRecord storage record = capsules[capsuleKey];
        if (record.owner != address(0)) {
            require(record.owner == msg.sender, "only owner");
        }

        record.owner = msg.sender;
        record.manifestRoot = manifestRoot;
        record.agentTokenId = agentTokenId;
        record.version = version;
        record.status = CapsuleStatus(status);
        record.updatedAt = uint64(block.timestamp);

        emit CapsuleUpserted(
            capsuleKey,
            msg.sender,
            manifestRoot,
            agentTokenId,
            version,
            CapsuleStatus(status)
        );
    }

    function setCapsuleStatus(bytes32 capsuleKey, uint8 status) external {
        require(status <= uint8(CapsuleStatus.Paused), "invalid status");
        CapsuleRecord storage record = capsules[capsuleKey];
        require(record.owner != address(0), "capsule missing");
        require(record.owner == msg.sender, "only owner");

        record.status = CapsuleStatus(status);
        record.updatedAt = uint64(block.timestamp);

        emit CapsuleStatusChanged(capsuleKey, CapsuleStatus(status));
    }

    function getCapsule(bytes32 capsuleKey)
        external
        view
        returns (CapsuleRecord memory)
    {
        return capsules[capsuleKey];
    }
}


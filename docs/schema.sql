CREATE TABLE IF NOT EXISTS `users`(
    `userId` INT(11) NOT NULL AUTO_INCREMENT,
    `userName` VARCHAR(50) NOT NULL UNIQUE,
    `password` BINARY(32) NOT NULL,
    PRIMARY KEY (`userId`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS `groups` (
    `groupId` INT(11) NOT NULL AUTO_INCREMENT,
    `groupUuid` VARCHAR(32) NOT NULL,
    `groupName` VARCHAR(32) NOT NULL,
    `userName` VARCHAR(50) NOT NULL,
    `lastUpdate` DATETIME NOT NULL,
    PRIMARY KEY (`groupId`),
    INDEX (`lastUpdate`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS `userGroup` (
    `userId` INT(11) NOT NULL,
    `groupId` INT(11) NOT NULL,
    INDEX (`userId`, `groupId`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS `schedule` (
    `groupId` INT(11) NOT NULL,
    `userName` VARCHAR(50) NOT NULL,
    `since` DATE NOT NULL,
    `until` DATE NOT NULL,
    `condition` TINYINT(1) NOT NULL,
    `lastUpdate` DATETIME NOT NULL,
    PRIMARY KEY (`groupId`),
    INDEX (`lastUpdate`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

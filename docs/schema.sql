CREATE TABLE IF NOT EXISTS `schedule` (
    `groupId` VARCHAR(32) NOT NULL,
    `userName` VARCHAR(50) NOT NULL,
    `year` TINYINT(4) NOT NULL,
    `month` TINYINT(2) NOT NULL,
    `week` TINYINT(1) NOT NULL,
    `condition` TINYINT(1) NOT NULL,
    PRIMARY KEY (`groupId`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
CREATE TABLE `users` (
  `id` int NOT NULL AUTO_INCREMENT,
  `login` varchar(20) DEFAULT NULL,
  `pwdhash` char(87) DEFAULT NULL,
  `surname` varchar(100) DEFAULT NULL,
  `city` varchar(50) DEFAULT NULL,
  `hobby` varchar(300) DEFAULT NULL,
  `birthday` date DEFAULT NULL,
  `name` varchar(100) DEFAULT NULL,
	`session` varchar(30) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

CREATE TABLE `subscriptions` (
  `id` int NOT NULL AUTO_INCREMENT, -- subscription ID
  `subscriber_id` int NOT NULL,     -- one who is subscribed
	`author_id` int NOT NULL,         -- one who creates messages
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

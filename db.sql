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
  PRIMARY KEY (`id`),
	FOREIGN KEY (`author_id`)  REFERENCES `users` (`id`),
	FOREIGN KEY (`subscriber_id`)  REFERENCES `users` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

CREATE TABLE `perspages` (
  `id` int NOT NULL AUTO_INCREMENT, -- pers page ID
	`user_id` int NOT NULL,
	pers_page varchar(1024),
  PRIMARY KEY (`id`),
	FOREIGN KEY (`user_id`)  REFERENCES `users` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

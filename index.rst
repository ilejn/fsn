Домашнее задание №2
~~~~~~~~~~~~~~~~~~~

* https://github.com/ilejn/fsn , branch hw2
* Использованные данные - https://drive.google.com/file/d/1wRQfw5EYpzulvRfHCGIUWB2am5JUYVGk/view (предположительно датасет пользователей Facebook)
* 1000000 записей загружено в таблицу ext_users с помощью src/bin/loadpeople.rs

::

	mysql> show create table extusers \G
	*************************** 1. row ***************************
	       Table: extusers
	Create Table: CREATE TABLE `extusers` (
	  `name` varchar(100) DEFAULT NULL,
	  `surname` varchar(100) DEFAULT NULL,
	  `id` int NOT NULL AUTO_INCREMENT,
	  PRIMARY KEY (`id`),
	  KEY `li` (`name`,`surname`,`id`)
	) ENGINE=InnoDB AUTO_INCREMENT=1000001 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci
	1 row in set (0,01 sec)

* поиск выполнялся командой

::

	for THRD in 1 10 100 1000; do echo "== $THRD threads==" && wrk -c $THRD -t 1 -s lookup.lua http://localhost:8080/lookup; done

* для wrk использовался конфигурационный файл

::

	wrk.method = "POST"
	wrk.body   = "name=john&surname=a"
	wrk.headers["Content-Type"] = "application/x-www-form-urlencoded"

* route */lookup* вызывет функцию *lookup_user* в файле db.rs, которая содержит запрос "select name, surname, id from extusers where name like ? and surname like ? order by id limit 10000"
* эксприменты проводились с индексами по
	* name
	* surname
	* name, surname
	* surname, name
	* name, surname, id
* наилучшие результаты были получены с индексом по name, surname, id. Это обсусловлено поисковым запросом: name *john%* давало лучшую селективность, чем surname *a%*.
* отчет по latency и throughput можно найти в fsn/index.pdf

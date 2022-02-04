Домашнее задание №8
~~~~~~~~~~~~~~~~~~~

* https://github.com/ilejn/fsn , branch hw8
* docker-compose.yml совмещает
	* три инстанса приложение
	* nginx, проксирующий API
  * xtradb-cluster с тремя инстансами
	* haproxy, проксирующий доступ приложения к mysql

Ход исполнения работы
~~~~~~~~~~~~~~~~~~~~~

* создал через UI запись о пользователе compose/compose
* убедился в наличии строки в таблице users во всех инстансах mysql и в работе haproxy

::
  $ mysql --port=13306 --host=localhost --protocol=tcp --user=root
  Welcome to the MySQL monitor.  Commands end with ; or \g.
  Your MySQL connection id is 24
  Server version: 8.0.26-16.1 Percona XtraDB Cluster (GPL), Release rel16, Revision b141904, WSREP version 26.4.3

  Copyright (c) 2000, 2021, Oracle and/or its affiliates.

  Oracle is a registered trademark of Oracle Corporation and/or its
  affiliates. Other names may be trademarks of their respective
  owners.

  Type 'help;' or '\h' for help. Type '\c' to clear the current input statement.

  mysql> use test;
  Reading table information for completion of table and column names
  You can turn off this feature to get a quicker startup with -A

  Database changed
  mysql> SELECT @@hostname;
  +--------------+
  | @@hostname   |
  +--------------+
  | d630cd1fb563 |
  +--------------+
  1 row in set (0,00 sec)

  mysql> select name, surname from users;
  +---------+---------+
  | name    | surname |
  +---------+---------+
  | compose | compose |
  +---------+---------+
  1 row in set (0,00 sec)

  mysql> ^DBye
  $ mysql --port=13306 --host=localhost --protocol=tcp --user=root
  Welcome to the MySQL monitor.  Commands end with ; or \g.
  Your MySQL connection id is 24
  Server version: 8.0.26-16.1 Percona XtraDB Cluster (GPL), Release rel16, Revision b141904, WSREP version 26.4.3

  Copyright (c) 2000, 2021, Oracle and/or its affiliates.

  Oracle is a registered trademark of Oracle Corporation and/or its
  affiliates. Other names may be trademarks of their respective
  owners.

  Type 'help;' or '\h' for help. Type '\c' to clear the current input statement.

  mysql> use test;
  Reading table information for completion of table and column names
  You can turn off this feature to get a quicker startup with -A

  Database changed
  mysql> select name, surname from users;
  +---------+---------+
  | name    | surname |
  +---------+---------+
  | compose | compose |
  +---------+---------+
  1 row in set (0,00 sec)

  mysql> SELECT @@hostname;
  +--------------+
  | @@hostname   |
  +--------------+
  | 4d2a09296ef2 |
  +--------------+
  1 row in set (0,00 sec)

* создал поток запросов командой

::

  watch curl -X POST -d 'login=compose' -d 'password=compose'   localhost:4088/signin

* убедился, что все работает. Судя по логам, запросы распределяются по разным инстансам приложения.

::

  fsn1             | [00:05:55][mio::poll][TRACE] registering with poller
  fsn1             | [00:05:55][actix_http::h1::decoder][TRACE] Length read: 30
  fsn1             | [00:05:55][fsn::crypto][INFO] hash $argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdA$sDSao/ZjLQphjGTa9WAhtGVkQIHqEdtVNdvSgfN3nRA
  fsn1             | [00:05:55][actix_web::middleware::logger][INFO] 172.23.0.3:51270 "POST /signin HTTP/1.0" 200 39 "-" "curl/7.74.0" 0.028429
  fsn1             | [00:05:55][mio::poll][TRACE] deregistering handle with poller
  nginx-lb         | 172.23.0.1 - - [04/Feb/2022:00:05:55 +0000] "POST /signin HTTP/1.1" 200 39 "-" "curl/7.74.0" "-"
  fsn2             | [00:05:55][mio::poll][TRACE] registering with poller
  fsn2             | [00:05:55][actix_http::h1::decoder][TRACE] Length read: 30
  fsn2             | [00:05:55][fsn::crypto][INFO] hash $argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdA$sDSao/ZjLQphjGTa9WAhtGVkQIHqEdtVNdvSgfN3nRA
  fsn2             | [00:05:55][actix_web::middleware::logger][INFO] 172.23.0.3:59040 "POST /signin HTTP/1.0" 200 39 "-" "curl/7.74.0" 0.027871
  nginx-lb         | 172.23.0.1 - - [04/Feb/2022:00:05:55 +0000] "POST /signin HTTP/1.1" 200 39 "-" "curl/7.74.0" "-"
  fsn2             | [00:05:55][mio::poll][TRACE] deregistering handle with poller
  fsn3             | [00:06:23][mio::poll][TRACE] registering with poller
  fsn3             | [00:06:23][actix_http::h1::decoder][TRACE] Length read: 30
  fsn3             | [00:06:23][fsn::crypto][INFO] hash $argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdA$sDSao/ZjLQphjGTa9WAhtGVkQIHqEdtVNdvSgfN3nRA
  fsn3             | [00:06:23][actix_web::middleware::logger][INFO] 172.23.0.3:36708 "POST /signin HTTP/1.0" 200 39 "-" "curl/7.74.0" 0.028111
  fsn3             | [00:06:23][mio::poll][TRACE] deregistering handle with poller
  nginx-lb         | 172.23.0.1 - - [04/Feb/2022:00:06:23 +0000] "POST /signin HTTP/1.1" 200 39 "-" "curl/7.74.0" "-"

* остановил один из контейнеров с xtradb

::

  $ docker-compose stop xtradb-slave2
  Stopping xtradb-slave2 ... done

  $ docker-compose ps
      Name                   Command                State                        Ports
  -------------------------------------------------------------------------------------------------------
  fsn1            ./target/release/fsn             Up         8080/tcp
  fsn2            ./target/release/fsn             Up         8080/tcp
  fsn3            ./target/release/fsn             Up         8080/tcp
  haproxy         docker-entrypoint.sh hapro ...   Up         0.0.0.0:13306->3306/tcp,:::13306->3306/tcp
  nginx-lb        /docker-entrypoint.sh ngin ...   Up         0.0.0.0:4088->80/tcp,:::4088->80/tcp,
                                                              0.0.0.0:4098->8080/tcp,:::4098->8080/tcp
  xtradb-master   /entrypoint.sh mysqld            Up         3306/tcp, 33060/tcp, 4444/tcp, 4567/tcp,
                                                              4568/tcp
  xtradb-slave1   /entrypoint.sh /bin/sh -c  ...   Up         3306/tcp, 33060/tcp, 4444/tcp, 4567/tcp,
                                                              4568/tcp
  xtradb-slave2   /entrypoint.sh /bin/sh -c  ...   Exit 137

* система продолжила работать (через некоторое время, были ошибки во время переключения haproxy)

::
  nginx-lb         | 172.23.0.1 - - [04/Feb/2022:06:54:24 +0000] "POST /signin HTTP/1.1" 200 39 "-" "curl/7.74.0" "-"
  xtradb-slave1    | 2022-02-04T06:54:25.296483Z 0 [Note] [MY-000000] [Galera] (ab3449da-9da2, 'ssl://0.0.0.0:4567') turning message relay requesting on, nonlive peers: ssl://172.23.0.7:4567
  xtradb-master    | 2022-02-04T06:54:25.296487Z 0 [Note] [MY-000000] [Galera] (9e3d445b-9a5b, 'ssl://0.0.0.0:4567') turning message relay requesting on, nonlive peers: ssl://172.23.0.7:4567
  xtradb-slave2 exited with code 137
  fsn2             | [06:54:26][fsn::crypto][INFO] hash $argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdA$sDSao/ZjLQphjGTa9WAhtGVkQIHqEdtVNdvSgfN3nRA
  xtradb-master    | 2022-02-04T06:54:26.453591Z 0 [Note] [MY-000000] [Galera] (9e3d445b-9a5b, 'ssl://0.0.0.0:4567') reconnecting to 443d2aa0-abba (ssl://172.23.0.7:4567), attempt 0
  xtradb-slave1    | 2022-02-04T06:54:26.666910Z 0 [Note] [MY-000000] [Galera] (ab3449da-9da2, 'ssl://0.0.0.0:4567') reconnecting to 443d2aa0-abba (ssl://172.23.0.7:4567), attempt 0
  nginx-lb         | 172.23.0.1 - - [04/Feb/2022:06:54:27 +0000] "POST /signin HTTP/1.1" 504 167 "-" "curl/7.74.0" "-"
  nginx-lb         | 2022/02/04 06:54:27 [warn] 38#38: *175 upstream server temporarily disabled while reading response header from upstream, client: 172.23.0.1, server: fsn-nginx-lb, request: "POST /signin HTTP/1.1", upstream: "http://172.23.0.5:8080/signin", host: "localhost:4088"
  nginx-lb         | 2022/02/04 06:54:27 [error] 38#38: *175 upstream timed out (110: Connection timed out) while reading response header from upstream, client: 172.23.0.1, server: fsn-nginx-lb, request: "POST /signin HTTP/1.1", upstream: "http://172.23.0.5:8080/signin", host: "localhost:4088"
  haproxy          | [WARNING]  (9) : Server servers/cl3 is DOWN, reason: Layer4 timeout, check duration: 501ms. 2 active and 0 backup servers left. 1 sessions active, 0 requeued, 0 remaining in queue.
  fsn2             | [06:54:29][fsn::crypto][INFO] hash $argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdA$sDSao/ZjLQphjGTa9WAhtGVkQIHqEdtVNdvSgfN3nRA

* остановил один из контейнеров с приложением

::

  $ docker-compose stop fsn2
  Stopping fsn2 ... done
  $ docker-compose ps
      Name                   Command                State                        Ports
  -------------------------------------------------------------------------------------------------------
  fsn1            ./target/release/fsn             Up         8080/tcp
  fsn2            ./target/release/fsn             Exit 0
  fsn3            ./target/release/fsn             Up         8080/tcp
  haproxy         docker-entrypoint.sh hapro ...   Up         0.0.0.0:13306->3306/tcp,:::13306->3306/tcp
  nginx-lb        /docker-entrypoint.sh ngin ...   Up         0.0.0.0:4088->80/tcp,:::4088->80/tcp,
                                                              0.0.0.0:4098->8080/tcp,:::4098->8080/tcp
  xtradb-master   /entrypoint.sh mysqld            Up         3306/tcp, 33060/tcp, 4444/tcp, 4567/tcp,
                                                              4568/tcp
  xtradb-slave1   /entrypoint.sh /bin/sh -c  ...   Up         3306/tcp, 33060/tcp, 4444/tcp, 4567/tcp,
                                                              4568/tcp
  xtradb-slave2   /entrypoint.sh /bin/sh -c  ...   Exit 137

* система продолжила работать, переключение произошло незаметно

::

  fsn3             | [06:58:17][fsn::crypto][INFO] hash $argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdA$sDSao/ZjLQphjGTa9WAhtGVkQIHqEdtVNdvSgfN3nRA
  fsn3             | [06:58:17][actix_web::middleware::logger][INFO] 172.23.0.3:36844 "POST /signin HTTP/1.0" 200 39 "-" "curl/7.74.0" 0.025189
  nginx-lb         | 172.23.0.1 - - [04/Feb/2022:06:58:17 +0000] "POST /signin HTTP/1.1" 200 39 "-" "curl/7.74.0" "-"
  fsn1             | [06:58:19][fsn::crypto][INFO] hash $argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdA$sDSao/ZjLQphjGTa9WAhtGVkQIHqEdtVNdvSgfN3nRA
  fsn1             | [06:58:19][actix_web::middleware::logger][INFO] 172.23.0.3:58480 "POST /signin HTTP/1.0" 200 39 "-" "curl/7.74.0" 0.023869
  nginx-lb         | 172.23.0.1 - - [04/Feb/2022:06:58:19 +0000] "POST /signin HTTP/1.1" 200 39 "-" "curl/7.74.0" "-"
  fsn2             | [06:58:21][fsn::crypto][INFO] hash $argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdA$sDSao/ZjLQphjGTa9WAhtGVkQIHqEdtVNdvSgfN3nRA
  fsn2             | [06:58:21][actix_web::middleware::logger][INFO] 172.23.0.3:51848 "POST /signin HTTP/1.0" 200 39 "-" "curl/7.74.0" 0.027214
  nginx-lb         | 172.23.0.1 - - [04/Feb/2022:06:58:21 +0000] "POST /signin HTTP/1.1" 200 39 "-" "curl/7.74.0" "-"
  fsn2             | [06:58:23][actix_server::builder][INFO] SIGTERM received, stopping
  fsn2             | [06:58:23][actix_server::worker][INFO] Shutting down worker, 0 connections
  fsn2             | [06:58:23][actix_server::worker][INFO] Shutting down worker, 0 connections
  fsn2             | [06:58:23][actix_server::worker][INFO] Shutting down worker, 0 connections
  fsn2             | [06:58:23][actix_server::worker][INFO] Shutting down worker, 0 connections
  fsn2             | [06:58:23][actix_server::worker][INFO] Shutting down worker, 0 connections
  fsn2             | [06:58:23][actix_server::worker][INFO] Shutting down worker, 0 connections
  fsn2             | [06:58:23][actix_server::worker][INFO] Shutting down worker, 0 connections
  fsn2             | [06:58:23][actix_server::worker][INFO] Shutting down worker, 0 connections
  fsn3             | [06:58:23][fsn::crypto][INFO] hash $argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdA$sDSao/ZjLQphjGTa9WAhtGVkQIHqEdtVNdvSgfN3nRA
  fsn3             | [06:58:23][actix_web::middleware::logger][INFO] 172.23.0.3:36846 "POST /signin HTTP/1.0" 200 39 "-" "curl/7.74.0" 0.022323
  nginx-lb         | 172.23.0.1 - - [04/Feb/2022:06:58:23 +0000] "POST /signin HTTP/1.1" 200 39 "-" "curl/7.74.0" "-"
  fsn2 exited with code 0
  fsn3             | [06:58:25][fsn::crypto][INFO] hash $argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdA$sDSao/ZjLQphjGTa9WAhtGVkQIHqEdtVNdvSgfN3nRA
  fsn3             | [06:58:25][actix_web::middleware::logger][INFO] 172.23.0.3:36848 "POST /signin HTTP/1.0" 200 39 "-" "curl/7.74.0" 0.027645
  nginx-lb         | 172.23.0.1 - - [04/Feb/2022:06:58:25 +0000] "POST /signin HTTP/1.1" 200 39 "-" "curl/7.74.0" "-"
  fsn3             | [06:58:27][fsn::crypto][INFO] hash $argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdA$sDSao/ZjLQphjGTa9WAhtGVkQIHqEdtVNdvSgfN3nRA
  fsn3             | [06:58:27][actix_web::middleware::logger][INFO] 172.23.0.3:36850 "POST /signin HTTP/1.0" 200 39 "-" "curl/7.74.0" 0.024875
  nginx-lb         | 172.23.0.1 - - [04/Feb/2022:06:58:27 +0000] "POST /signin HTTP/1.1" 200 39 "-" "curl/7.74.0" "-"
  fsn1             | [06:58:29][fsn::crypto][INFO] hash $argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdA$sDSao/ZjLQphjGTa9WAhtGVkQIHqEdtVNdvSgfN3nRA
  fsn1             | [06:58:29][actix_web::middleware::logger][INFO] 172.23.0.3:58482 "POST /signin HTTP/1.0" 200 39 "-" "curl/7.74.0" 0.024182
  nginx-lb         | 172.23.0.1 - - [04/Feb/2022:06:58:29 +0000] "POST /signin HTTP/1.1" 200 39 "-" "curl/7.74.0" "-"
  nginx-lb         | 2022/02/04 06:59:06 [error] 38#38: *407 connect() failed (113: No route to host) while connecting to upstream, client: 172.23.0.1, server: fsn-nginx-lb, request: "POST /signin HTTP/1.1", upstream: "http://172.23.0.5:8080/signin", host: "localhost:4088"
  nginx-lb         | 2022/02/04 06:59:06 [warn] 38#38: *407 upstream server temporarily disabled while connecting to upstream, client: 172.23.0.1, server: fsn-nginx-lb, request: "POST /signin HTTP/1.1", upstream: "http://172.23.0.5:8080/signin", host: "localhost:4088"
  fsn3             | [06:59:06][fsn::crypto][INFO] hash $argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdA$sDSao/ZjLQphjGTa9WAhtGVkQIHqEdtVNdvSgfN3nRA
  fsn3             | [06:59:06][actix_web::middleware::logger][INFO] 172.23.0.3:36852 "POST /signin HTTP/1.0" 200 39 "-" "curl/7.74.0" 0.025224
  nginx-lb         | 172.23.0.1 - - [04/Feb/2022:06:59:06 +0000] "POST /signin HTTP/1.1" 200 39 "-" "curl/7.74.0" "-"
  fsn1             | [06:59:08][fsn::crypto][INFO] hash $argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdA$sDSao/ZjLQphjGTa9WAhtGVkQIHqEdtVNdvSgfN3nRA
  fsn1             | [06:59:08][actix_web::middleware::logger][INFO] 172.23.0.3:58484 "POST /signin HTTP/1.0" 200 39 "-" "curl/7.74.0" 0.023986
  nginx-lb         | 172.23.0.1 - - [04/Feb/2022:06:59:08 +0000] "POST /signin HTTP/1.1" 200 39 "-" "curl/7.74.0" "-"
  fsn3             | [06:59:10][fsn::crypto][INFO] hash $argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdA$sDSao/ZjLQphjGTa9WAhtGVkQIHqEdtVNdvSgfN3nRA
  fsn3             | [06:59:10][actix_web::middleware::logger][INFO] 172.23.0.3:36856 "POST /signin HTTP/1.0" 200 39 "-" "curl/7.74.0" 0.023802
  nginx-lb         | 172.23.0.1 - - [04/Feb/2022:06:59:10 +0000] "POST /signin HTTP/1.1" 200 39 "-" "curl/7.74.0" "-"
  fsn1             | [06:59:12][fsn::crypto][INFO] hash $argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdA$sDSao/ZjLQphjGTa9WAhtGVkQIHqEdtVNdvSgfN3nRA
  fsn1             | [06:59:12][actix_web::middleware::logger][INFO] 172.23.0.3:58486 "POST /signin HTTP/1.0" 200 39 "-" "curl/7.74.0" 0.023044

/*
*     _____     ___    
*    / ___/__  / _/_ __
*   / /__/ _ \/ _/ // /
*   \___/\___/_/ \_, / 
*               /___/  
*
*   setup.sql: run this script to set up the db
*   returns: none, only run this script locally
*/

CREATE TABLE "cofy_server.ownership" ("ip" VARCHAR, "packages" VARCHAR[]);
CREATE TABLE "cofy_server.packages" (
    "title" VARCHAR, "version" VARCHAR, "path" VARCHAR
);

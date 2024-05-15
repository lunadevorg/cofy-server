/*
*     _____     ___    
*    / ___/__  / _/_ __
*   / /__/ _ \/ _/ // /
*   \___/\___/_/ \_, / 
*               /___/  
*
*   add_pkg.sql: add a package to the db
*   returns: none
*   parameters: 
*       $1 - user's ip
*       $2 - package title
*       $3 - package version
*       $4 - path
*/

INSERT INTO "cofy_server.packages" ("title", "version", "path") 
VALUES ($2, $3, $4);

IF EXISTS (SELECT 1 FROM "cofy_server.ownership" WHERE "ip" = $1) THEN
    UPDATE "cofy_server.ownership" 
    SET "packages" = array_append("packages", $2) 
    WHERE "ip" = $1;
ELSE
    INSERT INTO "cofy_server.ownership" ("ip", "packages") 
    VALUES ($1, ARRAY[$2]);
END IF;

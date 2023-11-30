# pglas
PostgreSQL extension to Read LAS 2.0 (well logging file).

Supports only version 2.0 of [LAS Specification](https://www.cwls.org/wp-content/uploads/2017/02/Las2_Update_Feb2017.pdf). For more information about this format, see the [Canadian Well Logging Society](http://www.cwls.org).

Functions:
las_na(_file_) - shows N/A value;
las_curves(_file_) - return enumerated curve names;
las_curve(_file_, _curve_) - return coupled values from the DEPT curve (expected to be first) and specific curve.

Example:
```
~/pglas$ cargo pgrx run
...
pglas=# create extension pglas;
CREATE EXTENSION

pglas=# select las_na('/home/.../lasrs/sample/A10.las');
 las_na  
---------
 -999.25
(1 row)

pglas=# select * from las_curves('/home/.../lasrs/sample/A10.las');
 IDX |     CURVE     
-----+---------------
   0 | DEPT
   1 | Perm
   2 | Gamma
   3 | Porosity
   4 | Fluvialfacies
   5 | NetGross
(6 rows)

pglas=# select * from las_curve('/home/.../lasrs/sample/A10.las','Fluvialfacies');
   DEPT   | VAL 
----------+-----
 1499.879 |    
 1500.129 |    
 1500.629 |    
 1501.129 |   0
 1501.629 |   0
 1502.129 |   0
 1502.629 |   0
 1503.129 |   0
 1503.629 |   0
 1504.129 |   0
 1504.629 |   0
 1505.129 |   0
 1505.629 |   0
 1506.129 |   0
 1506.629 |   0
 1507.129 |   0
 1507.629 |   0
 1508.129 |   0
 1508.629 |   0
 1509.129 |   0
 1509.629 |   0
 1510.129 |   0
 1510.629 |   0
 1511.129 |   0
 1511.629 |   0
 1512.129 |   0
 1512.629 |   0
 1513.129 |   0
 1513.629 |   0
 1514.129 |   0
pglas=# select las_na('/home/shestero/lasrs/sample/A10.las');
 las_na  
---------
 -999.25
(1 row)

pglas=# select * from las_curves('/home/shestero/lasrs/sample/A10.las');
 IDX |     CURVE     
-----+---------------
   0 | DEPT
   1 | Perm
   2 | Gamma
   3 | Porosity
   4 | Fluvialfacies
   5 | NetGross
(6 rows)

pglas=# select * from las_curve('/home/shestero/lasrs/sample/A10.las','Gamma');
   DEPT   |    VAL    
----------+-----------
 1499.879 |          
 1500.129 |          
 1500.629 |          
 1501.129 |          
 1501.629 | 78.869453
 1502.129 | 78.008301
 1502.629 | 75.581558
 1503.129 | 73.238037
 1503.629 | 71.504173
 1504.129 | 71.459229
 1504.629 |   73.4478
...
```

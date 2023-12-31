# pglas
PostgreSQL extension to read LAS 2.0 (well logging file).

_Note: LAS files are: https://en.wikipedia.org/wiki/Log_ASCII_standard .
Please, don't confuse with LIDAR files with the same extension!!_

Supports only version 2.0 of [LAS Specification](https://www.cwls.org/wp-content/uploads/2017/02/Las2_Update_Feb2017.pdf). For more information about this format, see the [Canadian Well Logging Society](http://www.cwls.org).

Functions:
* las_na(_file_) - shows N/A value;
* las_curves(_file_) - return enumerated curve names;
* las_curve(_file_, _curve_) - return coupled values from the DEPT curve (expected to be first) and specific curve.

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

pglas=# select * from las_curve('/home/.../lasrs/sample/A10.las','Gamma');
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

See also: https://en.wikipedia.org/wiki/Well_logging

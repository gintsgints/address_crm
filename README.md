# address search CRM API

## Filling in SQLite DB.

To load address.db from address.tsv file, you should start sqlite from data folder:

```
cd data
sqlite3 address.db
.separator ,
delete from address;
.import address.csv address
```

# jstest

## Steps to reproduce the issue

- cargo run

You will get the below output

```log
val=Atom('>' type=inline)
string val=>
result=>
done
```

Expected output

```log
val=Atom('{"rows":[{"appUid":"cloudio","comment":"👍","commentedBy":"admin","contextId":"$DD$","contextValue":"cloudio.home","createdBy":"admin","creationDate":"2022-08-18T06:17:06.871617Z","lastUpdateDate":"2022-08-18T06:17:06.871617Z","lastUpdatedBy":"admin","md":{},"orgUid":"cloudio","uid":"01GAQSJAHQ7WE54JNRC1J9HVH3","_rs":"Q"}]}' type=dynamic)
string val={"rows":[{"appUid":"cloudio","comment":"👍","commentedBy":"admin","contextId":"$DD$","contextValue":"cloudio.home","createdBy":"admin","creationDate":"2022-08-18T06:17:06.871617Z","lastUpdateDate":"2022-08-18T06:17:06.871617Z","lastUpdatedBy":"admin","md":{},"orgUid":"cloudio","uid":"01GAQSJAHQ7WE54JNRC1J9HVH3","_rs":"Q"}]}
result={"rows":[{"appUid":"cloudio","comment":"👍","commentedBy":"admin","contextId":"$DD$","contextValue":"cloudio.home","createdBy":"admin","creationDate":"2022-08-18T06:17:06.871617Z","lastUpdateDate":"2022-08-18T06:17:06.871617Z","lastUpdatedBy":"admin","md":{},"orgUid":"cloudio","uid":"01GAQSJAHQ7WE54JNRC1J9HVH3","_rs":"Q"}]}
done
```

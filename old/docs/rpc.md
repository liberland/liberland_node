# RPCs documentation

## Identity pallet

1. `get_passport_id`
 ```
 curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{ "jsonrpc":"2.0", "id":1, "method":"get_passport_id", "params": ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"] }'
 ```
4. `get_id_identities`
```
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{ "jsonrpc":"2.0", "id":1, "method":"get_id_identities", "params": [[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ]] }'
```

## Ministry of Interior pallet
1. `get_all_requests`
```
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{ "jsonrpc":"2.0", "id":1, "method":"get_all_requests" }'
```

## Referendum pallet
1. `get_active_petitions`
```
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{ "jsonrpc":"2.0", "id":1, "method":"get_active_petitions" }'

```
2. `get_active_referendums`
```
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{ "jsonrpc":"2.0", "id":1, "method":"get_active_referendums" }'

```
3. `get_successfull_referendums`
```
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{ "jsonrpc":"2.0", "id":1, "method":"get_successfull_referendums" }'

```


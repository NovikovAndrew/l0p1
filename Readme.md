
Создание ордера
```
curl -X POST http://localhost:9090/orders \
-H "Content-Type: application/json" \
-d '{
"order_uid": "123",
"track_number": "ABC123",
"entry": "WBIL",
"delivery": {
"name": "John Doe",
"phone": "+123456789",
"zip": "12345",
"city": "Sample City",
"address": "123 Sample St",
"region": "Sample Region",
"email": "john@example.com"
},
"payment": {
"transaction": "xyz789",
"request_id": "",
"currency": "USD",
"provider": "TestProvider",
"amount": 100,
"payment_dt": 123456789,
"bank": "TestBank",
"delivery_cost": 10,
"goods_total": 90,
"custom_fee": 0
},
"items": [
{
"chrt_id": 12345,
"track_number": "ABC123",
"price": 90,
"rid": "abc123",
"name": "Test Item",
"sale": 0,
"size": "M",
"total_price": 90,
"nm_id": 67890,
"brand": "TestBrand",
"status": 200
}
],
"locale": "en",
"internal_signature": "",
"customer_id": "customer123",
"delivery_service": "TestService",
"shardkey": "test",
"sm_id": 1,
"date_created": "2023-09-01T00:00:00Z",
"oof_shard": "test"
}'
```


GET /orders/:id для получения заказа по ID:
```curl http://localhost:9090/orders/123```

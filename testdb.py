from clickhouse_connect import create_client

client = create_client(host="localhost", port="8123", password="default", username="default", database="default")


print(client.query("select * from my_first_table").result_rows)

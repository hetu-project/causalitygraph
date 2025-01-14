import psycopg2
from psycopg2 import OperationalError

def create_connection(db_name, db_user, db_password, db_host, db_port):
    connection = None
    try:
        connection = psycopg2.connect(
            database=db_name,
            user=db_user,
            password=db_password,
            host=db_host,
            port=db_port,
        )
        print("Connection to PostgreSQL DB successful")
    except OperationalError as e:
        print(f"The error '{e}' occurred")
    return connection

def execute_read_query(connection, query):
    cursor = connection.cursor()
    result = None
    try:
        cursor.execute(query)
        result = cursor.fetchall()
        return result
    except OperationalError as e:
        print(f"The error '{e}' occurred")


host = '212.56.40.235'
db = 'udaya'
username = 'udaya'
password = 'udaya123'
port = 5432
db_table = 'event'

# 使用你的数据库信息替换以下变量
connection = create_connection(
    db, username, password, host, port
)

# 示例查询
select_users = f"SELECT * FROM {db_table}"
users = execute_read_query(connection, select_users)

for user in users:
    print(user)
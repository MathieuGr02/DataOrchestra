import time

from faker import Faker

fake = Faker()

while True:
    data = {"name": fake.name(), "email": fake.email()}
    print(data)
    time.sleep(0.5)

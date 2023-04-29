from collections import deque

class Person:
    def __init__(self, 
                 name: str, 
                 age: int, 
                 friends: list):
        self.name = name
        self.age = age
        self.friends = friends

def main():
    queue = deque()
    visited = set()

    # Create a graph
    John = Person('John', 20, [])
    Jane = Person('Jane', 35, [])
    Bob = Person('Bob', 45, [])
    Mary = Person('Mary', 13, [])
    Alice = Person('Alice', 50, [])

    John.friends.append(Jane)
    John.friends.append(Bob)
    Jane.friends.append(Mary)
    Jane.friends.append(Bob)
    Bob.friends.append(Alice)

    # Add the first node to the queue
    queue.append(John)
    visited.add(John)
    while len(queue) > 0:
        # Get the first node from the queue
        current = queue.popleft()
        print(f'Current: {current.name}')
        for friend in current.friends:
            if friend not in visited:
                queue.append(friend)
                visited.add(friend)
                if friend.age >= 50:
                    name = friend.name
                    print(f'Found old friend {name}!')
                    return

if __name__ == '__main__':
    main()
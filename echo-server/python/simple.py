import argparse
import socket
import sys
import time


def main(argv=sys.argv[1:]):
    parser = argparse.ArgumentParser()
    parser.add_argument('-H', '--host', default='127.0.0.1')
    parser.add_argument('-p', '--port', type=int, default=9999)
    args = parser.parse_args(argv)

    host = args.host
    port = args.port

    soc = socket.socket()
    soc.bind((host, port))
    soc.listen(5)

    try:
        while True:
            conn, addr = soc.accept()
            try:
                while True:
                    buf = conn.recv(1024)
                    print(buf.decode())
                    conn.send(buf)
            except Exception:
                conn.close()
    finally:
        soc.close()


if __name__ == '__main__':
    sys.exit(main())

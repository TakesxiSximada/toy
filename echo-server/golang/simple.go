package main

import (
	"flag"
	"fmt"
	"io"
	"log"
	"net"
	"os"
)

func main() {
	port := *flag.Int("p", 9999, "PortNumber")
	host := *flag.String("a", "127.0.0.1", "Address")

	fmt.Println(fmt.Sprintf("Listen: %s:%d", host, port))

	socket, e := net.Listen("tcp", fmt.Sprintf("%s:%d", host, port))
	if e != nil {
		log.Fatal(e)
		os.Exit(1)
	}

	for {
		conn, e := socket.Accept()
		if e != nil {
			log.Fatal(e)
		}
		handleClient(conn)
	}
	os.Exit(0)
}

func handleClient(conn net.Conn) {
	defer conn.Close()
	io.Copy(conn, conn)
}

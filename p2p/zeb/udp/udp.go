package main

import (
	"fmt"
	"net"
	"os"
)

func main() {
	ServerAddr, err := net.ResolveUDPAddr("udp", ":8050")
	if err != nil {
		fmt.Println("Error: ", err)
		os.Exit(0)
	}

	ServerConn, err := net.ListenUDP("udp", ServerAddr)
	if err != nil {
		fmt.Println("Error: ", err)
		os.Exit(0)
	}
	defer ServerConn.Close()

	buf := make([]byte, 1024)

	for {
		n, addr, err := ServerConn.ReadFromUDP(buf)
		if err != nil {
			fmt.Println("Error: ", err)
		}

		fmt.Println("Received ", string(buf[0:n]), " from ", addr)

		if _, err := ServerConn.WriteToUDP(buf[:n], addr); err != nil {
			fmt.Println("Error: ", err)
		}
	}
}

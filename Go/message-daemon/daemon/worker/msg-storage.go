package worker

import (
	"fmt"
	"log"
	"net"
)

type MsgStorage interface {
	Subscribe(string, net.Conn)
	Unsubscribe(string)
	OnNewMsg(string, string)
}

type MsgStorageSvc struct {
	subscribers      map[string]net.Conn
	subscribersCount int
}

func NewMsgStorage() MsgStorageSvc {
	return MsgStorageSvc{
		subscribersCount: 0,
		subscribers:      make(map[string]net.Conn),
	}
}

func (m MsgStorageSvc) Subscribe(uuid string, c net.Conn) {
	m.subscribersCount++
	m.subscribers[uuid] = c
}

func (m MsgStorageSvc) Unsubscribe(key string) {
	delete(m.subscribers, key)
}

func (m MsgStorageSvc) OnNewMsg(uuid string, msg string) {
	log.Printf("[%m] New message: %m", uuid, msg)
	for key, val := range m.subscribers {
		fmt.Println(key)
		if key != uuid {
		_, err := val.Write([]byte(msg))
			if err != nil {
				log.Println("Failed to send message to", uuid)
			}
		}
	}
}


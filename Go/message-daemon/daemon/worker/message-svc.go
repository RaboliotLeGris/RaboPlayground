package worker

import (
	"log"
	"net"
	"os"
	"os/signal"

	"../utils"
)

type Service interface {
	Start()
}

type MsgSvc struct {
	listener net.Listener
	msgSvc   MsgStorage
}

func NewMessageWorker() (MsgSvc, error) {
	l, err := net.Listen("unix", "/tmp/gilrain.sock")
	if err != nil {
		return MsgSvc{}, err
	}
	return MsgSvc{
		listener: l,
		msgSvc:   NewMsgStorage(),
	}, nil
}

func (w MsgSvc) Start() {
	log.Println("Starting message worker")
	go w.listenSignals()
	w.listen()
}

func (w MsgSvc) listen() {
	for {
		c, err := w.listener.Accept()
		if err != nil {
			log.Println("Failed to accept connection, got error", err)
		} else {
			go w.onNewClient(c)
		}
	}
}

func (w MsgSvc) onNewClient(c net.Conn) {
	uuid := utils.GenUuid(24)
	w.msgSvc.Subscribe(uuid, c)
	for {
		buf := make([]byte, 512)
		nr, err := c.Read(buf)
		if err != nil {
			w.msgSvc.Unsubscribe(uuid)
			return
		}
		data := buf[0:nr]
		go w.msgSvc.OnNewMsg(uuid, string(data))
	}
}

func (w MsgSvc) listenSignals() {
	sig := make(chan os.Signal, 1)
	signal.Notify(sig)
	s := <-sig
	log.Printf("RECEIVED SIGNAL: %s", s)

	err := w.listener.Close()
	if err != nil {
		log.Println("Failed to close listener. Error: ", err)
	}

	os.Exit(0)
}

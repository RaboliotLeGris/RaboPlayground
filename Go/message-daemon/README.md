# Message service (Gilrain)

Go systemd service with a client to send message to the other users. 
I use a perticlar because it's easier/shorter with systemctl

## Goal
Create a sort of IRC

## Usage
### How to test locally
* Open three terms (On on daemon side and two on client side)
* Then enter `make`

### How to create a systemd service
* Create a program to listen to unix signals
* Create a name.service file (based on another file service)
* Create service user : `sudo useradd name -s /sbin/nologin -M`
* Do: `cp name.service /lib/systemd/system/.`
* Then do: `sudo chmod 755 /lib/systemd/system/name.service`

### How to test the service
* Enable: `sudo systemctl enable name.service`
* Start: `sudo systemctl start sleepservice`
* Get the logs: `sudo journalctl -f -u sleepservice`

## Biblio
* https://fabianlee.org/2017/05/21/golang-running-a-go-binary-as-a-systemd-service-on-ubuntu-16-04/

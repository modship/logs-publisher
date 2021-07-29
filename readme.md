# Logs publisher

This program reads the log file and sends each entry to a mqtt broker.

## How to use

Run cli with args

### Command Args 
* -f : Path of the log file
* -u : Broker MQTT Url
* -t : MQTT topic target


```
logs-publisher -u tcp://127.0.0.1:1883 -f "/Users/me/MyLogFile.log" -t mytopic/logs
```



### TODO

- [x] Retrieve args
- [x] Connect to MQTT
- [x] Read log file
- [x] Send log to MQTT
- [ ] Log filtering

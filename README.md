# hms-mqtt-publisher

This tool fetches the current telemetry information from the HMS-XXXXW-2T series of micro-inverters and publishes the information into an MQTT broker. Please note that it doesn’t implement a DTU, but pulls the information off the internal DTU of these inverters. 

Firmware v01.x is supported through automatic Application Information and encryption detection. Encrypted DTUs obtain `enc_rand` automatically and use the A311/RealDataNew telemetry path; legacy firmware continues to use the existing A303/RealData path. Communication remains local and does not require the Hoymiles cloud.

### Encrypted firmware and slow telemetry

New firmware uses A311 / RealDataNew with AES-GCM. On the tested HMS-800W-2T, RealDataNew could remain stale for several minutes until the power limit was initialized once. If this occurs, configure the initialization for one process start:

```toml
initialize_power_limit = 100
```

After a successful initialization, remove or disable this option unless the limit is intentionally changed. The setting persisted across a restart on the tested device, but this behavior is not guaranteed for every model or firmware version.

### Performance Data Mode

Performance Data Mode can optionally be enabled during startup:

```toml
enable_performance_mode = true
```

The default is `false` to preserve compatibility with legacy devices.

The mode is sent once after Application Information has been read. It may improve telemetry refresh behavior, but it is not necessarily persistent across a DTU restart. If the device rejects the command, the publisher logs a warning and continues.

### Fast polling

Short intervals such as 5 seconds worked on the tested HMS-800W-2T after power-limit initialization. Depending on the firmware, fast local polling may still affect Hoymiles Cloud updates. The polling interval is not automatically changed or limited.

## How to run
The tool is distributed as source only — for now. You’ll have to download, compile and run it yourself. Please note that configuration of hosts, and passwords is done via `config.toml` from the current directory. It supports two different output channels. One is a simple MQTT publisher that doesn't follow a particular schema, and the other is made for [Home Assistant](https://www.home-assistant.io). It supports auto discovery of devices.

```
$ git clone https://github.com/DennisOSRM/hms-mqtt-publisher.git
$ cd hms-mqtt-publisher
$ cargo r
```
![image](https://github.com/lumapu/ahoy/assets/1067895/32c0b9b6-5aea-41e3-b9f8-161ce82fb99a)

### Docker

The latest release is directly deployable via a docker image from [DockerHub](https://hub.docker.com/r/dennisosrm/hms-mqtt-publisher). It is built automatically for the following Linux platforms: 
 - amd64,
 - arm/v7,
 - and arm64.

The parameters to access the inverter and MQTT instance are pulled from environment variables:
- `$INVERTER_HOST`
- `$MQTT_BROKER_HOST`
- `$MQTT_USERNAME` (optional)
- `$MQTT_PASSWORD` (optional)
- `$MQTT_PORT` (optional)

### Ansible (systemd)

You can use the [bellackn.homelab.hms-mqtt-publisher](https://github.com/bellackn/ansible-collection-homelab/blob/main/roles/hms_mqtt_publisher/README.md)
role to deploy hms-mqtt-publisher as a systemd service to a remote host. Check the role's documentation to see configuration options and setup instructions.

## Note of caution
Please note: The tool does not come with any guarantees and if by chance you fry your inverter with a funny series of bits, you are on your own. That being said, no inverters have been harmed during development. 

## Known limitations
- Some firmware versions may return the previous reading when polled again within approximately 30 seconds and may restart their internal refresh countdown. On the tested HMS-800W-2T, a one-time power-limit initialization enabled fresh RealDataNew values with shorter intervals, but this is not guaranteed for all models or firmware versions.
- The tool is a CLI tool and not a background service. 
- The tools was developed for (and with an) HMS-800W-2T. It may work with the other inverters from the series, but is untested at the time of writing

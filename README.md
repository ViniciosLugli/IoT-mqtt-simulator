<p align="center">
  <img src="https://raw.githubusercontent.com/PKief/vscode-material-icon-theme/ec559a9f6bfd399b82bb44393651661b08aaf7ba/icons/folder-markdown-open.svg" width="100" alt="project-logo">
</p>
<p align="center">
	<h1 align="center">IoT MQTT Simulator</h1>
</p>
<p align="center">
	<em> Project developed for the module 9 at INTELI.</em>
</p>
<p align="center">
	<img src="https://img.shields.io/github/license/ViniciosLugli/IoT-mqtt-simulator?style=default&logo=opensourceinitiative&logoColor=white&color=78DCE8" alt="license">
	<img src="https://img.shields.io/github/languages/top/ViniciosLugli/IoT-mqtt-simulator?style=default&color=78DCE8" alt="repo-top-language">

---

## Overview

A simple IoT simulator that uses MQTT protocol to send and receive messages from a broker. The simulator is capable of simulating the SPS30 sensor, which measures the concentration of particles in the air.

## Getting Started

### Prerequisites

-   [Docker](https://www.docker.com/)

### Setup environment

1. Clone the repository

```bash
git clone git@github.com:ViniciosLugli/IoT-mqtt-simulator.git
```

2. Navigate to the project's root directory

```bash
cd IoT-mqtt-simulator
```

3. Create a `.env` file in the root directory and add the following default environment variables to local connection, or change the values to connect to a remote.

```shell
BROKER = "mqtt://localhost:1891"
DATABASE_URL="postgresql://postgres:postgres@postgres:5432/postgres?schema=public"
```

### Run the project

Compile and run the project, you only need to run the docker environment, and all the services will be up and running.

```bash
docker compose up
```

### Run the tests

To run the tests, you need to have the broker of the environment variable running and [rust](https://www.rust-lang.org/) installed. Just go to the root directory and run the following command:

```bash
cargo test
```

The test sources are located in the directory of the files they are testing:

-   [MQTT](common/src/mqtt.rs#L102)
-   [Sensor](publisher/src/sensor.rs#L45)

## Demo

https://github.com/ViniciosLugli/IoT-mqtt-simulator/assets/40807526/0a3835fd-fde8-44c2-9c06-46a6ab0d18a2

## License

This project is protected under the [GNU General Public License v3.0](https://choosealicense.com/licenses/gpl-3.0/) License.

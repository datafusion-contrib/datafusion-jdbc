# DataFusion-JDBC

Run DataFusion as a JDBC server to query data from Delta Lake.

## Usage

### Setup Arrow Flight JDBC client

Download the Arrow Flight JDBC client of version 10.0.1 from [here](https://repo1.maven.org/maven2/org/apache/arrow/flight-sql-jdbc-driver/10.0.1/flight-sql-jdbc-driver-10.0.1.jar),
and explore the data using a broad range of [clients](https://docs.dremio.com/current/sonar/client-applications/clients/).

### Run the server

#### Build and run the server inside IntelliJ

Build and start the server using the run button ![run](run.png) next to `main.rs/main()`.

#### Build and run the server using Docker

First, build the Docker image:

```bash
docker build -f Dockerfile -t datafusion-jdbc .
```

Then, run the Docker image:

```bash
docker run -p 50051:50051 -it datafusion-jdbc
```

Optionally, you can specify the Delta lake root path:

```bash
docker run -e DELTA_DIR=hdfs://your-namenode:port/path/to/delta/store -p 50051:50051 -it datafusion-jdbc
```

FROM --platform=linux/arm64 rust:1.70 as builder

# Install JDK 11
RUN apt-get update && \
    apt-get install -y openjdk-11-jdk && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

RUN apt-get update && \
    apt-get -y install build-essential clang llvm cmake curl tar

# Set JAVA_HOME and related environment variables
ENV JAVA_HOME /usr/lib/jvm/java-11-openjdk-arm64
ENV LD_LIBRARY_PATH=$JAVA_HOME/lib/server:$LD_LIBRARY_PATH
ENV C_INCLUDE_PATH=$JAVA_HOME/include:$JAVA_HOME/include/linux:$C_INCLUDE_PATH

# Install Hadoop 3.3.2
RUN curl -O https://downloads.apache.org/hadoop/common/hadoop-3.3.2/hadoop-3.3.2.tar.gz && \
    tar -xzvf hadoop-3.3.2.tar.gz -C /opt && \
    rm hadoop-3.3.2.tar.gz

ENV HADOOP_HOME /opt/hadoop-3.3.2
ENV PATH $PATH:$HADOOP_HOME/bin:$HADOOP_HOME/sbin
ENV LD_LIBRARY_PATH $HADOOP_HOME/lib/native:$LD_LIBRARY_PATH

# Set CLASSPATH for Hadoop
RUN echo "export CLASSPATH=$CLASSPATH:`$HADOOP_HOME/bin/hadoop classpath --glob`" >> /etc/environment

# Set the working directory to /build/, build, move the binary and finally clean up
WORKDIR /build
COPY . .
RUN rustup component add rustfmt
RUN cargo build
RUN mv target/debug/datafusion-jdbc /usr/local/bin
RUN rm -rf /build

WORKDIR /jdbc

COPY run.sh /jdbc/run.sh
RUN chmod +x /jdbc/run.sh

ENTRYPOINT ["/jdbc/run.sh"]

EXPOSE 50051

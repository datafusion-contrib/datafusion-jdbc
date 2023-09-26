#!/bin/bash
#
# Licensed to the Apache Software Foundation (ASF) under one or more
# contributor license agreements.  See the NOTICE file distributed with
# this work for additional information regarding copyright ownership.
# The ASF licenses this file to You under the Apache License, Version 2.0
# (the "License"); you may not use this file except in compliance with
# the License.  You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#

rotate_logs ()
{
    log=$1
    num=5

    if [ -f "$log" ]; then # rotate logs
        while [ $num -gt 1 ]; do
            prev=`expr $num - 1`
            [ -f "$log.$prev" ] && mv "$log.$prev" "$log.$num"
            num=$prev
        done
        mv "$log" "$log.$num";
    fi
}

if [ -z "$DELTA_DIR" ]; then
    echo "DELTA_DIR environment variable is not set. Please provide a path using -e DELTA_DIR=/path/to/delta_dir when running the Docker container."
    exit 1
fi

mkdir -p ./log

stdout_log="./log/stdout.log"
stderr_log="./log/stderr.log"

# Rotate logs
rotate_logs "$stdout_log"
rotate_logs "$stderr_log"

datafusion-jdbc -d $DELTA_DIR >> "$stdout_log" 2>> "$stderr_log"

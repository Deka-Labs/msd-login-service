#!/bin/sh

if [ -z "${DATABASE_URL}" ]
then 
    echo "DATABASE_URL not set"
    exit -1
fi


if [ -f "${DATABASE_URL}" ]
then 
    echo "Skipping creation of DB: File exists"
else 
    diesel setup
fi


if [ -z "${DEV_SERVER}" ]
then 
    target/release/msd-login-service
else 
    cargo watch -x run
fi

exit 0

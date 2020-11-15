#!/bin/bash

export FIRESTORE_EMULATOR_HOST=http://localhost:8080
gcloud beta emulators firestore start --host-port localhost:8080
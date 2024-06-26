#!/bin/bash

response=$(curl -s -X PUT -d '{"Hello": "greetings"}' http://localhost:8080/mfa_fe_demo:mfa_fe_demo:template.os/api)

exit $response

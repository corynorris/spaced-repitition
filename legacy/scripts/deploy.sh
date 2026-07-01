#!/bin/bash

# Server details
DOKKU_HOST="192.168.2.34"
API_APP_NAME="srs-api"
WEB_APP_NAME="srs-web"

# Function to check if an app exists
app_exists() {
    ssh dokku@${DOKKU_HOST} apps:exists $1 2>/dev/null
    return $?
}

# Create apps if they don't exist
if ! app_exists ${API_APP_NAME}; then
    echo "Creating API app..."
    ssh dokku@${DOKKU_HOST} apps:create ${API_APP_NAME}
    # Set port mapping for new API app
    ssh dokku@${DOKKU_HOST} ports:set ${API_APP_NAME} http:80:5000
fi

if ! app_exists ${WEB_APP_NAME}; then
    echo "Creating web app..."
    ssh dokku@${DOKKU_HOST} apps:create ${WEB_APP_NAME}
fi

# Deploy the applications
git subtree push --prefix api dokku-api main
git subtree push --prefix web dokku-web main
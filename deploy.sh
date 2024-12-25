git remote add dokku-api dokku@192.168.2.26:srs-api
git remote add dokku-web dokku@192.168.2.26:srs

# Create a deployment script (deploy.sh):
#!/bin/bash
# Push the api subdirectory
git subtree push --prefix api dokku-api main

# Push the web subdirectory
git subtree push --prefix web dokku-web main
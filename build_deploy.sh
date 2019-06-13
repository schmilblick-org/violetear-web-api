#!/bin/sh -e

if ! docker info &>/dev/null; then
  if [ -z "$DOCKER_HOST" -a "$KUBERNETES_PORT" ]; then
    export DOCKER_HOST='tcp://localhost:2375'
  fi
fi

if [[ -n "$CI_REGISTRY_USER" ]]; then
  echo "Logging to GitLab Container Registry with CI credentials..."
  docker login -u "$CI_REGISTRY_USER" -p "$CI_REGISTRY_PASSWORD" "$CI_REGISTRY"
fi

sed 

docker build -f Dockerfile.deploy --tag "$CI_REGISTRY_IMAGE/$CI_COMMIT_REF_SLUG:deploy" .

docker push "$CI_REGISTRY_IMAGE/$CI_COMMIT_REF_SLUG:deploy"
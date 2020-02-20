# serverless [![Build Status](https://dev.azure.com/reitermarkus/serverless/_apis/build/status/reitermarkus.serverless?branchName=master)](https://dev.azure.com/reitermarkus/serverless/_build/latest?definitionId=2&branchName=master)

## Dependencies

- [Docker](https://docs.docker.com/install/)
- [OpenFaaS CLI](https://docs.openfaas.com/cli/install/)
- [Rust](https://rustup.rs)
- [Ruby](https://www.ruby-lang.org/)
  - [Rake](https://ruby.github.io/rake/)

## Kafka

### Deployment

To deploy for development, run

```
rake deploy
```

When finished, run

```
rake kill
```

To (re)deploy the `filter` function, run

```
rake deploy:functions[filter]
```

or to (re)deploy the `devices` and `filter` function, run

```
rake deploy:functions[devices, filter]
```

### Testing

#### Kafka Connector

To test if the Kafka Connector container is working, run

```
./produce.sh
```

and type a line, then check the logs using

```
docker service logs func_connector -f
```

to see if Kafka Connector received the line.

## Mobile App

### Requirements

- [Flutter](https://flutter.dev)
- [Xcode](https://developer.apple.com/xcode/) for iOS development
- [Android SDK](https://developer.android.com/studio) for Android development

### Running the Application

```
flutter run
```

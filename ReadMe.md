# serverless [![Build Status](https://dev.azure.com/reitermarkus/serverless/_apis/build/status/reitermarkus.serverless?branchName=master)](https://dev.azure.com/reitermarkus/serverless/_build/latest?definitionId=2&branchName=master)

## Dependencies

- [Docker](https://docs.docker.com/install/)
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
rake deploy:functions[devices,filter]
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

#### Functions

To test a function, i.e. `function-name`, first deploy the swarm, then run

```bash
./deploy.rs func function-name
```

## Mobile App

### Requirements

[Yarn](https://www.yarnpkg.com/en/docs/install) and the [React Native Toolkit](https://facebook.github.io/react-native/docs/getting-started) have to be installed.

### Developing for Android

For developing the Android app, open `mobile/SensorData` and run `react-native run-android`.

### Developing for iOS

For developing the iOS app, open `mobile/SensorData/ios/SensorData.xcodeproj` in Xcode.

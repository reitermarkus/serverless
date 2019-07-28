# serverless

## Dependencies

- Docker: https://docs.docker.com/install/
- Rust: https://rustup.rs
  - `cargo-script`: `cargo install cargo-script`

## Kafka

### Deployment

To deploy for development, run

```
./deploy.rs --no-auth
```

When finished, run

```
docker swarm leave -f
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
cd functions
faas build -f function-name.yml
faas deploy -f function-name.yml
sleep 5
printf TEST | faas invoke function-name
faas remove -f function-name.yml
```

## Mobile App

### Requirements

[Yarn](https://www.yarnpkg.com/en/docs/install) and the [React Native Toolkit](https://facebook.github.io/react-native/docs/getting-started) have to be installed.

### Developing for Android

For developing the Android app, open `mobile/SensorData` and run `react-native run-android`.

### Developing for iOS

For developing the iOS app, open `mobile/SensorData/ios/SensorData.xcodeproj` in Xcode.

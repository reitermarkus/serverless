/**
 * @format
 * @flow
 */

import React, {Component} from 'react'
import { Text as RnText } from 'react-native'
import { setUpdateIntervalForType, accelerometer, SensorTypes , gyroscope, magnetometer, barometer } from 'react-native-sensors'
import { Container, Header, Content, List, ListItem, Text } from 'native-base'
import DeviceInfo from 'react-native-device-info'

import { styles } from './styles/styles'

type Props = {};
export default class App extends Component<Props> {
  constructor(props) {
    super(props)

    this.state = {
      accelerometer: {},
      gyroscope: {},
      magnetometer: {},
      barometer: 0
    }

    const round = (x, n) => Math.round(x * Math.pow(10, n)) / Math.pow(10, n)

    setUpdateIntervalForType(SensorTypes.accelerometer, 100)
    setUpdateIntervalForType(SensorTypes.gyroscope, 100)
    setUpdateIntervalForType(SensorTypes.barometer, 100)
    setUpdateIntervalForType(SensorTypes.magnetometer, 100)

    accelerometer.subscribe(({ x, y, z, _ }) =>
      this.setState({
        accelerometer: {
          x: round(x, 5),
          y: round(y, 5),
          z: round(z, 5)
        }
      })
    )

    gyroscope.subscribe(({ x, y, z, _ }) =>
      this.setState({
        gyroscope: {
          x: round(x, 5),
          y: round(y, 5),
          z: round(z, 5)
        }
      })
    )

    magnetometer.subscribe(({ x, y, z, _ }) =>
      this.setState({
        magnetometer: {
          x: round(x, 5),
          y: round(y, 5),
          z: round(z, 5)
        }
      })
    )

    barometer.subscribe(({ pressure }) =>
      this.setState({
        barometer: round(pressure, 5)
      })
    )
  }

  render() {
    return (
      <Container>
        <Header noShadow androidStatusBarColor={styles.header.backgroundColor} style={styles.header}>
          <RnText style={styles.headerText}>Sensor Data</RnText>
        </Header>
        <Content>
          <List>
            <ListItem itemDivider>
              <Text>Device Info</Text>
            </ListItem>
            <ListItem style={styles.listItem}>
              <Text>Manufacturer</Text>
              <Text>{DeviceInfo.getBrand()}</Text>
            </ListItem>
            <ListItem style={styles.listItem}>
              <Text>Model</Text>
              <Text>{DeviceInfo.getModel()}</Text>
            </ListItem>
            <ListItem style={styles.listItem}>
              <Text>OS</Text>
              <Text>{DeviceInfo.getSystemName()}</Text>
            </ListItem>
            <ListItem style={styles.listItem}>
              <Text>OS version</Text>
              <Text>{DeviceInfo.getSystemVersion()}</Text>
            </ListItem>
            <ListItem itemDivider>
              <Text>Gyroscope</Text>
            </ListItem>
            <ListItem style={styles.listItem}>
              <Text>x</Text>
              <Text>{this.state.gyroscope.x}</Text>
            </ListItem>
            <ListItem style={styles.listItem}>
              <Text>y</Text>
              <Text>{this.state.gyroscope.y}</Text>
            </ListItem>
            <ListItem style={styles.listItem}>
              <Text>z</Text>
              <Text>{this.state.gyroscope.z}</Text>
            </ListItem>
            <ListItem itemDivider>
              <Text>Accelerometer</Text>
            </ListItem>
            <ListItem style={styles.listItem}>
              <Text>x</Text>
              <Text>{this.state.accelerometer.x}</Text>
            </ListItem>
            <ListItem style={styles.listItem}>
              <Text>y</Text>
              <Text>{this.state.accelerometer.y}</Text>
            </ListItem>
            <ListItem style={styles.listItem}>
              <Text>z</Text>
              <Text>{this.state.accelerometer.z}</Text>
            </ListItem>
            <ListItem itemDivider>
              <Text>Magnetometer</Text>
            </ListItem>
            <ListItem style={styles.listItem}>
              <Text>x</Text>
              <Text>{this.state.magnetometer.x}</Text>
            </ListItem>
            <ListItem style={styles.listItem}>
              <Text>y</Text>
              <Text>{this.state.magnetometer.y}</Text>
            </ListItem>
            <ListItem style={styles.listItem}>
              <Text>z</Text>
              <Text>{this.state.magnetometer.z}</Text>
            </ListItem>
            <ListItem itemDivider>
              <Text>Air pressure</Text>
            </ListItem>
            <ListItem style={styles.listItem}>
              <Text>pressure</Text>
              <Text>{this.state.barometer}</Text>
            </ListItem>
          </List>
        </Content>
      </Container>
    )
  }
}

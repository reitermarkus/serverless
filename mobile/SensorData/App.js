/**
 * @format
 * @flow
 */

import React, {Component} from 'react'
import { Text as RnText, Platform } from 'react-native'
import { setUpdateIntervalForType, accelerometer, SensorTypes , gyroscope, magnetometer, barometer } from 'react-native-sensors'
import { Container, Header, Content, List, ListItem, Text } from 'native-base'
import DeviceInfo from 'react-native-device-info'

import { styles } from './styles/styles'

import { CpuInfo, SensorService } from './native'

export default class App extends Component {
  constructor(props) {
    super(props)

    this.state = {
      accelerometer: {},
      gyroscope: {},
      magnetometer: {},
      barometer: {},
      cores: 0,
      coresInfo: {}
    }

    const round = (x, n) => Math.round(x * Math.pow(10, n)) / Math.pow(10, n)

    setUpdateIntervalForType(SensorTypes.accelerometer, 100)
    setUpdateIntervalForType(SensorTypes.gyroscope, 100)
    setUpdateIntervalForType(SensorTypes.barometer, 100)
    setUpdateIntervalForType(SensorTypes.magnetometer, 100)

    accelerometer.subscribe(({ x, y, z, timestamp }) =>
      this.setState({
        accelerometer: {
          x: round(x, 5),
          y: round(y, 5),
          z: round(z, 5),
          timestamp: timestamp
        }
      })
    )

    gyroscope.subscribe(({ x, y, z, timestamp }) =>
      this.setState({
        gyroscope: {
          x: round(x, 5),
          y: round(y, 5),
          z: round(z, 5),
          timestamp: timestamp
        }
      })
    )

    magnetometer.subscribe(({ x, y, z, timestamp }) =>
      this.setState({
        magnetometer: {
          x: round(x, 5),
          y: round(y, 5),
          z: round(z, 5),
          timestamp: timestamp
        }
      })
    )

    barometer.subscribe(({ pressure }) =>
      this.setState({
        barometer: {
          pressure: round(pressure, 5) || 0,
          timestamp: Date.now()
        }
      })
    )

    if (Platform.OS === 'android') {
      CpuInfo.getCpuCores(cores =>
        this.setState({
          cores: cores
        })
      )

      CpuInfo.getCoresInfo(info =>
        this.setState({
          coresInfo: Object.entries(info).sort((a, b) => a[0] === b[0] ? 0 : a[0] > b[0] ? 1 : -1)
        })
      )

      SensorService.startService()
        .then(success => console.log(`service: ${success}`))
        .catch(fail => `service: ${fail}`)
    }
  }

  async componentDidMount() {
    if (Platform.OS !== 'android') {
      this.timer = setInterval(async () => {
        try {
          const response = await fetch('http://10.0.0.5:4000/sensor', {
            method: 'POST',
            headers: {
              Accept: 'application/json',
              'Content-Type': 'application/json',
            },
            body: JSON.stringify({
              accelerometer: this.state.accelerometer,
              gyroscope: this.state.gyroscope,
              magnetometer: this.state.magnetometer,
              barometer: this.state.barometer
            }),
          })

          const responseJson = await response.json()
          console.log(responseJson)
        } catch (err) {
          console.error(err)
        }
      }, 5000)
    }
  }

  componentWillUnmount() {
    clearInterval(this.timer)
  }

  render() {
    return (
      <Container>
        <Header noShadow androidStatusBarColor={styles.header.backgroundColor} iosBarStyle='light-content' style={styles.header}>
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
            {Platform.OS === 'android' ? (
              <React.Fragment>
                <ListItem itemDivider>
                  <Text>CPU</Text>
                </ListItem>
                <ListItem style={styles.listItem}>
                  <Text>cores</Text>
                  <Text>{this.state.cores}</Text>
                </ListItem>
                {Object.entries(this.state.coresInfo).map(([key, value]) =>
                  <ListItem key={key} style={styles.listItem}>
                    <Text>{value[0]}</Text>
                    <Text style={{textAlign: 'right'}}>{value[1]}</Text>
                  </ListItem>
                )}
              </React.Fragment>
            ): null}
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
              <Text>{this.state.barometer.pressure}</Text>
            </ListItem>
          </List>
        </Content>
      </Container>
    )
  }
}

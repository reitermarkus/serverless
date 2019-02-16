/**
 * @format
 * @flow
 */

import React, {Component} from 'react'
import { ScrollView } from 'react-native'
import { setUpdateIntervalForType, accelerometer, SensorTypes , gyroscope, magnetometer, barometer } from 'react-native-sensors'
import { List, ListItem, Text, Left, Right } from 'native-base'
import DeviceInfo from 'react-native-device-info'

export default class IosView extends Component {
  constructor(props) {
    super(props)

    this.state = {
      accelerometer: {
        x: 0.0,
        y: 0.0,
        z: 0.0,
      },
      barometer: {
        pressure: 0.0,
      },
      gyroscope: {
        x: 0.0,
        y: 0.0,
        z: 0.0,
      },
      magnetometer: {
        x: 0.0,
        y: 0.0,
        z: 0.0,
      },
    }

    setUpdateIntervalForType(SensorTypes.accelerometer, 1000)
    setUpdateIntervalForType(SensorTypes.gyroscope,     1000)
    setUpdateIntervalForType(SensorTypes.barometer,     1000)
    setUpdateIntervalForType(SensorTypes.magnetometer,  1000)
  }

  async componentDidMount() {
    this.timer = setInterval(async () => {
      try {
        const response = await fetch('http://httpbin.org/anything', {
          method: 'POST',
          headers: {
            'Accept': 'application/json',
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

  componentWillMount() {
    this.accelerometerSubscription = accelerometer.subscribe(({ x, y, z, timestamp }) =>
      this.setState({
        accelerometer: {
          x: x,
          y: y,
          z: z,
          timestamp: timestamp,
        }
      })
    )

    this.barometerSubscription = barometer.subscribe(({ pressure }) =>
      this.setState({
        barometer: {
          pressure: pressure,
          timestamp: Date.now(),
        }
      })
    )

    this.gyroscopeSubscription = gyroscope.subscribe(({ x, y, z, timestamp }) =>
      this.setState({
        gyroscope: {
          x: x,
          y: y,
          z: z,
          timestamp: timestamp,
        }
      })
    )

    this.magnetometerSubscription = magnetometer.subscribe(({ x, y, z, timestamp }) =>
      this.setState({
        magnetometer: {
          x: x,
          y: y,
          z: z,
          timestamp: timestamp,
        }
      })
    )
  }

  componentWillUnmount() {
    clearInterval(this.timer)

    this.magnetometerSubscription.unsubscribe()
    this.accelerometerSubscription.unsubscribe()
    this.gyroscopeSubscription.unsubscribe()
    this.barometerSubscription.unsubscribe()
  }

  render() {
    return (
      <ScrollView>
        <List>
          <ListItem itemDivider>
            <Text>Device Info</Text>
          </ListItem>
          <ListItem>
            <Left>
              <Text>Manufacturer</Text>
            </Left>
            <Right>
              <Text>{DeviceInfo.getBrand()}</Text>
            </Right>
          </ListItem>
          <ListItem>
            <Left>
              <Text>Model</Text>
            </Left>
            <Right>
              <Text>{DeviceInfo.getModel()}</Text>
            </Right>
          </ListItem>
          <ListItem>
            <Left>
              <Text>OS</Text>
            </Left>
            <Right>
              <Text>{DeviceInfo.getSystemName()} {DeviceInfo.getSystemVersion()}</Text>
            </Right>
          </ListItem>
          <ListItem itemDivider>
            <Text>Gyroscope</Text>
          </ListItem>
          <ListItem>
            <Left>
              <Text>X</Text>
            </Left>
            <Right>
              <Text>{this.state.gyroscope.x.toFixed(5)}</Text>
            </Right>
          </ListItem>
          <ListItem>
            <Left>
              <Text>Y</Text>
            </Left>
            <Right>
              <Text>{this.state.gyroscope.y.toFixed(5)}</Text>
            </Right>
          </ListItem>
          <ListItem>
            <Left>
              <Text>Z</Text>
            </Left>
            <Right>
              <Text>{this.state.gyroscope.z.toFixed(5)}</Text>
            </Right>
          </ListItem>
          <ListItem itemDivider>
            <Text>Accelerometer</Text>
          </ListItem>
          <ListItem>
            <Left>
              <Text>X</Text>
            </Left>
            <Right>
              <Text>{this.state.accelerometer.x.toFixed(5)}</Text>
            </Right>
          </ListItem>
          <ListItem>
            <Left>
              <Text>Y</Text>
            </Left>
            <Right>
              <Text>{this.state.accelerometer.y.toFixed(5)}</Text>
            </Right>
          </ListItem>
          <ListItem>
            <Left>
              <Text>Z</Text>
            </Left>
            <Right>
              <Text>{this.state.accelerometer.z.toFixed(5)}</Text>
            </Right>
          </ListItem>
          <ListItem itemDivider>
            <Text>Magnetometer</Text>
          </ListItem>
          <ListItem>
            <Left>
              <Text>X</Text>
            </Left>
            <Right>
              <Text>{this.state.magnetometer.x.toFixed(5)}</Text>
            </Right>
          </ListItem>
          <ListItem>
            <Left>
              <Text>Y</Text>
            </Left>
            <Right>
              <Text>{this.state.magnetometer.y.toFixed(5)}</Text>
            </Right>
          </ListItem>
          <ListItem>
            <Left>
              <Text>Z</Text>
            </Left>
            <Right>
              <Text>{this.state.magnetometer.z.toFixed(5)}</Text>
            </Right>
          </ListItem>
          <ListItem itemDivider>
            <Text>Air pressure</Text>
          </ListItem>
          <ListItem>
            <Left>
              <Text>pressure</Text>
            </Left>
            <Right>
              <Text>{this.state.barometer.pressure.toFixed(5)}</Text>
            </Right>
          </ListItem>
        </List>
      </ScrollView>
    )
  }
}

/**
 * @format
 * @flow
 */

import React, {Component} from 'react'
import { ScrollView, DeviceEventEmitter, AsyncStorage } from 'react-native'
import { List, ListItem, Text, Left, Right, Spinner } from 'native-base'
import DeviceInfo from 'react-native-device-info'

import { SensorService } from './native'

export default class AndroidView extends Component {
  constructor(props) {
    super(props)

    this.state = {
      accelerometer: {},
      gyroscope: {},
      magnetometer: {},
      barometer: {},
      cores: 0,
      coresInfo: {},
      isLoading: true
    }

    const round = (x, n) => Math.round(x * Math.pow(10, n)) / Math.pow(10, n)

    const formatOutput = (val, unit) => {
      const rounded = round(+val.substring(0, val.indexOf(unit)), 4)
      return `${rounded} ${unit}`
    }

    this.deviceSubscription = DeviceEventEmitter.addListener('sensors', data => {
      this.setState({isLoading: false})

      const parsedSensors = JSON.parse(data)
      const values = parsedSensors.records[0].value

      if (values.cpu) {
        const cpu = values.cpu

        if (cpu.cores) {
          this.setState({
            cores: cpu.cores
          })
        }

        if (cpu.frequency) {
          this.setState({
            coresInfo: Object.entries(cpu.frequency).sort((a, b) => a[0] === b[0] ? 0 : a[0] > b[0] ? 1 : -1)
          })
        }
      }

      if (values.sensors) {
        const sensors = values.sensors

        if (sensors.gyroscope) {
          const gyro = sensors.gyroscope

          this.setState({
            gyroscope: {
              x: formatOutput(gyro.x, 'rad/s'),
              y: formatOutput(gyro.y, 'rad/s'),
              z: formatOutput(gyro.z, 'rad/s')
            }
          })
        }

        if (sensors.acceleration) {
          const acc = sensors.acceleration

          this.setState({
            accelerometer: {
              x: formatOutput(acc.x, 'm/s²'),
              y: formatOutput(acc.y, 'm/s²'),
              z: formatOutput(acc.z, 'm/s²')
            }
          })
        }

        if (sensors.magnetic) {
          const mag = sensors.magnetic

          this.setState({
            magnetometer: {
              x: formatOutput(mag.x, 'μT'),
              y: formatOutput(mag.y, 'μT'),
              z: formatOutput(mag.z, 'μT')
            }
          })
        }

        if (sensors.air_pressure) {
          this.setState({
            barometer: {
              pressure: formatOutput(sensors.air_pressure, 'hPa'),
            }
          })
        }
      }
    })

    AsyncStorage.getItem('ip').then(ip => {
      SensorService.startService(ip)
        .then(success => console.log(`service: ${success}`))
        .catch(fail => `service: ${fail}`)
    }).catch(e => console.log(e))
  }

  componentWillUnmount() {
    this.deviceSubscription.remove()
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
            <Text>CPU</Text>
          </ListItem>
            {this.state.isLoading ?
              <Spinner /> : (
                <>
                  <ListItem>
                    <Left>
                      <Text>cores</Text>
                    </Left>
                    <Right>
                      <Text>{this.state.cores}</Text>
                    </Right>
                  </ListItem>
                  {Object.entries(this.state.coresInfo).map(([key, value]) =>
                    <ListItem key={key}>
                      <Left>
                        <Text>{value[0]}</Text>
                      </Left>
                      <Right>
                        <Text style={{textAlign: 'right'}}>{value[1]}</Text>
                      </Right>
                    </ListItem>
                  )}
                </>
              )
            }
          <ListItem itemDivider>
            <Text>Gyroscope</Text>
          </ListItem>
          {this.state.isLoading ?
            <Spinner /> : (
              <>
                <ListItem>
                  <Left>
                  <Text>X</Text>
                  </Left>
                  <Right>
                    <Text>{this.state.gyroscope.x}</Text>
                  </Right>
                </ListItem>
                <ListItem>
                  <Left>
                    <Text>Y</Text>
                  </Left>
                  <Right>
                    <Text>{this.state.gyroscope.y}</Text>
                  </Right>
                </ListItem>
                <ListItem>
                  <Left>
                    <Text>Z</Text>
                  </Left>
                  <Right>
                    <Text>{this.state.gyroscope.z}</Text>
                  </Right>
                </ListItem>
              </>
            )}
          <ListItem itemDivider>
            <Text>Accelerometer</Text>
          </ListItem>
          {this.state.isLoading ?
            <Spinner /> : (
              <>
                <ListItem>
                  <Left>
                    <Text>X</Text>
                  </Left>
                  <Right>
                    <Text>{this.state.accelerometer.x}</Text>
                  </Right>
                </ListItem>
                <ListItem>
                  <Left>
                    <Text>Y</Text>
                  </Left>
                  <Right>
                    <Text>{this.state.accelerometer.y}</Text>
                  </Right>
                </ListItem>
                <ListItem>
                  <Left>
                    <Text>Z</Text>
                  </Left>
                  <Right>
                    <Text>{this.state.accelerometer.z}</Text>
                  </Right>
                </ListItem>
              </>
            )}
          <ListItem itemDivider>
            <Text>Magnetometer</Text>
          </ListItem>
          {this.state.isLoading ?
            <Spinner /> : (
              <>
                <ListItem>
                  <Left>
                    <Text>X</Text>
                  </Left>
                  <Right>
                    <Text>{this.state.magnetometer.x}</Text>
                  </Right>
                </ListItem>
                <ListItem>
                  <Left>
                    <Text>Y</Text>
                  </Left>
                  <Right>
                    <Text>{this.state.magnetometer.y}</Text>
                  </Right>
                </ListItem>
                <ListItem>
                  <Left>
                    <Text>Z</Text>
                  </Left>
                  <Right>
                    <Text>{this.state.magnetometer.z}</Text>
                  </Right>
                </ListItem>
              </>
            )}
          <ListItem itemDivider>
            <Text>Air pressure</Text>
          </ListItem>
          {this.state.isLoading ?
            <Spinner /> : (
              <>
                <ListItem>
                  <Left>
                    <Text>pressure</Text>
                  </Left>
                  <Right>
                    <Text>{this.state.barometer.pressure}</Text>
                  </Right>
                </ListItem>
              </>
            )}
        </List>
      </ScrollView>
    )
  }
}

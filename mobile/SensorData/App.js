/**
 * @format
 * @flow
 */

import React, {Component} from 'react'
import {Text, View } from 'react-native'
import { setUpdateIntervalForType, accelerometer, SensorTypes , gyroscope, barometer } from 'react-native-sensors'

import { styles } from './styles/styles'
import { grid } from './styles/grid'

type Props = {};
export default class App extends Component<Props> {
  constructor(props) {
    super(props)

    this.state = {
      accelerometer: {},
      gyroscope: {},
      barometer: 0,
    }

    const round = (x, n) => Math.round(x * Math.pow(10, n)) / Math.pow(10, n)

    setUpdateIntervalForType(SensorTypes.accelerometer, 100)
    setUpdateIntervalForType(SensorTypes.gyroscope, 100)
    setUpdateIntervalForType(SensorTypes.barometer, 100)

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

    barometer.subscribe(({ pressure }) =>
      this.setState({
        barometer: round(pressure, 5)
      })
    )
  }

  render() {
    return (
      <React.Fragment>
        <Text style={styles.headLine}>Sensor Data</Text>
        <View style={styles.container}>
          <Text style={styles.subHeadline}>Gyroscope</Text>
          <View style={grid.container}>
            <View style={grid.list}>
              <View style={grid.item}>
                <Text style={{textAlign: 'left'}}>x</Text>
              </View>
              <View style={grid.item}>
                <Text style={{textAlign: 'right'}}>{this.state.gyroscope.x}</Text>
              </View>
              <View style={grid.item}>
                <Text style={{textAlign: 'left'}}>y</Text>
              </View>
              <View style={grid.item}>
                <Text style={{textAlign: 'right'}}>{this.state.gyroscope.y}</Text>
              </View>
              <View style={grid.item}>
                <Text style={{textAlign: 'left'}}>z</Text>
              </View>
              <View style={grid.item}>
                <Text style={{textAlign: 'right'}}>{this.state.gyroscope.z}</Text>
              </View>
            </View>
          </View>
          <Text style={styles.subHeadline}>Accelerometer</Text>
          <View style={grid.container}>
            <View style={grid.list}>
              <View style={grid.item}>
                <Text style={{textAlign: 'left'}}>x</Text>
              </View>
              <View style={grid.item}>
                <Text style={{textAlign: 'right'}}>{this.state.accelerometer.x}</Text>
              </View>
              <View style={grid.item}>
                <Text style={{textAlign: 'left'}}>y</Text>
              </View>
              <View style={grid.item}>
                <Text style={{textAlign: 'right'}}>{this.state.accelerometer.y}</Text>
              </View>
              <View style={grid.item}>
                <Text style={{textAlign: 'left'}}>z</Text>
              </View>
              <View style={grid.item}>
                <Text style={{textAlign: 'right'}}>{this.state.accelerometer.z}</Text>
              </View>
            </View>
          </View>
          <Text style={styles.subHeadline}>Air pressure</Text>
          <View style={grid.container}>
            <View style={grid.list}>
              <View style={grid.item}>
                <Text style={{textAlign: 'left'}}>pressure</Text>
              </View>
              <View style={grid.item}>
                <Text style={{textAlign: 'right'}}>{this.state.barometer}</Text>
              </View>
            </View>
          </View>
        </View>
      </React.Fragment>
    )
  }
}

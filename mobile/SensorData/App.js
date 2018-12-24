/**
 * Sample React Native App
 * https://github.com/facebook/react-native
 *
 * @format
 * @flow
 */

import React, {Component} from 'react';
import {StyleSheet, Text, View, Dimensions} from 'react-native';
import { setUpdateIntervalForType, accelerometer, SensorTypes , gyroscope, barometer } from "react-native-sensors";

const { width, _ } = Dimensions.get('window');

const round = (x, n) => Math.round(x * Math.pow(10, n)) / Math.pow(10, n)

type Props = {};
export default class App extends Component<Props> {
  constructor(props) {
    super(props);

    this.state = {
      accelerometer: {},
      gyroscope: {},
      barometer: 0,
    }
  }

  componentDidMount() {
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
          <View style={gridStyle.container}>
            <View style={gridStyle.list}>
              <View style={gridStyle.item}>
                <Text style={{textAlign: 'left'}}>x</Text>
              </View>
              <View style={gridStyle.item}>
                <Text style={{textAlign: 'right'}}>{this.state.gyroscope.x}</Text>
              </View>
              <View style={gridStyle.item}>
                <Text style={{textAlign: 'left'}}>y</Text>
              </View>
              <View style={gridStyle.item}>
                <Text style={{textAlign: 'right'}}>{this.state.gyroscope.y}</Text>
              </View>
              <View style={gridStyle.item}>
                <Text style={{textAlign: 'left'}}>z</Text>
              </View>
              <View style={gridStyle.item}>
                <Text style={{textAlign: 'right'}}>{this.state.gyroscope.z}</Text>
              </View>
            </View>
          </View>
          <Text style={styles.subHeadline}>Accelerometer</Text>
          <View style={gridStyle.container}>
            <View style={gridStyle.list}>
              <View style={gridStyle.item}>
                <Text style={{textAlign: 'left'}}>x</Text>
              </View>
              <View style={gridStyle.item}>
                <Text style={{textAlign: 'right'}}>{this.state.accelerometer.x}</Text>
              </View>
              <View style={gridStyle.item}>
                <Text style={{textAlign: 'left'}}>y</Text>
              </View>
              <View style={gridStyle.item}>
                <Text style={{textAlign: 'right'}}>{this.state.accelerometer.y}</Text>
              </View>
              <View style={gridStyle.item}>
                <Text style={{textAlign: 'left'}}>z</Text>
              </View>
              <View style={gridStyle.item}>
                <Text style={{textAlign: 'right'}}>{this.state.accelerometer.z}</Text>
              </View>
            </View>
          </View>
          <Text style={styles.subHeadline}>Air pressure</Text>
          <View style={gridStyle.container}>
            <View style={gridStyle.list}>
              <View style={gridStyle.item}>
                <Text style={{textAlign: 'left'}}>pressure</Text>
              </View>
              <View style={gridStyle.item}>
                <Text style={{textAlign: 'right'}}>{this.state.barometer}</Text>
              </View>
            </View>
          </View>
        </View>
      </React.Fragment>
    );
  }
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    flexDirection: 'column',
  },
  headLine: {
    fontSize: 32,
    textAlign: 'center',
    padding: 20
  },
  subHeadline: {
    fontSize: 20,
    paddingBottom: 15,
    paddingLeft: 15
  }
});

const gridStyle = StyleSheet.create({
  container: {
    marginBottom: 15
  },
  item: {
    marginLeft: 'auto',
    marginRight: 'auto',
    width: width * 0.7 / 2,
    justifyContent: 'center',
    paddingBottom: 5
  },
  list: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    flexWrap: 'wrap'
  },
})

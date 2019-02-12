/**
 * @format
 * @flow
 */

import React, {Component} from 'react'
import { Text as RnText, Platform, ScrollView, TextInput } from 'react-native'
import { setUpdateIntervalForType, accelerometer, SensorTypes , gyroscope, magnetometer, barometer } from 'react-native-sensors'
import { StyleProvider, Container, Header, Content, Form, Item, Input, List, ListItem, Text, Title, Left, Right, Body, Icon, View, Footer, FooterTab, Button, getTheme } from 'native-base'
import DeviceInfo from 'react-native-device-info'

import theme from './styles/variables';

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
      coresInfo: {},
      footerTab: 0,
      ip: null,
    }

    const round = (x, n) => Math.round(x * Math.pow(10, n)) / Math.pow(10, n)

    setUpdateIntervalForType(SensorTypes.accelerometer, 1000)
    setUpdateIntervalForType(SensorTypes.gyroscope, 1000)
    setUpdateIntervalForType(SensorTypes.barometer, 1000)
    setUpdateIntervalForType(SensorTypes.magnetometer, 1000)

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
  }

  componentWillUnmount() {
    clearInterval(this.timer)
  }

  render() {
    const sensorPage = () => (
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
              <Text>{DeviceInfo.getSystemName()}</Text>
            </Right>
          </ListItem>
          <ListItem>
            <Left>
              <Text>OS version</Text>
            </Left>
            <Right>
              <Text>{DeviceInfo.getSystemVersion()}</Text>
            </Right>
          </ListItem>
          {Platform.OS === 'android' ? (
            <>
              <ListItem itemDivider>
                <Text>CPU</Text>
              </ListItem>
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
          ): null}
          <ListItem itemDivider>
            <Text>Gyroscope</Text>
          </ListItem>
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
          <ListItem itemDivider>
            <Text>Accelerometer</Text>
          </ListItem>
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
          <ListItem itemDivider>
            <Text>Magnetometer</Text>
          </ListItem>
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
          <ListItem itemDivider>
            <Text>Air pressure</Text>
          </ListItem>
          <ListItem>
            <Left>
              <Text>pressure</Text>
            </Left>
            <Right>
              <Text>{this.state.barometer.pressure}</Text>
            </Right>
          </ListItem>
        </List>
      </ScrollView>
    )

    const settingsPage = () => (
      <View>
        <Form>
          <Item>
            <Icon type='FontAwesome' name='server'/>
            <Input
              placeholder="Kafka REST URL"
              value={this.state.ip}
              onChangeText={(text) => this.setState({ip: text})}
              keyboardType={Platform.OS == 'ios' ? 'url' : 'default'}
            />
          </Item>
        </Form>
      </View>
    )

    const renderTab = (number) => {
      switch (number) {
        case 0:
          return sensorPage()
        case 1:
          return settingsPage()
        default:
          return sensorPage()
      }
    }

    const changeTab = (number) => {
      if (this.state.footerTab !== number) {
        this.setState({ footerTab: number });
      }
    }

    return (
      <StyleProvider style={getTheme(theme)}>
        <Container>
          <Header>
            <Body>
              <Title>Sensor Data</Title>
            </Body>
          </Header>
          <Content>
            {renderTab(this.state.footerTab)}
          </Content>
          <Footer>
            <FooterTab>
              <Button vertical active={this.state.footerTab === 0} onPress={() => { changeTab(0) }}>
                <Icon type="FontAwesome" name="microchip" />
                <Text>Sensors</Text>
              </Button>
              <Button vertical active={this.state.footerTab === 1} onPress={() => { changeTab(1) }}>
                <Icon name="settings" />
                <Text>Settings</Text>
              </Button>
            </FooterTab>
          </Footer>
        </Container>
      </StyleProvider>
    )
  }
}

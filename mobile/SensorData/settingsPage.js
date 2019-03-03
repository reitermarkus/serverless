import React, {Component} from 'react'
import { Platform, AsyncStorage, Keyboard, ToastAndroid, NativeModules } from 'react-native'
import { Form, Item, Input, Icon, View, Button, Text, } from 'native-base'
import { SensorService } from './native'

const ToastIOS = NativeModules.ToastIOS

export default class SettingsPage extends Component {
  constructor(props) {
    super(props)

    this.state = {
      ip: null,
      interval: 15000
    }

    this.storeData = async (key, value) => {
      try {
        await AsyncStorage.setItem(key, JSON.stringify(value))

        if (Platform.OS === 'android' && key === 'ip') {
          SensorService.setKafkaUrl(value)
        }

        if (Platform.OS === 'android' && key === 'interval') {
          SensorService.setKafkaUpdateInterval(value)
        }
      } catch (err) {
        console.log(err)
      }
    }
  }

  async componentDidMount() {
    try {
      const ip = JSON.parse(await AsyncStorage.getItem('ip'))
      if (ip) {
        this.setState({ip: ip})
      }

      const interval = JSON.parse(await AsyncStorage.getItem('interval'))

      if (interval) {
        this.setState({interval: interval})
      }
    } catch (err) {
      console.log(err)
    }
  }

  render() {
    return (
      <View>
        <Form style={{width: '96%'}}>
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
        <Button
          onPress={() => {
            if (this.state.ip.match(/^http:\/\/[a-zA-Z0-9.\-\/\_]+$/g)) {
              this.storeData('ip', this.state.ip)
              Keyboard.dismiss()

              const message = 'IP has been updated.'

              if (Platform.OS == 'ios') {
                ToastIOS.show(message, 0.75)
              } else {
                ToastAndroid.show(message, ToastAndroid.SHORT)
              }
            } else {
              const message = 'IP is invalid. Format needs to be: http://\"ip\"'

              if (Platform.OS == 'ios') {
                ToastIOS.show(message, 0.75)
              } else {
                ToastAndroid.show(message, ToastAndroid.SHORT)
              }
            }
          }}
          style={{margin: 10, width: '94%', backgroundColor: '#27ae60', justifyContent: 'center'}} iconRight>
          <Text>Save</Text>
        </Button>
        <Form style={{width: '96%'}}>
          <Item>
            <Icon name='clock'/>
            <Input
              placeholder="Update interval"
              value={this.state.interval.toString()}
              onChangeText={(text) => this.setState({interval: parseInt(text, 10)})}
              keyboardType='number-pad'
            />
          </Item>
        </Form>
        <Button
          onPress={() => {
            this.storeData('interval', this.state.interval)
            Keyboard.dismiss()

            const message = 'Send interval has been updated.'

            if (Platform.OS == 'ios') {
              ToastIOS.show(message, 0.75)
            } else {
              ToastAndroid.show(message, ToastAndroid.SHORT)
            }
          }}
          style={{margin: 10, width: '94%', backgroundColor: '#27ae60', justifyContent: 'center'}} iconRight>
          <Text>Save</Text>
        </Button>
      </View>
    )
  }
}

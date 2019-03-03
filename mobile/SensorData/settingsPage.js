import React, {Component} from 'react'
import { Platform, AsyncStorage, ToastAndroid } from 'react-native'
import { Form, Item, Input, Icon, View, Button, Text, } from 'native-base'
import { SensorService } from './native'

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

      const interval = await AsyncStorage.getItem('interval')

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
              ToastAndroid.show('IP has been updated.', ToastAndroid.SHORT)
            } else {
              ToastAndroid.show('IP is invalid. Format needs to be: http://\"ip\"', ToastAndroid.SHORT)
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
              value={`${this.state.interval / 1000}`}
              onChangeText={(text) => this.setState({interval: text})}
              keyboardType='number-pad'
            />
          </Item>
        </Form>
        <Button
          onPress={() => {
            this.storeData('interval', this.state.interval)
            ToastAndroid.show('Send interval has been updated.', ToastAndroid.SHORT)
          }}
          style={{margin: 10, width: '94%', backgroundColor: '#27ae60', justifyContent: 'center'}} iconRight>
          <Text>Save</Text>
        </Button>
      </View>
    )
  }
}

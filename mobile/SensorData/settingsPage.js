import React, {Component} from 'react'
import { Platform, AsyncStorage } from 'react-native'
import { Form, Item, Input, Icon, View, Button, Text } from 'native-base'

export default class SettingsPage extends Component {
  constructor(props) {
    super(props)

    this.state = {
      ip: null
    }

    this.storeData = async (ip) => {
      try {
        await AsyncStorage.setItem('ip', ip)
      } catch (err) {
        console.log(err)
      }
    }
  }

  async componentDidMount() {
    try {
      const ip = await AsyncStorage.getItem('ip')
      if (ip) {
        this.setState({ip: ip})
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
          onPress={() => this.storeData(this.state.ip)}
          style={{margin: 10, width: '94%', backgroundColor: '#27ae60', justifyContent: 'center'}} iconRight>
          <Text>Save</Text>
        </Button>
      </View>
    )
  }
}

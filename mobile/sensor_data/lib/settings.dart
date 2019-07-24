import 'package:flutter/material.dart';

class Settings extends StatelessWidget {
  const Settings({
    Key key
   }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Column(
      children: <Widget>[
        Column(
          children: <Widget>[
            Container(
              margin: EdgeInsets.only(left: 20, right: 20, top: 15),
              child: TextField(
                keyboardType: TextInputType.number,
                decoration: InputDecoration(
                  hintText: 'update interval',
                  icon: Icon(Icons.timer)
                ),
                style: new TextStyle(
                  fontSize: 18
                )
              )
            ),
            Container(
              margin: EdgeInsets.only(left: 20, right: 20, top: 20),
              child: TextField(
                keyboardType: TextInputType.url,
                decoration: InputDecoration(
                  hintText: 'url',
                  icon: Icon(Icons.dns)
                ),
                style: TextStyle(
                  fontSize: 18
                )
              )
            ),
            Container(
              margin: EdgeInsets.only(top: 30),
              child: FractionallySizedBox(
                widthFactor: 0.9,
                child: ButtonTheme(
                  height: 45,
                  child: FlatButton(
                    child: Text(
                      'SAVE',
                      style: TextStyle(
                        fontSize: 16,
                        color: Colors.white
                      ),
                    ),
                    color: Colors.green,
                    onPressed: () {

                    },
                  ),
                )
              )
            )
          ],
        )
      ],
    );
  }
}

import 'package:flutter/material.dart';

Widget buildList(BuildContext context, Map<String, dynamic> propertyMap, String title) {
  return Column(
    children: <Widget>[
      Container(
        child: ListTile(
          title: Text(
            title,
            style: Theme.of(context).textTheme.headline
          ),
        ),
      ),
      Container(
        color: Colors.white,
        child: propertyMap.isNotEmpty ? ListView(
          shrinkWrap: true,
          physics: ClampingScrollPhysics(),
          children: propertyMap.keys.map((String property) {
            return Column(
              children: <Widget>[
                Container(
                  padding: const EdgeInsets.all(12),
                  child: Row(
                    mainAxisSize: MainAxisSize.max,
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: <Widget>[
                      Container(
                        child: Text(
                          property,
                          style: TextStyle(
                            fontWeight: FontWeight.bold,
                            fontSize: 16
                          ),
                        ),
                      ),
                      Flexible(
                        child: Container(
                          child: Text(
                            '${propertyMap[property]}',
                            overflow: TextOverflow.ellipsis,
                            style: TextStyle(
                              fontSize: 16
                            ),
                          ),
                        ),
                      ),
                    ],
                  ),
                ),
                FractionallySizedBox(
                  widthFactor: 0.95,
                  child: Container(color: Color.fromARGB(255, 220, 220, 220), height: 0.6),
                )
              ],
            );
          }).toList(),
        ) : LinearProgressIndicator(backgroundColor: Color.fromARGB(255, 210, 210, 210))
      )
    ],
  );
}

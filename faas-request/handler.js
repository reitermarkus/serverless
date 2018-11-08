"use strict"
const mongoClient = require('mongodb').MongoClient

const MONGO_HOST = process.env.MONGO_HOST || 'localhost'
const MONGO_DB = process.env.MONGO_INITDB_DATABASE || 'sensor'
const MONGO_USER = process.env.MONGO_INITDB_ROOT_USERNAME || 'admin'
const MONGO_PASSWORD = process.env.MONGO_INITDB_ROOT_PASSWORD || 'root'

var sensorDB;  // Cached connection-pool for further requests.

const prepareDB = () => {
  const url = `mongodb://${MONGO_USER}:${MONGO_PASSWORD}@${MONGO_HOST}:27017/${MONGO_DB}?authSource=admin`

  return new Promise((resolve, reject) => {
    if(sensorDB) {
      console.error("DB already connected.")
      return resolve(sensorDB)
    }

    console.error("DB connecting");

    mongoClient.connect(url, (err, database) => {
      if(err) {
        return reject(err)
      }

      sensorDB = database.db(MONGO_DB)
      return resolve(sensorDB)
    })
  })
}

module.exports = (context, callback) => {
  prepareDB().then((sensor) => {
    sensor.collection("request").insertOne({ name: "req", value: context }, (insertErr) => {
      if(insertErr) {
        console.error(insertErr.toString())
      }
    }).catch(err => {
      console.error(err.toString())
    })

    callback(undefined, `saved ${context} in database.`)
  })
}
